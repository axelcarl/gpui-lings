use crate::{CheckState, LESSONS, Lesson, Report, input};
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
/// colored output separately for display.
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

pub struct Terminal {
    color: bool,
    interactive: bool,
    /// Rows and columns for output that isn't a live terminal.
    fallback: (usize, usize),
}

impl Terminal {
    pub fn new() -> Self {
        let tty = io::stdout().is_terminal() && env::var("TERM").as_deref() != Ok("dumb");
        let number = |name| env::var(name).ok().and_then(|s| s.parse::<usize>().ok());
        Self {
            color: supports_color(),
            interactive: tty && io::stdin().is_terminal(),
            fallback: (
                number("LINES").unwrap_or(24),
                number("COLUMNS").unwrap_or(80),
            ),
        }
    }

    /// Rows and columns, measured on every call so screens follow resizes.
    pub fn size(&self) -> (usize, usize) {
        let (rows, columns) = self
            .interactive
            .then(input::terminal_size)
            .flatten()
            .unwrap_or(self.fallback);
        (rows.max(4), columns.max(20))
    }

    pub fn width(&self) -> usize {
        self.size().1
    }

    pub fn ink(&self, value: &str, code: &str) -> String {
        if self.color {
            format!("\x1b[{code}m{value}\x1b[0m")
        } else {
            value.to_owned()
        }
    }

    /// Replace the screen, aligning it to the bottom so the prompt stays on the
    /// last row. Taller screens scroll as usual, keeping the prompt visible.
    pub fn show(&self, screen: &str) {
        let mut output = String::new();
        if self.interactive {
            // Clear the screen and its scrollback, as Rustlings does.
            output.push_str("\x1b[H\x1b[2J\x1b[3J");
            let (rows, columns) = self.size();
            let used = screen_rows(screen, columns);
            output.push_str(&"\n".repeat(rows.saturating_sub(used)));
        } else {
            output.push('\n');
        }
        output.push_str(screen);
        print!("{output}");
        let _ = io::stdout().flush();
    }

    /// Rustlings' progress bar: `Progress: [####>-----]  12/38`.
    pub fn progress(&self, completed: usize) -> String {
        let total = LESSONS.len();
        let completed = completed.min(total);
        let suffix = format!("] {completed:>3}/{total}");
        let width = self
            .width()
            .saturating_sub("Progress: [".len() + suffix.len());
        if width < 4 {
            return format!("Progress: {completed}/{total}");
        }
        let filled = width * completed / total;
        let mut bar = self.ink(&"#".repeat(filled), "32");
        if filled < width {
            bar.push('>');
            bar.push_str(&self.ink(&"-".repeat(width - filled - 1), "31"));
        }
        format!("Progress: [{bar}{suffix}")
    }

