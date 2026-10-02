use crate::{CheckState, LESSONS, Lesson, Report, state::LessonState};
use std::{
    borrow::Cow,
    env,
    fmt::Write as _,
    fs,
    io::{self, IsTerminal, Write},
};

fn supports_color() -> bool {
    io::stdout().is_terminal()
        && env::var("TERM").as_deref() != Ok("dumb")
        && env::var_os("NO_COLOR").is_none()
}

/// Checks are captured through pipes, so `auto` would discard their colors.
/// Match the destination terminal instead, just as for the guide's own UI.
pub fn color_argument() -> &'static str {
    if supports_color() {
        "--color=always"
    } else {
        "--color=never"
    }
}

/// Parse compiler/libtest output without ANSI styling, retaining the original
/// colored output separately for display and replay through `d`.
pub fn plain_text(text: &str) -> Cow<'_, str> {
    if !text.contains('\x1b') {
        return Cow::Borrowed(text);
    }
    let mut plain = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(character) = chars.next() {
        if character == '\x1b' && matches!(chars.peek(), Some('(' | ')')) {
            // terminfo's xterm reset is ESC(B ESC[m: reset the character
            // set as well as the style. Libtest emits this after `ok`/`FAILED`.
            chars.next();
            chars.next();
        } else if character == '\x1b' && chars.peek() == Some(&'[') {
            chars.next();
            for code in chars.by_ref() {
                if ('@'..='~').contains(&code) {
                    break;
                }
            }
        } else {
            plain.push(character);
        }
    }
    Cow::Owned(plain)
}

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
            color: supports_color(),
            interactive: tty && io::stdin().is_terminal(),
            width: env::var("COLUMNS")
                .ok()
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(80)
                .clamp(32, 120),
        }
    }

    fn ink(&self, value: &str, code: &str) -> String {
        if self.color {
            format!("\x1b[{code}m{value}\x1b[0m")
        } else {
            value.to_owned()
        }
    }
    pub fn dashboard(
        &self,
        index: usize,
        completed: usize,
        report: Option<&Report>,
        message: &str,
        panel: Option<&str>,
    ) {
        if self.interactive {
            print!("\x1b[2J\x1b[H");
        }
        print!("{}", self.screen(index, completed, report, message, panel));
        let _ = io::stdout().flush();
    }

    fn screen(
        &self,
        index: usize,
        completed: usize,
        report: Option<&Report>,
        message: &str,
        panel: Option<&str>,
    ) -> String {
        let mut screen = String::new();
        // Explicit panels (guide, hints, raw details) replace the diagnostic,
        // so requesting raw output never prints the same failure twice.
        if let Some(panel) = panel {
            writeln!(screen, "{}\n", short_paths(panel).trim()).unwrap();
        } else if failure_details(report, message).is_some() {
            let report = report.unwrap();
            writeln!(screen, "{}\n", self.report_diagnostic(report)).unwrap();
        }
        if !message.is_empty() {
            let tone = if is_loading(message) { "1;34" } else { "1;31" };
            writeln!(screen, "{}\n", self.ink(message, tone)).unwrap();
        } else if index == LESSONS.len() {
            writeln!(screen, "{}\n", self.ink("All exercises complete!", "1;32")).unwrap();
        } else if report.is_some_and(|r| r.state == CheckState::Passed) {
            writeln!(
                screen,
                "{}\n",
                self.ink("Check passed. Press n to continue.", "1;32")
            )
            .unwrap();
        }
        if is_loading(message) {
            writeln!(screen, "Progress: {}", self.ink("checking…", "34")).unwrap();
        } else {
            writeln!(screen, "{}", self.progress(completed)).unwrap();
        }
        if let Some(lesson) = LESSONS.get(index) {
            writeln!(
                screen,
                "Current exercise: {}",
                self.ink(lesson.file, "1;34")
            )
            .unwrap();
            for line in wrap(&format!("Goal: {}", lesson.objective), self.width) {
                writeln!(screen, "{line}").unwrap();
            }
        }
        screen.push('\n');
        for line in wrap(
            "h:hint / n:next / l:list / d:details / q:quit / ?:help [Enter]",
            self.width,
        ) {
            writeln!(screen, "{line}").unwrap();
        }
        write!(screen, "{} ", self.ink("?", "1;36")).unwrap();
        screen
    }

    fn progress(&self, completed: usize) -> String {
        let completed = completed.min(LESSONS.len());
        let count = format!("{completed}/{}", LESSONS.len());
        let width = self.width.saturating_sub(14 + count.len()).max(4);
        let filled = width * completed / LESSONS.len();
        let mut bar = "#".repeat(filled);
        if filled < width {
            bar.push('>');
            bar.push_str(&"-".repeat(width - filled - 1));
        }
        format!("Progress: [{}] {count}", self.ink(&bar, "32"))
    }

    fn report_diagnostic(&self, report: &Report) -> String {
        let details = self.diagnostic(report.state, &report.details);
        if report.state == CheckState::BuildError
            && !plain_text(&details)
                .lines()
                .any(|line| line.trim_start().starts_with("error"))
        {
            let heading = format!("{}: {}", self.ink("error", "1;31"), report.summary);
            if details.is_empty() || details == report.summary {
                return heading;
            }
            return format!("{heading}\n\n{details}");
        }
        details
    }

    fn diagnostic(&self, state: CheckState, details: &str) -> String {
        let shortened = short_paths(details);
        if state != CheckState::Failed {
            return shortened.trim().to_owned();
        }
        let plain = plain_text(&shortened);
        let Some(failure) = TestFailure::parse(&plain) else {
            // Unfamiliar panic/test formats must remain visible in full.
            return shortened.trim().to_owned();
        };
        let mut output = format!("{}: exercise check failed\n", self.ink("error", "1;31"));
        writeln!(
            output,
            "  {} {}:{}:{}",
            self.ink("-->", "1;34"),
            failure.file,
            failure.line,
            failure.column
        )
        .unwrap();
        // Only read a known lesson, never an arbitrary path printed by a test.
        let source = LESSONS
            .iter()
            .find(|l| l.file == failure.file)
            .and_then(|l| fs::read_to_string(crate::root().join(l.file)).ok());
        if let Some(line) = source
            .as_deref()
            .and_then(|s| s.lines().nth(failure.line - 1))
        {
            let gutter = failure.line.to_string().len();
            let pipe = self.ink("|", "1;34");
            writeln!(output, "{:gutter$} {pipe}", "").unwrap();
            writeln!(
                output,
                "{} {pipe} {line}",
                self.ink(&failure.line.to_string(), "1;34")
            )
            .unwrap();
            // Rust's column is one-based; bound it to the source line before padding.
            let column = failure.column.saturating_sub(1).min(line.chars().count());
            let padding: String = line
                .chars()
                .take(column)
                .map(|c| if c == '\t' { '\t' } else { ' ' })
                .collect();
            writeln!(
                output,
                "{:gutter$} {pipe} {padding}{}",
                "",
                self.ink("^ check failed here", "1;31")
            )
            .unwrap();
            writeln!(output, "{:gutter$} {pipe}", "").unwrap();
        }
        for line in failure.message.lines() {
            writeln!(output, "  {} {line}", self.ink("=", "1;34")).unwrap();
        }
        if !failure.output.is_empty() {
            writeln!(output, "\nOutput:\n{}", failure.output).unwrap();
        }
        output.trim_end().to_owned()
    }

    pub fn check_result(&self, lesson: &Lesson, report: &Report) {
        if report.state == CheckState::Passed {
            println!("{} {}", self.ink("Passed:", "1;32"), lesson.file);
        } else {
            println!("{}", self.report_diagnostic(report));
            println!("\nCurrent exercise: {}", self.ink(lesson.file, "1;34"));
            println!("Goal: {}\n", lesson.objective);
        }
    }
}

