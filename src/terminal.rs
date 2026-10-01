use crate::{CheckState, LESSONS, Lesson, Report};
use std::{
    env,
    io::{self, IsTerminal, Write},
};

pub const SESSION_HELP: &str = "h hint     g guide     l exercises
n next     p previous  r rerun
d details  q quit
Type a command, then Enter.";

pub struct Terminal {
    color: bool,
    interactive: bool,
    width: usize,
}

impl Terminal {
    pub fn new() -> Self {
        let tty = io::stdout().is_terminal() && env::var("TERM").as_deref() != Ok("dumb");
        Self {
            color: tty && env::var_os("NO_COLOR").is_none(),
            interactive: tty && io::stdin().is_terminal(),
            width: env::var("COLUMNS")
                .ok()
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(80)
                .clamp(32, 96)
                - 4,
        }
    }

    fn ink(&self, value: &str, code: &str) -> String {
        if self.color {
            format!("\x1b[{code}m{value}\x1b[0m")
        } else {
            value.to_owned()
        }
    }
    fn line(&self, value: &str) {
        for line in wrap(value, self.width) {
            println!("  {line}");
        }
    }
    pub fn dashboard(
        &self,
        index: usize,
        report: Option<&Report>,
        message: &str,
        panel: Option<&str>,
    ) {
        if self.interactive {
            print!("\x1b[2J\x1b[H");
        }
        println!();
        if let Some(lesson) = LESSONS.get(index) {
            let heading = format!(
                "Exercise {} / {:02}  {}",
                lesson.id,
                LESSONS.len(),
                lesson.title
            );
            for line in wrap(&heading, self.width) {
                println!("  {}", self.ink(&line, "1"));
            }
            for line in wrap(lesson.file, self.width) {
                println!("  {}", self.ink(&line, "38;5;245"));
            }
            println!();
            self.line(lesson.objective);
            println!();
        } else {
            println!("  {}", self.ink("All exercises complete", "1;38;5;158"));
            println!();
        }

        // A build/check message replaces the old result while work is in progress.
        if !message.is_empty() {
            self.line(message);
        } else if index == LESSONS.len() {
            self.line("p to revisit an exercise");
        } else if let Some(report) = report {
            let (status, color) = match report.state {
                CheckState::Passed => ("✓ Passed · n to continue".to_owned(), "38;5;158"),
                CheckState::Failed => ("○ Not passing yet · edit and save".to_owned(), "38;5;222"),
                CheckState::BuildError => {
                    (format!("! {} · d for details", report.summary), "38;5;210")
                }
            };
            for line in wrap(&status, self.width) {
                println!("  {}", self.ink(&line, color));
            }
        }
        if let Some(panel) = panel {
            println!();
            for line in panel.lines() {
                self.line(line);
            }
        }
        println!();
        self.line("h hint  n next  ? help  q quit  [Enter]");
        print!("  {} ", self.ink("›", "38;5;158"));
        let _ = io::stdout().flush();
    }

    pub fn check_result(&self, lesson: &Lesson, report: &Report) {
        let (mark, color) = match report.state {
            CheckState::Passed => ("PASS", "38;5;158"),
            CheckState::Failed => ("TODO", "38;5;222"),
            CheckState::BuildError => ("ERROR", "38;5;210"),
        };
        println!(
            "\n  {}  {} / {}",
            self.ink(mark, color),
            lesson.id,
            lesson.title
        );
        if report.state == CheckState::BuildError {
            self.line(&report.summary);
        }
        self.line(&format!("Edit {}", lesson.file));
        if report.state != CheckState::Passed {
            println!("\n{}", report.details.trim());
        }
    }
}

pub fn lesson_list(index: usize) -> String {
    let mut lines = vec!["Exercises".to_string()];
    let mut chapter = "";
    for (i, lesson) in LESSONS.iter().enumerate() {
        if chapter != lesson.chapter {
            chapter = lesson.chapter;
            lines.push(format!("\n{chapter}"));
        }
        let mark = if i < index {
            "✓"
        } else if i == index {
            "›"
        } else {
            "·"
        };
        lines.push(format!(
            "{mark} {}  {}  /  {}",
            lesson.id, lesson.title, lesson.duration
        ));
    }
    lines.join("\n")
}

fn wrap(value: &str, width: usize) -> Vec<String> {
    if value.chars().count() <= width {
        return vec![value.to_owned()];
    }
    let mut result = Vec::new();
    let mut line = String::new();
    for word in value.split_whitespace() {
        if !line.is_empty() && line.chars().count() + 1 + word.chars().count() > width {
            result.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        for character in word.chars() {
            if line.chars().count() == width {
                result.push(std::mem::take(&mut line));
            }
            line.push(character);
        }
    }
    if !line.is_empty() {
        result.push(line);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn narrow_terminals_wrap_unicode_and_long_paths() {
        let text = "✓ playground/src/exercises/basics/greeting.rs";
        let lines = wrap(text, 12);
        assert!(lines.iter().all(|line| line.chars().count() <= 12));
        assert_eq!(lines.join("").replace(' ', ""), text.replace(' ', ""));
    }
}
