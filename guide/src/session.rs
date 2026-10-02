//! The interactive session: Rustlings' watch mode beside a native preview.
//!
//! As in Rustlings, saving checks the current exercise, `n` moves on once it
//! passes, `c` checks every exercise, `x` resets the current one and `l` opens
//! a list to jump between exercises. Each key acts immediately, without Enter.
use crate::{
    LESSONS, PlaygroundProcess, Report, check_all, check_lesson_with_cancel,
    input::{Input, Key},
    lessons::Lesson,
    preview::Refresh,
    reset_exercise, root, source_fingerprint,
    state::LessonState,
    terminal::{Terminal, wrap},
};
use std::{process::ExitCode, sync::mpsc::RecvTimeoutError, time::Duration};

pub fn run(start: Option<usize>) -> ExitCode {
    let mut session = Session::new(Input::start());
    if let Some(index) = start {
        session.state.current = index;
        session.save();
    }
    session.refresh();
    session.event_loop();
    session.finish()
}

enum View {
    Main,
    ConfirmReset,
    List(List),
}

/// A transient line above the progress bar.
struct Message {
    text: String,
    error: bool,
}

struct Session {
    ui: Terminal,
    input: Input,
    state: LessonState,
    app: PlaygroundProcess,
    report: Option<Report>,
    app_error: Option<String>,
    /// Hint levels revealed for the current exercise.
    hints: usize,
    /// The last "check all" result, shown until the next check.
    summary: Option<String>,
    message: Option<Message>,
    view: View,
    last_change: Option<u64>,
    pending_change: Option<u64>,
    size: (usize, usize),
}

impl Session {
    fn new(input: Input) -> Self {
        let ui = Terminal::new();
        let size = ui.size();
        Self {
            ui,
            input,
            state: LessonState::load(&root()),
            app: PlaygroundProcess::default(),
            report: None,
            app_error: None,
            hints: 0,
            summary: None,
            message: None,
            view: View::Main,
            last_change: None,
            pending_change: None,
            size,
        }
    }

    fn event_loop(&mut self) {
        let mut ticks = 0u32;
        loop {
            if self.app.has_exited() {
                break;
            }
            match self.input.next(Duration::from_millis(100)) {
                Ok(key) => {
                    if !self.handle(key) {
                        break;
                    }
                }
                Err(RecvTimeoutError::Disconnected) => break,
                Err(RecvTimeoutError::Timeout) => {}
            }
            if self.app.has_exited() {
                break;
            }
            // Like Rustlings' list, the list view pauses checks until it closes.
            if matches!(self.view, View::Main) {
                let change = source_fingerprint();
                // Wait for two matching observations so editor atomic saves settle first.
                if change != self.last_change && change == self.pending_change {
                    self.summary = None;
                    self.refresh();
                }
                self.pending_change = change;
            }
            ticks = ticks.wrapping_add(1);
            if ticks.is_multiple_of(5) && self.ui.size() != self.size {
                self.render();
            }
        }
    }

    fn finish(&mut self) -> ExitCode {
        let code = match self.app.exit_status() {
            Ok(Some(status)) if !status.success() => {
                eprintln!("\n  Playground exited unexpectedly ({status}).");
                ExitCode::FAILURE
            }
            Err(error) => {
                eprintln!("\n  Could not monitor the playground: {error}");
                ExitCode::FAILURE
            }
            _ => ExitCode::SUCCESS,
        };
        println!("\n\n  Session closed.\n");
        code
    }