pub fn lesson_list(state: &LessonState) -> String {
    let mut lines = vec!["Exercises".to_string()];
    let mut chapter = "";
    for (i, lesson) in LESSONS.iter().enumerate() {
        if chapter != lesson.chapter {
            chapter = lesson.chapter;
            lines.push(format!("\n{chapter}"));
        }
        let current = if i == state.current { "›" } else { " " };
        let mark = if state.is_done(lesson.name) {
            "✓"
        } else {
            "·"
        };
        lines.push(format!(
            "{current} {mark} {}  {}  /  {}  /  {}",
            lesson.id, lesson.name, lesson.title, lesson.duration
        ));
    }
    lines.join("\n")
}

fn is_loading(message: &str) -> bool {
    matches!(
        message,
        "Checking…" | "Building playground…" | "Refreshing playground…"
    )
}

fn failure_details<'a>(report: Option<&'a Report>, message: &str) -> Option<&'a str> {
    if is_loading(message) {
        return None;
    }
    report
        .filter(|r| r.state != CheckState::Passed)
        .map(|r| r.details.as_str())
}

/// Shorten only known workspace paths; keep rustc's colors, gutters and hints.
fn short_paths(text: &str) -> String {
    let mut text = text.replace(&format!("{}/", crate::root().display()), "");
    for lesson in &LESSONS {
        if let Some(group) = lesson
            .test
            .strip_prefix("exercises::")
            .and_then(|t| t.split("::").next())
        {
            for prefix in ["playground/", ""] {
                text = text.replace(
                    &format!("{prefix}src/exercises/{group}/../../../../{}", lesson.file),
                    lesson.file,
                );
            }
        }
    }
    text
}

