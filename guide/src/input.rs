//! Single-key input, as in Rustlings' watch mode: no Enter needed.
//!
//! The guide has no dependencies, so it switches the terminal with `stty`
//! instead of a terminal library. Without a terminal (pipes, tests, Windows),
//! input stays line-buffered and each line's characters are read as keys.
use std::{
    cell::Cell,
    io::{self, IsTerminal, Read},
    process::{Command, Stdio},
    sync::mpsc::{self, Receiver, RecvTimeoutError},
    thread,
    time::Duration,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Up,
    Down,
    Home,
    End,
    Enter,
    Esc,
    Backspace,
    /// Ctrl-C arrives as a key because raw mode disables terminal signals.
    Interrupt,
}

impl Key {
    pub fn quits(self) -> bool {
        matches!(self, Key::Char('q') | Key::Interrupt)
    }
}

/// Decode one read from the terminal. Escape sequences arrive within a read.
pub fn parse_keys(bytes: &[u8]) -> Vec<Key> {
    let text = String::from_utf8_lossy(bytes);
    let mut chars = text.chars().peekable();
    let mut keys = Vec::new();
    while let Some(c) = chars.next() {
        keys.push(match c {
            '\x1b' if matches!(chars.peek(), Some('[' | 'O')) => {
                chars.next();
                let mut sequence = String::new();
                for c in chars.by_ref() {
                    sequence.push(c);
                    if ('@'..='~').contains(&c) {
                        break;
                    }
                }
                match sequence.as_str() {
                    "A" => Key::Up,
                    "B" => Key::Down,
                    "H" | "1~" | "7~" => Key::Home,
                    "F" | "4~" | "8~" => Key::End,
                    _ => continue,
                }
            }
            '\x1b' => Key::Esc,
            '\x03' => Key::Interrupt,
            '\r' | '\n' => Key::Enter,
            '\x7f' | '\x08' => Key::Backspace,
            c if c.is_control() => continue,
            c => Key::Char(c),
        });
    }
    keys
}

/// Restores the terminal's previous settings when dropped, including on panic.
struct RawMode {
    saved: String,
}

impl RawMode {
    fn enable() -> Option<Self> {
        if !cfg!(unix) || !io::stdin().is_terminal() {
            return None;
        }
        let saved = stty(&["-g"])?;
        // Keep output processing, so "\n" still starts a new line.
        stty(&["-icanon", "-echo", "-isig", "min", "1", "time", "0"])?;
        Some(Self {
            saved: saved.trim().to_owned(),
        })
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        let _ = stty(&[&self.saved]);
    }
}

fn stty(args: &[&str]) -> Option<String> {
    let output = Command::new("stty")
        .args(args)
        .stdin(Stdio::inherit())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Rows and columns of the controlling terminal, when there is one.
pub fn terminal_size() -> Option<(usize, usize)> {
    if !cfg!(unix) || !io::stdin().is_terminal() {
        return None;
    }
    let size = stty(&["size"])?;
    let mut values = size.split_whitespace().map(str::parse::<usize>);
    match (values.next()?.ok()?, values.next()?.ok()?) {
        (0, _) | (_, 0) => None,
        size => Some(size),
    }
}

pub struct Input {
    keys: Receiver<Key>,
    quit: Cell<bool>,
    _raw: Option<RawMode>,
}

impl Input {
    pub fn start() -> Self {
        let raw = RawMode::enable();
        let (tx, keys) = mpsc::channel();
        thread::spawn(move || {
            let mut buffer = [0; 64];
            loop {
                match io::stdin().lock().read(&mut buffer) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        for key in parse_keys(&buffer[..n]) {
                            if tx.send(key).is_err() {
                                return;
                            }
                        }
                    }
                }
            }
        });
        Self {
            keys,
            quit: Cell::new(false),
            _raw: raw,
        }
    }

    /// Input that never produces a key, for tests.
    #[cfg(test)]
    pub fn detached() -> Self {
        Self {
            keys: mpsc::channel().1,
            quit: Cell::new(false),
            _raw: None,
        }
    }

    /// The next key, or `Disconnected` once standard input has closed.
    pub fn next(&self, timeout: Duration) -> Result<Key, RecvTimeoutError> {
        if self.quit.get() {
            return Err(RecvTimeoutError::Disconnected);
        }
        self.keys.recv_timeout(timeout)
    }

    /// Like Rustlings, ignore keys pressed while a check runs, except quitting.
    /// Long checks poll this so `q` and Ctrl-C can still stop them.
    pub fn quit_requested(&self) -> bool {
        loop {
            match self.keys.try_recv() {
                Ok(key) if key.quits() => self.quit.set(true),
                Ok(_) => {}
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.quit.set(true);
                    break;
                }
            }
        }
        self.quit.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_arrive_without_enter_and_sequences_decode() {
        assert_eq!(
            parse_keys(b"nh\x1b[A\x1b[B\x1bOH\x1b[4~"),
            [
                Key::Char('n'),
                Key::Char('h'),
                Key::Up,
                Key::Down,
                Key::Home,
                Key::End
            ]
        );
        assert_eq!(
            parse_keys(b"\x03\x1b\r\x7f"),
            [Key::Interrupt, Key::Esc, Key::Enter, Key::Backspace]
        );
        // Unknown sequences and other control bytes are ignored.
        assert_eq!(parse_keys(b"\x1b[200~x\x01"), [Key::Char('x')]);
        // Line-buffered input still works one character at a time.
        assert_eq!(parse_keys(b"c\n"), [Key::Char('c'), Key::Enter]);
        assert!(Key::Interrupt.quits() && Key::Char('q').quits());
        assert!(!Key::Char('x').quits());
    }
}