    fn lesson(&self) -> Option<&'static Lesson> {
        LESSONS.get(self.state.current)
    }

    fn done(&self) -> bool {
        self.report.as_ref().is_some_and(Report::passed)
    }

    fn more_hints(&self) -> bool {
        self.lesson().is_some_and(|l| self.hints < l.hints.len())
    }

    /// The playground closed, or the learner pressed q during a long check.
    fn stopped(&mut self) -> bool {
        self.app.has_exited() || self.input.quit_requested()
    }

    fn save(&mut self) {
        if let Err(error) = self.state.save(&root()) {
            self.notify(format!("Could not save your progress: {error}"), true);
        }
    }

    fn notify(&mut self, text: String, error: bool) {
        self.message = Some(Message { text, error });
    }

    /// Returns false to end the session.
    fn handle(&mut self, key: Key) -> bool {
        if key == Key::Interrupt {
            return false;
        }
        match self.view {
            View::Main => return self.main_key(key),
            View::ConfirmReset => self.confirm_key(key),
            View::List(_) => self.list_key(key),
        }
        true
    }

    fn main_key(&mut self, key: Key) -> bool {
        let complete = self.lesson().is_none();
        match key {
            Key::Char('q') => return false,
            Key::Char('n') if self.done() => self.next(),
            Key::Char('h') if self.more_hints() => {
                self.hints += 1;
                self.render();
            }
            Key::Char('l') => {
                let current = self.state.current.min(LESSONS.len() - 1);
                self.view = View::List(List::new(current));
                self.render();
            }
            Key::Char('c') => self.check_everything(),
            Key::Char('x') if !complete => {
                self.view = View::ConfirmReset;
                self.render();
            }
            _ => {}
        }
        true
    }

    fn confirm_key(&mut self, key: Key) {
        match key {
            Key::Char('y' | 'Y') => {
                self.view = View::Main;
                self.reset(self.state.current);
                // The file watcher checks the restored exercise.
                self.render();
            }
            Key::Char('n' | 'N') | Key::Esc => {
                self.view = View::Main;
                self.render();
            }
            _ => {}
        }
    }

    fn reset(&mut self, index: usize) -> bool {
        let lesson = &LESSONS[index];
        match reset_exercise(index) {
            Ok(()) => {
                self.state.set_done(index, false);
                self.save();
                self.notify(
                    format!(
                        "The exercise {} has been reset. `git stash pop` brings your version back.",
                        lesson.name
                    ),
                    false,
                );
                true
            }
            Err(error) => {
                self.notify(error, true);
                false
            }
        }
    }

    /// Rustlings' `n`: the next pending exercise, or a final check of them all.
    fn next(&mut self) {
        let next = self.state.next_pending();
        if next < LESSONS.len() {
            self.open(next, None);
        } else {
            self.check_everything();
        }
    }

    /// Navigate, then check the exercise unless its report is already known.
    fn open(&mut self, index: usize, report: Option<Report>) {
        self.state.current = index;
        self.save();
        self.hints = 0;
        self.message = None;
        match report {
            Some(report) => {
                self.report = Some(report);
                self.relaunch();
            }
            None => self.refresh(),
        }
    }

    /// Check the current exercise, then rebuild the preview to match it.
    fn refresh(&mut self) {
        // Capture first, so edits made during the check trigger another one.
        self.last_change = source_fingerprint();
        self.pending_change = self.last_change;
        let index = self.state.current;
        if index < LESSONS.len() {
            self.show_loading("Checking the exercise. Please wait…");
            let _ = self
                .app
                .publish(index, self.report.as_ref(), Refresh::Checking);
            let (app, input) = (&mut self.app, &self.input);
            let report =
                check_lesson_with_cancel(index, &mut || app.has_exited() || input.quit_requested());
            if self.stopped() {
                return;
            }
            self.state.set_done(index, report.passed());
            self.save();
            self.report = Some(report);
        } else {
            self.report = None;
        }
        self.relaunch();
    }

    fn relaunch(&mut self) {
        self.app.completed = self.state.completed();
        self.show_loading("Refreshing the playground…");
        let input = &self.input;
        self.app_error = self
            .app
            .restart(self.state.current, self.report.as_ref(), &mut || {
                input.quit_requested()
            })
            .err();
        if self.stopped() {
            return;
        }
        self.render();
    }

    /// Rustlings' `c`: check everything, then continue at the first pending
    /// exercise if the current one is done.
    fn check_everything(&mut self) {
        self.view = View::Main;
        let index = self.state.current;
        let _ = self
            .app
            .publish(index, self.report.as_ref(), Refresh::Checking);
        self.show_loading("Checking all exercises. Please wait…");
        let (app, input) = (&mut self.app, &self.input);
        let reports = check_all(&mut || app.has_exited() || input.quit_requested());
        if self.stopped() {
            return;
        }
        for (i, report) in reports.iter().enumerate() {
            self.state.set_done(i, report.passed());
        }
        self.save();
        self.summary = Some(self.check_summary(&reports));
        match reports.iter().position(|report| !report.passed()) {
            None => self.open(LESSONS.len(), None),
            Some(first) if reports.get(index).is_none_or(Report::passed) => {
                self.open(first, Some(reports[first].clone()));
            }
            Some(_) => {
                // Stay, keeping the preview and its state; just update its status.
                self.report = Some(reports[index].clone());
                self.app.completed = self.state.completed();
                let refresh = if self.app_error.is_some() {
                    Refresh::Failed
                } else {
                    Refresh::Current
                };
                let _ = self.app.publish(index, self.report.as_ref(), refresh);
                self.render();
            }
        }
    }

    fn check_summary(&self, reports: &[Report]) -> String {
        let done = reports.iter().filter(|r| r.passed()).count();
        let cells: Vec<_> = LESSONS
            .iter()
            .zip(reports)
            .map(|(lesson, report)| {
                if report.passed() {
                    self.ui.ink(&format!("✓{}", lesson.id), "32")
                } else {
                    self.ui.ink(&format!("✗{}", lesson.id), "31")
                }
            })
            .collect();
        let per_line = (self.ui.width() / 4).max(1);
        let grid: Vec<_> = cells.chunks(per_line).map(|row| row.join(" ")).collect();
        format!(
            "{}\n{}",
            self.ui.ink(
                &format!("Checked all exercises: {done} of {} done", LESSONS.len()),
                "1"
            ),
            grid.join("\n")
        )
    }

    fn show_loading(&mut self, status: &str) {
        let screen = self.main_screen(Some(status));
        self.size = self.ui.size();
        self.ui.show(&screen);
    }

    fn render(&mut self) {
        self.size = self.ui.size();
        let screen = match &mut self.view {
            View::Main => self.main_screen(None),
            View::ConfirmReset => self.reset_screen(),
            View::List(list) => {
                let mut list = std::mem::replace(list, List::new(0));
                let screen = list.screen(&self.ui, &self.state);
                self.view = View::List(list);
                screen
            }
        };
        self.ui.show(&screen);
    }

    /// Rustlings' layout: output, hint, done message, progress, exercise, prompt.
    fn main_screen(&self, loading: Option<&str>) -> String {
        let ui = &self.ui;
        let width = ui.width();
        let mut screen = String::new();
        if loading.is_none() {
            if let Some(summary) = &self.summary {
                screen.push_str(summary);
                screen.push_str("\n\n");
            }
            let failure = self.report.as_ref().filter(|r| !r.passed());
            if let Some(report) = failure {
                screen.push_str(&ui.report_diagnostic(report));
                screen.push_str("\n\n");
            }
            if let Some(error) = &self.app_error {
                let heading = ui.ink("The playground could not refresh", "1;31");
                // A compiler error already shown above usually broke the build too.
                if failure.is_some_and(|r| r.state == crate::CheckState::BuildError) {
                    screen.push_str(&format!("{heading}; it shows the previous preview.\n\n"));
                } else {
                    screen.push_str(&format!("{heading}:\n{}\n\n", error.trim()));
                }
            }
            if let Some(lesson) = self.lesson() {
                for level in 0..self.hints.min(lesson.hints.len()) {
                    let heading = if lesson.hints.len() == 1 {
                        "Hint".to_owned()
                    } else {
                        format!("Hint {} of {}", level + 1, lesson.hints.len())
                    };
                    screen.push_str(&ui.ink(&heading, "1;4;36"));
                    screen.push('\n');
                    for line in wrap(lesson.hints[level], width) {
                        screen.push_str(&line);
                        screen.push('\n');
                    }
                    screen.push('\n');
                }
            }
            if self.done() {
                screen.push_str(&ui.ink("Exercise done ✓", "1;32"));
                screen.push('\n');
                screen.push_str(
                    "When done experimenting, press n to move on to the next exercise.\n\n",
                );
            } else if self.lesson().is_none() {
                screen.push_str(&ui.ink("All exercises complete! ✓", "1;32"));
                screen.push_str("\nRevisit any exercise from the list with l.\n\n");
            }
            if let Some(message) = &self.message {
                let tone = if message.error { "1;31" } else { "1;36" };
                screen.push_str(&ui.ink(&message.text, tone));
                screen.push_str("\n\n");
            }
        }
        screen.push_str(&ui.progress(self.state.completed()));
        screen.push('\n');
        if let Some(lesson) = self.lesson() {
            screen.push_str(&format!(
                "Current exercise: {}\n",
                ui.ink(lesson.file, "1;34")
            ));
            for line in wrap(&format!("Goal: {}", lesson.objective), width) {
                screen.push_str(&line);
                screen.push('\n');
            }
        }
        screen.push('\n');
        match loading {
            Some(status) => screen.push_str(&ui.ink(status, "1;34")),
            None => screen.push_str(&self.prompt()),
        }
        screen
    }

    fn prompt(&self) -> String {
        let mut keys = Vec::new();
        if self.done() {
            keys.push(("n", "next"));
        }
        if self.more_hints() {
            keys.push(("h", if self.hints == 0 { "hint" } else { "next hint" }));
        }
        keys.extend([("l", "list"), ("c", "check all")]);
        if self.lesson().is_some() {
            keys.push(("x", "reset"));
        }
        keys.push(("q", "quit"));
        let keys: Vec<_> = keys
            .into_iter()
            .map(|(key, label)| format!("{}:{label}", self.ui.ink(key, "1")))
            .collect();
        format!("{} ? ", keys.join(" / "))
    }

    fn reset_screen(&self) -> String {
        let file = self.lesson().map_or("", |l| l.file);
        format!("Resetting will undo all your changes to the file {file}\nReset (y/n)? ")
    }

    fn list_key(&mut self, key: Key) {
        let View::List(list) = &mut self.view else {
            return;
        };
        let rows = list.rows(&self.state);
        if let Some(query) = &mut list.search {
            match key {
                Key::Esc | Key::Enter => list.search = None,
                Key::Char(c) => query.push(c),
                Key::Backspace => {
                    query.pop();
                }
                _ => return,
            }
            list.find(&rows);
            self.render();
            return;
        }
        list.message.clear();
        let selected = rows.get(list.selected).copied();
        let last = rows.len().saturating_sub(1);
        match key {
            Key::Char('q') => self.view = View::Main,
            Key::Down | Key::Char('j') => list.selected = (list.selected + 1).min(last),
            Key::Up | Key::Char('k') => list.selected = list.selected.saturating_sub(1),
            Key::Home | Key::Char('g') => list.selected = 0,
            Key::End | Key::Char('G') => list.selected = last,
            Key::Char('d') => list.toggle(Filter::Done, &self.state),
            Key::Char('p') => list.toggle(Filter::Pending, &self.state),
            Key::Char('s' | '/') => {
                list.search = Some(String::new());
                list.find(&rows);
            }
            Key::Char('r') => match selected {
                Some(index) => {
                    let reset = self.reset(index);
                    let message = self.message.take().map(|m| m.text).unwrap_or_default();
                    if let View::List(list) = &mut self.view {
                        list.message = if reset {
                            format!("The exercise `{}` has been reset", LESSONS[index].name)
                        } else {
                            message
                        };
                        let rows = list.rows(&self.state);
                        list.selected = list.selected.min(rows.len().saturating_sub(1));
                    }
                }
                None => list.message = "Nothing selected to reset!".into(),
            },
            Key::Char('c') => match selected {
                Some(index) => {
                    self.view = View::Main;
                    self.summary = None;
                    self.open(index, None);
                    return;
                }
                None => list.message = "Nothing selected to continue at!".into(),
            },
            // Redraw to remove the message.
            Key::Esc => {}
            _ => return,
        }
        self.render();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Filter {
    None,
    Done,
    Pending,
}

/// Rustlings' exercise list, drawn in the session's terminal.
struct List {
    /// The cursor, as a position among the rows that pass the filter.
    selected: usize,
    offset: usize,
    filter: Filter,
    search: Option<String>,
    message: String,
}

impl List {
    fn new(current: usize) -> Self {
        Self {
            selected: current,
            offset: 0,
            filter: Filter::None,
            search: None,
            message: String::new(),
        }
    }

    /// Lesson indices that pass the filter.
    fn rows(&self, state: &LessonState) -> Vec<usize> {
        (0..LESSONS.len())
            .filter(|&i| match self.filter {
                Filter::None => true,
                Filter::Done => state.is_done(LESSONS[i].name),
                Filter::Pending => !state.is_done(LESSONS[i].name),
            })
            .collect()
    }

    fn toggle(&mut self, filter: Filter, state: &LessonState) {
        let name = if filter == Filter::Done {
            "DONE"
        } else {
            "PENDING"
        };
        if self.filter == filter {
            self.filter = Filter::None;
            self.message = format!("Disabled filter {name}");
        } else {
            self.filter = filter;
            let key = if filter == Filter::Done { 'd' } else { 'p' };
            self.message =
                format!("Enabled filter {name} │ Press {key} again to disable the filter");
        }
        self.selected = self.selected.min(self.rows(state).len().saturating_sub(1));
    }

    /// Move the cursor to the first exercise whose name contains the query.
    fn find(&mut self, rows: &[usize]) {
        let Some(query) = self.search.as_deref().filter(|q| !q.is_empty()) else {
            return;
        };
        if let Some(position) = rows.iter().position(|&i| LESSONS[i].name.contains(query)) {
            self.selected = position;
        }
    }

    fn footer(&self) -> Vec<String> {
        if let Some(query) = &self.search {
            return vec![format!("search:{query}|")];
        }
        if !self.message.is_empty() {
            return vec![self.message.clone()];
        }
        let filter = match self.filter {
            Filter::None => "<d>one/<p>ending",
            Filter::Done => "[<d>one]/<p>ending",
            Filter::Pending => "<d>one/[<p>ending]",
        };
        vec![
            "↓/j ↑/k home/g end/G | <c>ontinue at | <r>eset exercise".into(),
            format!("<s>earch | filter {filter} | <q>uit list"),
        ]
    }

    fn screen(&mut self, ui: &Terminal, state: &LessonState) -> String {
        let (height, width) = ui.size();
        let footer: Vec<_> = self
            .footer()
            .iter()
            .map(|line| ui.ink(&truncate(line, width), "35"))
            .collect();
        let rows = self.rows(state);
        // Header and progress bar, plus the footer.
        let visible = height.saturating_sub(2 + footer.len()).max(1);
        self.selected = self.selected.min(rows.len().saturating_sub(1));
        if self.selected < self.offset {
            self.offset = self.selected;
        } else if self.selected >= self.offset + visible {
            self.offset = self.selected + 1 - visible;
        }
        let mut lines = vec![truncate(&table_header(), width)];
        for (position, &index) in rows.iter().enumerate().skip(self.offset).take(visible) {
            lines.push(table_row(
                ui,
                state,
                index,
                position == self.selected,
                width,
            ));
        }
        lines.resize(visible + 1, String::new());
        lines.push(ui.progress(state.completed()));
        lines.extend(footer);
        lines.join("\n")
    }
}

const NAME_WIDTH: usize = 14;

fn table_header() -> String {
    format!("  Current  State    {:NAME_WIDTH$}Path", "Name")
}

fn table_row(
    ui: &Terminal,
    state: &LessonState,
    index: usize,
    selected: bool,
    width: usize,
) -> String {
    let lesson = &LESSONS[index];
    let marker = if selected {
        ui.ink("> ", "1;36")
    } else {
        "  ".into()
    };
    let current = if index == state.current {
        ui.ink(">>>>>>>  ", "31")
    } else {
        " ".repeat(9)
    };
    let status = if state.is_done(lesson.name) {
        ui.ink("DONE   ", "32")
    } else {
        ui.ink("PENDING", "33")
    };
    let name = format!("{:NAME_WIDTH$}", lesson.name);
    let name = if selected { ui.ink(&name, "1") } else { name };
    let path = truncate(
        lesson.file,
        width.saturating_sub(2 + 9 + 7 + 2 + NAME_WIDTH),
    );
    format!("{marker}{current}{status}  {name}{path}")
}

/// The exercise table for `./gpui-lings list`.
pub fn lesson_table(ui: &Terminal, state: &LessonState) -> String {
    let width = ui.width();
    let mut lines = vec![truncate(&table_header(), width)];
    lines.extend((0..LESSONS.len()).map(|index| table_row(ui, state, index, false, width)));
    lines.push(ui.progress(state.completed()));
    lines.join("\n")
}

fn truncate(text: &str, width: usize) -> String {
    if text.chars().count() <= width {
        return text.to_owned();
    }
    let mut short: String = text.chars().take(width.saturating_sub(1)).collect();
    short.push('…');
    short
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CheckState, terminal::plain_terminal};

    fn session(current: usize, report: Option<CheckState>) -> Session {
        let mut state = LessonState::load(std::path::Path::new("/nonexistent-gpui-state"));
        state.current = current;
        Session {
            ui: plain_terminal(80),
            input: Input::detached(),
            state,
            app: PlaygroundProcess::default(),
            report: report.map(|state| Report {
                state,
                summary: String::new(),
                details: "error[E0308]: mismatched types".into(),
            }),
            app_error: None,
            hints: 0,
            summary: None,
            message: None,
            view: View::Main,
            last_change: None,
            pending_change: None,
            size: (24, 80),
        }
    }

    #[test]
    fn main_screen_follows_rustlings_order_and_prompt() {
        let session = session(19, Some(CheckState::BuildError));
        let screen = session.main_screen(None);
        let error = screen.find("error[E0308]").unwrap();
        let progress = screen.find("Progress: [").unwrap();
        let current = screen.find("Current exercise: exercises/").unwrap();
        assert!(error < progress && progress < current);
        assert!(screen.ends_with("h:hint / l:list / c:check all / x:reset / q:quit ? "));
        assert!(
            !screen.contains("n:next"),
            "n only appears once the check passes"
        );

        let mut session = self::session(19, Some(CheckState::Passed));
        assert!(session.main_screen(None).contains("Exercise done ✓"));
        assert!(session.prompt().starts_with("n:next / h:hint"));
        session.hints = 1;
        let screen = session.main_screen(None);
        assert!(screen.contains("Hint 1 of 3\n"));
        assert!(session.prompt().contains("h:next hint"));
        session.hints = 3;
        assert!(!session.prompt().contains("h:"), "every hint is visible");
    }

    #[test]
    fn loading_hides_results_until_the_refresh_finishes() {
        let session = session(3, Some(CheckState::BuildError));
        let screen = session.main_screen(Some("Checking the exercise. Please wait…"));
        assert!(!screen.contains("error[E0308]"));
        assert!(screen.ends_with("Checking the exercise. Please wait…"));
    }

    #[test]
    fn complete_course_offers_the_list_and_no_reset() {
        let session = session(LESSONS.len(), None);
        let screen = session.main_screen(None);
        assert!(screen.contains("All exercises complete!"));
        assert!(!screen.contains("Current exercise"));
        assert!(screen.ends_with("l:list / c:check all / q:quit ? "));
    }

    #[test]
    fn single_keys_drive_hints_list_and_reset_confirmation() {
        let mut session = session(0, Some(CheckState::Failed));
        assert!(session.handle(Key::Char('h')));
        assert_eq!(session.hints, 1);
        assert!(
            session.handle(Key::Char('h')),
            "no further hint level is harmless"
        );
        assert_eq!(session.hints, 1);
        assert!(session.handle(Key::Char('x')));
        assert!(matches!(session.view, View::ConfirmReset));
        assert!(session.reset_screen().ends_with("Reset (y/n)? "));
        assert!(session.handle(Key::Char('n')));
        assert!(matches!(session.view, View::Main));
        assert!(session.handle(Key::Char('l')));
        assert!(session.handle(Key::Down));
        let View::List(list) = &session.view else {
            panic!("l opens the list")
        };
        assert_eq!(list.selected, 1);
        assert!(session.handle(Key::Char('q')), "q closes only the list");
        assert!(matches!(session.view, View::Main));
        assert!(!session.handle(Key::Char('q')));
        assert!(!session.handle(Key::Interrupt));
    }

    #[test]
    fn list_scrolls_filters_and_searches() {
        let mut state = LessonState::load(std::path::Path::new("/nonexistent-gpui-state"));
        state.set_done(0, true);
        state.set_done(1, true);
        let ui = Terminal::new_sized(10, 80);
        let mut list = List::new(30);
        let screen = list.screen(&ui, &state);
        assert_eq!(screen.lines().count(), 10, "the list fills the terminal");
        assert!(screen.contains("> "), "the selection stays visible");
        assert!(screen.contains(LESSONS[30].name));

        list.toggle(Filter::Done, &state);
        assert_eq!(list.rows(&state), [0, 1]);
        assert_eq!(list.selected, 1);
        list.toggle(Filter::Done, &state);
        assert_eq!(list.filter, Filter::None);

        list.search = Some("async".into());
        list.find(&list.rows(&state));
        assert_eq!(LESSONS[list.selected].name, "async1");
        assert!(list.footer()[0].starts_with("search:async"));
    }
}