struct TestFailure<'a> {
    file: &'a str,
    line: usize,
    column: usize,
    message: &'a str,
    output: &'a str,
}

impl<'a> TestFailure<'a> {
    fn parse(text: &'a str) -> Option<Self> {
        let panic = text
            .lines()
            .find(|line| line.starts_with("thread '") && line.contains(" panicked at "))?;
        // More than one panic needs the full output to avoid hiding a failure.
        if text
            .lines()
            .filter(|line| line.starts_with("thread '") && line.contains(" panicked at "))
            .count()
            != 1
        {
            return None;
        }
        let (_, location) = panic.split_once(" panicked at ")?;
        let (path_line, column) = location.strip_suffix(':')?.rsplit_once(':')?;
        let (file, line) = path_line.rsplit_once(':')?;
        let line = line.parse::<usize>().ok().filter(|&n| n > 0)?;
        let column = column.parse::<usize>().ok().filter(|&n| n > 0)?;
        let (before, after) = text.split_once(panic)?;
        let message = after
            .trim_start_matches('\n')
            .split("\nfailures:\n")
            .next()?;
        let message = message
            .split("\nnote: run with `RUST_BACKTRACE=1`")
            .next()?
            .trim();
        if message.is_empty() {
            return None;
        }
        // Preserve anything the test printed before panicking, excluding the harness.
        let output = before
            .split_once(" stdout ----\n")
            .map_or("", |(_, output)| output.trim());
        Some(Self {
            file,
            line,
            column,
            message,
            output,
        })
    }
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

    fn terminal(color: bool) -> Terminal {
        Terminal {
            color,
            interactive: false,
            width: 80,
        }
    }

    fn assertion_report() -> Report {
        let source = fs::read_to_string(crate::root().join(LESSONS[0].file)).unwrap();
        let line = source
            .lines()
            .position(|line| line.contains("assert_eq!"))
            .unwrap()
            + 1;
        Report {
            state: CheckState::Failed,
            summary: "Not passing yet".into(),
            full_details: None,
            details: format!(
                "running 1 test\ntest tests::exercise_01 ... \x1b[31mFAILED\x1b[0m\n\nfailures:\n\n---- tests::exercise_01 stdout ----\ncustom diagnostic from test\n\nthread 'tests::exercise_01' (123) panicked at {}/{}:{line}:9:\nassertion `left == right` failed\n  left: \"Hello, Rust!\"\n right: \"Hello, GPUI!\"\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n\nfailures:\n    tests::exercise_01\n\ntest result: FAILED. 0 passed; 1 failed;\n",
                crate::root().display(),
                LESSONS[0].file,
            ),
        }
    }

