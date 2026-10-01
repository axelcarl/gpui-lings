//! Run checks without trapping the guide in Cargo after the preview closes.
use std::{
    io,
    process::{Command, Output, Stdio},
    sync::mpsc,
    thread,
    time::Duration,
};

pub trait InterruptibleCommand {
    fn interruptible_output(&mut self, cancelled: &mut dyn FnMut() -> bool) -> io::Result<Output>;
}

impl InterruptibleCommand for Command {
    fn interruptible_output(&mut self, cancelled: &mut dyn FnMut() -> bool) -> io::Result<Output> {
        if cancelled() {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "Playground closed",
            ));
        }
        // Cargo's compiler and test children must stop too, including processes
        // that still hold its output pipes after Cargo itself exits.
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            self.process_group(0);
        }
        let child = self
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let pid = child.id();
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let _ = tx.send(child.wait_with_output());
        });
        loop {
            match rx.recv_timeout(Duration::from_millis(50)) {
                Ok(output) => return output,
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    return Err(io::Error::other("Check process monitor disconnected"));
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {}
            }
            if cancelled() {
                #[cfg(unix)]
                let stopped = Command::new("kill")
                    .args(["-KILL", &format!("-{pid}")])
                    .status()?;
                #[cfg(windows)]
                let stopped = Command::new("taskkill")
                    .args(["/PID", &pid.to_string(), "/T", "/F"])
                    .status()?;
                if !stopped.success() {
                    return Err(io::Error::other("Could not stop the check process"));
                }
                // Drain/reap the terminated process before returning to the session.
                let _ = rx.recv();
                return Err(io::Error::new(
                    io::ErrorKind::Interrupted,
                    "Playground closed",
                ));
            }
        }
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn cancellation_stops_a_command_and_its_children() {
        let started = Instant::now();
        let error = Command::new("sh")
            .args(["-c", "sleep 30 & wait"])
            .interruptible_output(&mut || started.elapsed() >= Duration::from_millis(100))
            .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::Interrupted);
        assert!(started.elapsed() < Duration::from_secs(5));
    }

    #[test]
    fn completed_commands_keep_their_output_and_status() {
        let output = Command::new("sh")
            .args(["-c", "echo check; echo diagnostic >&2; exit 7"])
            .interruptible_output(&mut || false)
            .unwrap();
        assert_eq!(output.status.code(), Some(7));
        assert_eq!(output.stdout, b"check\n");
        assert_eq!(output.stderr, b"diagnostic\n");
    }
}