    pub fn report_diagnostic(&self, report: &Report) -> String {
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
        let (heading, notes) = failure.explain();
        let mut output = format!("{}: check failed: {heading}\n", self.ink("error", "1;31"));
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
        let excerpt = source
            .as_deref()
            .map(|source| statement(source, failure.line, failure.column))
            .unwrap_or_default();
        let gutter = (failure.line + excerpt.len().saturating_sub(1))
            .to_string()
            .len();
        let pipe = self.ink("|", "1;34");
        if !excerpt.is_empty() {
            writeln!(output, "{:gutter$} {pipe}", "").unwrap();
            for (offset, line) in excerpt.iter().enumerate() {
                let number = format!("{:>gutter$}", failure.line + offset);
                writeln!(output, "{} {pipe} {line}", self.ink(&number, "1;34")).unwrap();
            }
            if let [line] = excerpt.as_slice() {
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
            }
            writeln!(output, "{:gutter$} {pipe}", "").unwrap();
        }
        for note in notes {
            writeln!(output, "{:gutter$} {} {note}", "", self.ink("=", "1;34")).unwrap();
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

/// Terminal rows a screen occupies once long lines wrap.
fn screen_rows(screen: &str, columns: usize) -> usize {
    screen
        .split('\n')
        .map(|line| plain_text(line).chars().count().div_ceil(columns).max(1))
        .sum()
}

/// The source lines of the statement starting at `line`, such as a multi-line
/// `assert_eq!(...)`, so the check's arguments and message stay visible.
fn statement(source: &str, line: usize, column: usize) -> Vec<&str> {
    let lines: Vec<_> = source
        .lines()
        .skip(line.saturating_sub(1))
        .take(12)
        .collect();
    let mut depth = 0usize;
    let mut opened = false;
    let mut in_string = false;
    for (index, text) in lines.iter().enumerate() {
        let start = if index == 0 {
            column.saturating_sub(1)
        } else {
            0
        };
        let mut escaped = false;
        for c in text.chars().skip(start) {
            match c {
                _ if escaped => escaped = false,
                '\\' if in_string => escaped = true,
                '"' => in_string = !in_string,
                '(' | '[' | '{' if !in_string => {
                    depth += 1;
                    opened = true;
                }
                ')' | ']' | '}' if !in_string => depth = depth.saturating_sub(1),
                _ => {}
            }
        }
        if !opened || depth == 0 {
            return lines[..=index].to_vec();
        }
    }
    // An unbalanced or very long statement: point at its first line only.
    lines.into_iter().take(1).collect()
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

    /// Lead with the check's own explanation; keep compared values as notes.
    fn explain(&self) -> (String, Vec<String>) {
        let mut lines = self.message.lines();
        let first = lines.next().unwrap_or_default();
        let notes = lines
            .map(|line| {
                let line = line.trim();
                match line.split_once(": ") {
                    Some(("left", value)) => format!("left:  {value}"),
                    Some(("right", value)) => format!("right: {value}"),
                    _ => line.to_owned(),
                }
            })
            .collect();
        let heading = match first.split_once(" failed") {
            Some((assertion, rest)) if assertion.starts_with("assertion `left") => {
                match rest.strip_prefix(": ") {
                    Some(reason) => reason.to_owned(),
                    None => "left and right differ".to_owned(),
                }
            }
            _ => first.to_owned(),
        };
        (heading, notes)
    }
}

pub fn wrap(value: &str, width: usize) -> Vec<String> {
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
impl Terminal {
    /// Uncolored output of a fixed size, for tests.
    pub fn new_sized(rows: usize, columns: usize) -> Self {
        Self {
            color: false,
            interactive: false,
            fallback: (rows, columns),
        }
    }
}

#[cfg(test)]
pub fn plain_terminal(columns: usize) -> Terminal {
    Terminal::new_sized(24, columns)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assertion_report(message: &str, line: usize) -> Report {
        Report {
            state: CheckState::Failed,
            summary: "Not passing yet".into(),
            details: format!(
                "running 1 test\ntest tests::exercise_01 ... \x1b[31mFAILED\x1b[0m\n\nfailures:\n\n---- tests::exercise_01 stdout ----\ncustom diagnostic from test\n\nthread 'tests::exercise_01' (123) panicked at {}/{}:{line}:9:\n{message}\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n\nfailures:\n    tests::exercise_01\n\ntest result: FAILED. 0 passed; 1 failed;\n",
                crate::root().display(),
                LESSONS[0].file,
            ),
        }
    }

    fn assertion_line(lesson: &Lesson, needle: &str) -> usize {
        let source = fs::read_to_string(crate::root().join(lesson.file)).unwrap();
        source
            .lines()
            .position(|line| line.contains(needle))
            .unwrap()
            + 1
    }

    #[test]
    fn test_diagnostic_shows_source_and_values_without_harness_noise() {
        let line = assertion_line(&LESSONS[0], "assert_eq!");
        let report = assertion_report(
            "assertion `left == right` failed\n  left: \"Hello, Rust!\"\n right: \"Hello, GPUI!\"",
            line,
        );
        let text = plain_terminal(80).report_diagnostic(&report);
        assert!(text.starts_with(
            "error: check failed: left and right differ\n  --> exercises/01_basics/basics1.rs:"
        ));
        assert!(text.contains("assert_eq!(welcome_text(), \"Hello, GPUI!\");"));
        assert!(text.contains("^ check failed here"));
        assert!(text.contains("= left:  \"Hello, Rust!\""));
        assert!(text.contains("= right: \"Hello, GPUI!\""));
        assert!(text.contains("Output:\ncustom diagnostic from test"));
        assert!(!text.contains("running 1 test"));
        assert!(!text.contains("test result:"));
        assert!(!text.contains("RUST_BACKTRACE"));
        assert!(!text.contains('\x1b'));
    }

    #[test]
    fn multi_line_assertions_show_their_arguments_and_lead_with_the_message() {
        let source = "    fn check() {\n        assert_eq!(\n            status,\n            \"Available (\",\n            \"upgrade succeeds\"\n        );\n        other();\n";
        assert_eq!(
            statement(source, 2, 9),
            [
                "        assert_eq!(",
                "            status,",
                "            \"Available (\",",
                "            \"upgrade succeeds\"",
                "        );"
            ]
        );
        assert_eq!(statement(source, 7, 9), ["        other();"]);
        assert_eq!(statement("panic!(\"open", 1, 1), ["panic!(\"open"]);
        let failure = TestFailure {
            file: "",
            line: 1,
            column: 1,
            message: "assertion `left == right` failed: upgrade succeeds\n  left: \"Released\"\n right: \"Available\"",
            output: "",
        };
        assert_eq!(
            failure.explain(),
            (
                "upgrade succeeds".into(),
                vec!["left:  \"Released\"".into(), "right: \"Available\"".into()]
            )
        );
        let failure = TestFailure {
            message: "the shortcut must remain scoped",
            ..failure
        };
        assert_eq!(failure.explain().0, "the shortcut must remain scoped");
    }

    #[test]
    fn compiler_diagnostics_and_unfamiliar_failures_remain_intact() {
        let diagnostic = "\x1b[1;31merror[E0425]\x1b[0m: missing value\n  --> src/exercises/views/../../../../exercises/02_views/views1.rs:4:5\n  |\n4 | bad()\n  | ^^^\nhelp: use another value";
        let rendered = plain_terminal(80).diagnostic(CheckState::BuildError, diagnostic);
        assert_eq!(
            rendered,
            diagnostic.replace("src/exercises/views/../../../../", "")
        );
        let unknown = "test failed with an unfamiliar panic format\nkeep this detail";
        assert_eq!(
            plain_terminal(80).diagnostic(CheckState::Failed, unknown),
            unknown
        );
        let mut report = assertion_report("assertion failed: ok", 1);
        report
            .details
            .push_str("\nthread 'worker' panicked at another.rs:1:1:\nsecond panic");
        assert!(
            plain_terminal(80)
                .report_diagnostic(&report)
                .contains("second panic")
        );
    }

    #[test]
    fn progress_matches_rustlings_and_fits_a_small_terminal() {
        let progress = plain_terminal(32).progress(6);
        assert!(progress.ends_with("]   6/38"), "{progress}");
        assert_eq!(progress.chars().count(), 32);
        assert!(plain_terminal(32).progress(usize::MAX).ends_with("38/38"));
        assert_eq!(plain_terminal(20).progress(6), "Progress: 6/38");
    }

    #[test]
    fn bottom_alignment_counts_wrapped_rows() {
        assert_eq!(screen_rows("one\ntwo\n? ", 80), 3);
        assert_eq!(screen_rows(&"x".repeat(81), 80), 2);
        assert_eq!(screen_rows("\x1b[1;31merror\x1b[0m", 5), 1);
    }

    #[test]
    fn narrow_terminals_wrap_unicode_and_long_paths() {
        let text = "✓ exercises/01_basics/basics1.rs";
        let lines = wrap(text, 12);
        assert!(lines.iter().all(|line| line.chars().count() <= 12));
        assert_eq!(lines.join("").replace(' ', ""), text.replace(' ', ""));
    }
}