    #[test]
    fn test_diagnostic_shows_source_and_values_without_harness_noise() {
        let report = assertion_report();
        let screen = terminal(false).screen(0, 0, Some(&report), "", None);
        assert!(
            screen
                .starts_with("error: exercise check failed\n  --> exercises/01_basics/basics1.rs:")
        );
        assert!(screen.contains("assert_eq!(welcome_text(), \"Hello, GPUI!\");"));
        assert!(screen.contains("^ check failed here"));
        assert!(screen.contains("left: \"Hello, Rust!\""));
        assert!(screen.contains("right: \"Hello, GPUI!\""));
        assert!(screen.contains("Output:\ncustom diagnostic from test"));
        assert!(!screen.contains("running 1 test"));
        assert!(!screen.contains("test result:"));
        assert!(!screen.contains("RUST_BACKTRACE"));
        assert!(!screen.contains('\x1b'));
        assert!(screen.find("error:").unwrap() < screen.find("Progress:").unwrap());
        assert!(screen.contains("0/38\nCurrent exercise: exercises/01_basics/basics1.rs"));
        assert!(
            report.details.contains("test result:"),
            "original output remains available for d"
        );
    }

    #[test]
    fn raw_details_replace_the_summary_and_pending_work_hides_results() {
        let report = assertion_report();
        let screen = terminal(true).screen(0, 1, Some(&report), "Building playground…", None);
        assert!(screen.contains("\x1b[1;34mBuilding playground…"));
        assert!(!screen.contains("exercise check failed"));
        assert!(
            !screen.contains("1/38"),
            "don't reveal progress before the refresh"
        );
        let panel = format!("CHECK DETAILS\n{}", report.details);
        let screen = terminal(true).screen(0, 0, Some(&report), "", Some(&panel));
        assert!(screen.contains("test result:"));
        assert_eq!(screen.matches("assertion `left == right`").count(), 1);
        assert!(screen.contains("\x1b[31mFAILED"));
    }

    #[test]
    fn compiler_diagnostics_and_unfamiliar_failures_remain_intact() {
        let diagnostic = "\x1b[1;31merror[E0425]\x1b[0m: missing value\n  --> src/exercises/views/../../../../exercises/02_views/views1.rs:4:5\n  |\n4 | bad()\n  | ^^^\nhelp: use another value";
        let rendered = terminal(true).diagnostic(CheckState::BuildError, diagnostic);
        assert_eq!(
            rendered,
            diagnostic.replace("src/exercises/views/../../../../", "")
        );
        let unknown = "test failed with an unfamiliar panic format\nkeep this detail";
        assert_eq!(
            terminal(false).diagnostic(CheckState::Failed, unknown),
            unknown
        );
        let mut report = assertion_report();
        report
            .details
            .push_str("\nthread 'worker' panicked at another.rs:1:1:\nsecond panic");
        assert!(
            terminal(true)
                .report_diagnostic(&report)
                .contains("second panic")
        );
    }

    #[test]
    fn progress_uses_completion_count_and_fits_a_small_terminal() {
        let terminal = Terminal {
            width: 32,
            ..terminal(false)
        };
        let progress = terminal.progress(6);
        assert!(progress.ends_with("6/38"));
        assert!(progress.len() <= 32);
        assert!(terminal.progress(usize::MAX).ends_with("38/38"));
    }

    #[test]
    fn failures_are_visible_automatically_and_hidden_while_rebuilding() {
        let report = Report::error(
            "error[E0308]: mismatched types\n  --> exercises/03_contexts/contexts2.rs:42:9\n   |\n42 | bad()\n   | ^^^^^ expected callback",
        );
        assert_eq!(
            failure_details(Some(&report), ""),
            Some(report.details.as_str())
        );
        assert_eq!(
            failure_details(Some(&report), "Not passing yet · h for a hint"),
            Some(report.details.as_str())
        );
        assert_eq!(
            failure_details(Some(&report), "Refreshing playground…"),
            None
        );
    }

    #[test]
    fn list_uses_completion_records_not_cursor_position() {
        let mut state = LessonState::load(std::path::Path::new("/nonexistent-gpui-state"));
        state.current = 2;
        state.set_done(0, true);
        state.set_done(3, true);
        let list = lesson_list(&state);
        assert!(list.contains("  ✓ 01"));
        assert!(list.contains("  · 02"));
        assert!(list.contains("› · 03"));
        assert!(list.contains("  ✓ 04"));
    }

    #[test]
    fn narrow_terminals_wrap_unicode_and_long_paths() {
        let text = "✓ exercises/01_basics/basics1.rs";
        let lines = wrap(text, 12);
        assert!(lines.iter().all(|line| line.chars().count() <= 12));
        assert_eq!(lines.join("").replace(' ', ""), text.replace(' ', ""));
    }
}
