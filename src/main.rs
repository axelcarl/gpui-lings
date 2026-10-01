#[path = "../shared/lessons.rs"]
pub mod lessons;
#[path = "../shared/preview.rs"]
mod preview;
mod process;
mod terminal;
use process::InterruptibleCommand;

use preview::{PreviewState, Refresh};

use lessons::{LESSONS, Lesson, Verifier, progress_index, progress_value};
use std::{
    collections::hash_map::DefaultHasher,
    env, fs,
    hash::{Hash, Hasher},
    io::{self, BufRead},
    path::{Path, PathBuf},
    process::{Child, Command, ExitCode, ExitStatus, Output, Stdio},
    sync::mpsc,
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CheckState {
    Passed,
    Failed,
    BuildError,
}

#[derive(Clone)]
struct Report {
    state: CheckState,
    summary: String,
    details: String,
}

impl Report {
    fn error(message: impl Into<String>) -> Self {
        let message = message.into();
        Self {
            state: CheckState::BuildError,
            summary: message.clone(),
            details: message,
        }
    }
    fn status(&self) -> &'static str {
        match self.state {
            CheckState::Passed => "passed",
            CheckState::Failed => "failed",
            CheckState::BuildError => "error",
        }
    }
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
fn manifest() -> PathBuf {
    root().join("playground/Cargo.toml")
}
fn progress_file() -> PathBuf {
    root().join(".gpui-lings-progress")
}
fn load_progress() -> usize {
    progress_index(&fs::read_to_string(progress_file()).unwrap_or_default())
}
fn save_progress(index: usize) -> io::Result<()> {
    let temporary = root().join(".gpui-lings-progress.tmp");
    fs::write(&temporary, format!("{}\n", progress_value(index)))?;
    fs::rename(temporary, progress_file())
}

fn output_text(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn evaluate_output(success: bool, details: String) -> Report {
    // libtest exits successfully even if its exact filter matched zero tests.
    let ran_one = details.lines().any(|line| line.trim() == "running 1 test");
    if !ran_one {
        return Report {
            state: CheckState::BuildError,
            summary: if details.contains("running 0 tests") {
                "No matching exercise test"
            } else {
                "Check could not run"
            }
            .into(),
            details,
        };
    }
    if success && !details.contains("test result: ok. 1 passed; 0 failed; 0 ignored;") {
        return Report::error("Exercise test did not run; check for #[ignore]");
    }
    if success {
        Report {
            state: CheckState::Passed,
            summary: "Passed".into(),
            details,
        }
    } else {
        Report {
            state: CheckState::Failed,
            summary: "Not passing yet".into(),
            details,
        }
    }
}

fn check_lesson(index: usize) -> Report {
    check_lesson_with_cancel(index, &mut || false)
}

fn check_lesson_with_cancel(index: usize, cancelled: &mut dyn FnMut() -> bool) -> Report {
    let lesson = &LESSONS[index];
    let result = match lesson.verifier {
        Verifier::Lightweight => {
            let directory = root().join("target/lesson-checks");
            if let Err(error) = fs::create_dir_all(&directory) {
                return Report::error(error.to_string());
            }
            let binary = directory.join(format!("lesson-{}{}", lesson.id, env::consts::EXE_SUFFIX));
            match Command::new("rustc")
                .args(["--test", "--edition=2024"])
                .arg(root().join(lesson.file))
                .arg("-o")
                .arg(&binary)
                .interruptible_output(cancelled)
            {
                Ok(output) if output.status.success() => {}
                Ok(output) => {
                    return Report {
                        state: CheckState::BuildError,
                        summary: "Compilation failed".into(),
                        details: output_text(&output),
                    };
                }
                Err(error) => return Report::error(format!("Could not run rustc: {error}")),
            }
            Command::new(binary)
                .args([lesson.test, "--exact", "--color=never"])
                .interruptible_output(cancelled)
        }
        Verifier::Native => Command::new("cargo")
            .args(["test", "--color=never", "--manifest-path"])
            .arg(manifest())
            .args(["--lib", lesson.test, "--", "--exact", "--color=never"])
            .interruptible_output(cancelled),
    };
    match result {
        Ok(output) => evaluate_output(output.status.success(), output_text(&output)),
        Err(error) => Report::error(format!("Could not run the check: {error}")),
    }
}

struct SessionChecks {
    reports: Vec<Report>,
}

impl SessionChecks {
    fn completed(&self) -> usize {
        // Progress is the verified course prefix, not the saved navigation cursor.
        self.reports
            .iter()
            .take_while(|r| r.state == CheckState::Passed)
            .count()
    }

    fn resume_index(&self, current: usize) -> usize {
        self.reports
            .iter()
            .take(current)
            .position(|r| {
                r.state == CheckState::Failed
                    || (current == LESSONS.len() && r.state != CheckState::Passed)
            })
            .unwrap_or(current)
    }
}

fn native_report(test: &str, details: &str) -> Report {
    let prefix = format!("test {test} ... ");
    let result = details.lines().find_map(|line| line.strip_prefix(&prefix));
    let (state, summary) = match result {
        Some("ok") => (CheckState::Passed, "Passed"),
        Some("FAILED") => (CheckState::Failed, "Not passing yet"),
        Some(result) if result.starts_with("ignored") => (
            CheckState::BuildError,
            "Exercise test did not run; check for #[ignore]",
        ),
        _ => (CheckState::BuildError, "Check could not run"),
    };
    // Keep `d` focused on this exercise when a batch contains multiple failures.
    let marker = format!("---- {test} stdout ----\n");
    let details = details.split_once(&marker).map_or(details, |(_, failure)| {
        failure
            .split("\n---- ")
            .next()
            .unwrap()
            .split("\nfailures:\n")
            .next()
            .unwrap()
    });
    Report {
        state,
        summary: summary.into(),
        details: details.into(),
    }
}

fn check_reached(index: usize, cancelled: &mut dyn FnMut() -> bool) -> SessionChecks {
    let reached = &LESSONS[..(index + 1).min(LESSONS.len())];
    let native: Vec<_> = reached
        .iter()
        .filter(|lesson| matches!(lesson.verifier, Verifier::Native))
        .collect();
    // Multiple exact libtest filters share one Cargo build and test process.
    let output = if native.is_empty() {
        None
    } else {
        Some(
            Command::new("cargo")
                .args(["test", "--color=never", "--manifest-path"])
                .arg(manifest())
                .args(["--lib", "--", "--exact", "--color=never", "--format=pretty"])
                .args(native.iter().map(|lesson| lesson.test))
                .interruptible_output(cancelled)
                .map(|output| output_text(&output))
                .unwrap_or_else(|error| format!("Could not run the checks: {error}")),
        )
    };
    let reports = reached
        .iter()
        .enumerate()
        .map(|(index, lesson)| match lesson.verifier {
            Verifier::Lightweight => check_lesson_with_cancel(index, cancelled),
            Verifier::Native => native_report(lesson.test, output.as_deref().unwrap()),
        })
        .collect();
    SessionChecks { reports }
}

fn check_progress(index: &mut usize, app: &mut PlaygroundProcess) -> Option<Report> {
    let checks = check_reached(*index, &mut || app.has_exited());
    if app.has_exited() {
        return None;
    }
    app.completed = checks.completed();
    let resume = checks.resume_index(*index);
    if resume != *index {
        if let Err(error) = save_progress(resume) {
            return Some(Report::error(format!("Could not save your place: {error}")));
        }
        *index = resume;
    }
    checks.reports.get(*index).cloned()
}

/// Hash sorted paths and contents: detect deletion, atomic saves and timestamp ties.
fn fingerprint(directory: &Path) -> io::Result<u64> {
    fn visit(directory: &Path, hasher: &mut DefaultHasher) -> io::Result<()> {
        let mut entries: Vec<_> = fs::read_dir(directory)?.collect::<io::Result<_>>()?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let file = entry.path();
            file.hash(hasher);
            let kind = entry.file_type()?;
            if kind.is_dir() {
                visit(&file, hasher)?;
            } else if kind.is_file() {
                fs::read(file)?.hash(hasher);
            }
        }
        Ok(())
    }
    let mut hasher = DefaultHasher::new();
    visit(directory, &mut hasher)?;
    Ok(hasher.finish())
}

fn source_fingerprint() -> Option<u64> {
    let mut hasher = DefaultHasher::new();
    fingerprint(&root().join("playground/src"))
        .ok()?
        .hash(&mut hasher);
    for file in [
        "playground/Cargo.toml",
        "shared/lessons.rs",
        "shared/preview.rs",
    ] {
        fs::read(root().join(file)).ok()?.hash(&mut hasher);
    }
    Some(hasher.finish())
}

#[derive(Default)]
struct PlaygroundProcess {
    child: Option<Child>,
    directory: Option<PathBuf>,
    generation: u64,
    completed: usize,
}
impl PlaygroundProcess {
    fn exit_status(&mut self) -> io::Result<Option<ExitStatus>> {
        self.child.as_mut().map_or(Ok(None), Child::try_wait)
    }

    fn has_exited(&mut self) -> bool {
        // No child yet (for example after an initial build error) is recoverable.
        !matches!(self.exit_status(), Ok(None))
    }

    fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
    fn publish(&self, index: usize, report: Option<&Report>, refresh: Refresh) -> io::Result<()> {
        if let Some(directory) = &self.directory {
            PreviewState {
                index,
                status: report.map_or("", Report::status).into(),
                completed: self.completed,
                refresh,
            }
            .write(&directory.join(format!("state-{}", self.generation)))?;
        }
        Ok(())
    }

    fn restart(&mut self, index: usize, report: Option<&Report>) -> Result<(), String> {
        let result = self.replace(index, report);
        if result.is_err() {
            let _ = self.publish(index, report, Refresh::Failed);
        }
        result
    }

    fn replace(&mut self, index: usize, report: Option<&Report>) -> Result<(), String> {
        self.publish(index, report, Refresh::Building)
            .map_err(|e| e.to_string())?;
        let output = Command::new("cargo")
            .args(["build", "--color=never", "--manifest-path"])
            .arg(manifest())
            .interruptible_output(&mut || self.has_exited())
            .map_err(|e| format!("Could not run cargo: {e}"))?;
        if !output.status.success() {
            return Err(output_text(&output));
        }
        // Ask Cargo for the target directory, including CARGO_TARGET_DIR/config overrides.
        let metadata = Command::new("cargo")
            .args([
                "metadata",
                "--no-deps",
                "--format-version",
                "1",
                "--manifest-path",
            ])
            .arg(manifest())
            .interruptible_output(&mut || self.has_exited())
            .map_err(|e| e.to_string())?;
        if !metadata.status.success() {
            return Err(output_text(&metadata));
        }
        let target = target_directory(&String::from_utf8_lossy(&metadata.stdout))
            .ok_or_else(|| "Could not locate Cargo's target directory.".to_string())?;
        let binary = PathBuf::from(target)
            .join("debug")
            .join(format!("gpui-lings-playground{}", env::consts::EXE_SUFFIX));
        let directory = match &self.directory {
            Some(directory) => directory.clone(),
            None => {
                let nonce = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos();
                let directory =
                    env::temp_dir().join(format!("gpui-lings-{}-{nonce}", std::process::id()));
                fs::create_dir(&directory).map_err(|e| e.to_string())?;
                self.directory = Some(directory.clone());
                directory
            }
        };
        let generation = self.generation + 1;
        let ready = directory.join(format!("ready-{generation}"));
        let state = directory.join(format!("state-{generation}"));
        // A failed launch can retry the same generation; never reuse its readiness marker.
        let _ = fs::remove_file(&ready);
        PreviewState {
            index,
            status: report.map_or("", Report::status).into(),
            completed: self.completed,
            refresh: Refresh::Current,
        }
        .write(&state)
        .map_err(|e| e.to_string())?;
        let log = directory.join(format!("startup-{generation}.log"));
        let stderr = fs::File::create(&log).map_err(|e| e.to_string())?;
        if self.has_exited() {
            return Err("Playground closed".into());
        }
        let replacement = Command::new(binary)
            .env(
                "GPUI_LINGS_LESSON",
                LESSONS.get(index).map_or("complete", |l| l.id),
            )
            .env("GPUI_LINGS_STATUS", report.map_or("", Report::status))
            .env("GPUI_LINGS_COMPLETED", self.completed.to_string())
            .env("GPUI_LINGS_PREVIEW_STATE", state)
            .env("GPUI_LINGS_WINDOW_BOUNDS", directory.join("bounds"))
            .env("GPUI_LINGS_READY", &ready)
            .env("GPUI_LINGS_MANAGED", "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(stderr)
            .spawn()
            .map_err(|e| e.to_string())?;
        self.handoff(replacement, &ready, Duration::from_secs(15))
            .map_err(|error| format!("{error}\n{}", fs::read_to_string(log).unwrap_or_default()))?;
        self.generation = generation;
        Ok(())
    }

    fn handoff(
        &mut self,
        mut replacement: Child,
        ready: &Path,
        timeout: Duration,
    ) -> Result<(), String> {
        // Keep the old window until the replacement has rendered its first frame.
        if let Err(error) =
            wait_for_preview(&mut replacement, ready, timeout, &mut || self.has_exited())
        {
            let _ = replacement.kill();
            let _ = replacement.wait();
            return Err(error);
        }
        self.stop();
        self.child = Some(replacement);
        Ok(())
    }
}
impl Drop for PlaygroundProcess {
    fn drop(&mut self) {
        self.stop();
        if let Some(directory) = &self.directory {
            let _ = fs::remove_dir_all(directory);
        }
    }
}

fn wait_for_preview(
    child: &mut Child,
    ready: &Path,
    timeout: Duration,
    cancelled: &mut dyn FnMut() -> bool,
) -> Result<(), String> {
    let started = Instant::now();
    loop {
        if cancelled() {
            return Err("Playground closed".into());
        }
        if let Some(status) = child.try_wait().map_err(|e| e.to_string())? {
            // A user can close the replacement in the gap between its first
            // frame and our poll. Adopt that clean exit so it closes the session.
            if status.success() && ready.is_file() {
                return Ok(());
            }
            return Err(format!(
                "Playground exited before opening its window ({status})."
            ));
        }
        if ready.is_file() {
            return Ok(());
        }
        if started.elapsed() >= timeout {
            return Err("Playground did not open its window; keeping the previous preview.".into());
        }
        thread::sleep(Duration::from_millis(25));
    }
}

/// Cargo emits a JSON string here. Decode escapes without adding a guide dependency.
fn target_directory(metadata: &str) -> Option<String> {
    let rest = metadata
        .split("\"target_directory\":")
        .nth(1)?
        .trim_start()
        .strip_prefix('"')?;
    let mut result = String::new();
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => return Some(result),
            '\\' => match chars.next()? {
                '\\' => result.push('\\'),
                '"' => result.push('"'),
                '/' => result.push('/'),
                'n' => result.push('\n'),
                'r' => result.push('\r'),
                't' => result.push('\t'),
                'b' => result.push('\u{0008}'),
                'f' => result.push('\u{000c}'),
                'u' => {
                    let code: String = chars.by_ref().take(4).collect();
                    result.push(char::from_u32(u32::from_str_radix(&code, 16).ok()?)?);
                }
                _ => return None,
            },
            _ => result.push(c),
        }
    }
    None
}

fn learning_session() -> ExitCode {
    let ui = terminal::Terminal::new();
    let mut index = load_progress();
    let mut app = PlaygroundProcess::default();
    let mut last_change = source_fingerprint();
    ui.dashboard(index, None, "Checking…", None);
    let mut report = check_progress(&mut index, &mut app);
    ui.dashboard(index, report.as_ref(), "Building playground…", None);
    let mut app_error = app.restart(index, report.as_ref()).err();
    ui.dashboard(index, report.as_ref(), app_message(&app_error), None);

    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        for line in io::stdin().lock().lines() {
            match line {
                Ok(line) if tx.send(line.trim().to_owned()).is_ok() => {}
                _ => break,
            }
        }
    });
    let mut pending_change = last_change;
    // The lesson whose hints are showing, and the next level to reveal.
    let mut hint = (index, 0);
    loop {
        if app.has_exited() {
            break;
        }
        let mut rerun = false;
        let command = rx.recv_timeout(Duration::from_millis(100));
        if app.has_exited() {
            break;
        }
        match command {
            Ok(command) => {
                let mut panel = None;
                let mut message = app_message(&app_error).to_string();
                match command.as_str() {
                    "q" | "quit" => break,
                    "h" | "hint" => {
                        let level = if hint.0 == index { hint.1 } else { 0 };
                        panel = LESSONS.get(index).map(|l| l.hint(level, "h again"));
                        hint = (index, level + 1);
                    }
                    "?" | "help" => panel = Some(terminal::SESSION_HELP.into()),
                    "l" | "list" => panel = Some(terminal::lesson_list(index)),
                    "d" | "details" => {
                        panel = Some(format!(
                            "CHECK DETAILS\n{}{}",
                            report
                                .as_ref()
                                .map_or("No check needed.", |r| r.details.as_str()),
                            app_error
                                .as_ref()
                                .map_or(String::new(), |e| format!("\nPLAYGROUND BUILD\n{e}"))
                        ))
                    }
                    "g" | "guide" => {
                        panel = LESSONS.get(index).map(|l| {
                            fs::read_to_string(root().join(l.file))
                                .map(|source| lessons::instructions(&source))
                                .unwrap_or_else(|e| e.to_string())
                        })
                    }
                    "r" | "run" => rerun = true,
                    "p" | "previous" if index > 0 => match save_progress(index - 1) {
                        Ok(()) => {
                            index -= 1;
                            rerun = true;
                        }
                        Err(error) => message = format!("Could not save your place: {error}"),
                    },
                    "n" | "next" if index < LESSONS.len() => {
                        ui.dashboard(index, report.as_ref(), "Checking…", None);
                        let _ = app.publish(index, report.as_ref(), Refresh::Checking);
                        let previous_index = index;
                        report = check_progress(&mut index, &mut app);
                        if app.has_exited() {
                            break;
                        }
                        rerun |= index != previous_index;
                        let refresh = if app_error.is_some() {
                            Refresh::Failed
                        } else {
                            Refresh::Current
                        };
                        let _ = app.publish(index, report.as_ref(), refresh);
                        if report
                            .as_ref()
                            .is_some_and(|r| r.state == CheckState::Passed)
                        {
                            match save_progress(index + 1) {
                                Ok(()) => {
                                    index += 1;
                                    rerun = true;
                                }
                                Err(error) => {
                                    message = format!("Could not save your place: {error}")
                                }
                            }
                        } else {
                            message = "Not passing yet · h for a hint".into();
                        }
                    }
                    "n" | "next" => message = "All exercises complete · p to revisit".into(),
                    "p" | "previous" => message = "Already at the first exercise".into(),
                    "" => {}
                    _ => message = "Unknown command · ? for help".into(),
                }
                if !rerun {
                    ui.dashboard(index, report.as_ref(), &message, panel.as_deref());
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
            Err(mpsc::RecvTimeoutError::Timeout) => {}
        }
        let change = source_fingerprint();
        // Wait for two matching observations so editor atomic saves settle first.
        if change != last_change && change == pending_change {
            rerun = true;
        }
        pending_change = change;
        if rerun {
            // Capture before the build so edits made during a build trigger the next check.
            last_change = change;
            ui.dashboard(index, report.as_ref(), "Checking…", None);
            let _ = app.publish(index, report.as_ref(), Refresh::Checking);
            report = check_progress(&mut index, &mut app);
            if app.has_exited() {
                break;
            }
            ui.dashboard(index, report.as_ref(), "Refreshing playground…", None);
            app_error = app.restart(index, report.as_ref()).err();
            if app.has_exited() {
                break;
            }
            ui.dashboard(index, report.as_ref(), app_message(&app_error), None);
        }
    }
    let code = match app.exit_status() {
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
    println!("\n  Session closed.\n");
    code
}

fn app_message(error: &Option<String>) -> &'static str {
    if error.is_some() {
        "Playground refresh failed · d for details"
    } else {
        ""
    }
}

fn selected_lesson(id: Option<&str>) -> Result<usize, String> {
    match id {
        Some(id) => LESSONS
            .iter()
            .position(|l| l.id == id)
            .ok_or_else(|| format!("Unknown lesson: {id}. Use list to see the available IDs.")),
        None => Ok(load_progress().min(LESSONS.len() - 1)),
    }
}

fn print_usage() {
    println!(
        "\n  ./gpui-lings             Start the session\n  ./gpui-lings list        List exercises\n  ./gpui-lings check [ID]  Check all exercises, or one\n  ./gpui-lings hint [ID] [N]  Show a hint, or its Nth level\n  ./gpui-lings app [ID]    Preview an exercise\n\n  In a session, use ? for commands. Type a command, then Enter.\n"
    );
}

fn execute(args: &[String]) -> Result<ExitCode, String> {
    let command = args.first().map(String::as_str);
    let id = args.get(1).map(String::as_str);
    let max_args = match command {
        Some("hint") => 3,
        Some("check" | "app") => 2,
        _ => 1,
    };
    if args.len() > max_args {
        return Err("Too many arguments. Use ./gpui-lings help.".into());
    }
    let ui = terminal::Terminal::new();
    match command {
        None | Some("watch") => Ok(learning_session()),
        Some("help" | "--help" | "-h") => {
            print_usage();
            Ok(ExitCode::SUCCESS)
        }
        Some("list") => {
            println!("\n{}", terminal::lesson_list(load_progress()));
            Ok(ExitCode::SUCCESS)
        }
        Some("hint") => {
            let l = &LESSONS[selected_lesson(id)?];
            let level = match args.get(2) {
                Some(n) => n
                    .parse::<usize>()
                    .ok()
                    .filter(|&n| n > 0)
                    .ok_or_else(|| format!("Hint level must be 1 or more, not {n}."))?,
                None => 1,
            };
            let more = format!("Run ./gpui-lings hint {} {}", l.id, level + 1);
            let text = l.hint(level - 1, &more).replace('\n', "\n  ");
            println!("\n  {} / {}\n\n  {text}\n", l.id, l.title);
            Ok(ExitCode::SUCCESS)
        }
        Some("check") => {
            let indices: Vec<_> = match id {
                Some(_) => vec![selected_lesson(id)?],
                None => (0..LESSONS.len()).collect(),
            };
            let mut passed = true;
            for index in indices {
                let report = check_lesson(index);
                ui.check_result(&LESSONS[index], &report);
                passed &= report.state == CheckState::Passed;
            }
            Ok(if passed {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            })
        }
        Some("app") => {
            let index = if id.is_some() {
                selected_lesson(id)?
            } else {
                load_progress()
            };
            let checks = check_reached(index, &mut || false);
            let status = Command::new("cargo")
                .args(["run", "--manifest-path"])
                .arg(manifest())
                .env(
                    "GPUI_LINGS_LESSON",
                    LESSONS.get(index).map_or("complete", |l| l.id),
                )
                .env("GPUI_LINGS_COMPLETED", checks.completed().to_string())
                .env(
                    "GPUI_LINGS_STATUS",
                    checks.reports.get(index).map_or("", Report::status),
                )
                .status()
                .map_err(|e| format!("Could not run cargo: {e}"))?;
            Ok(if status.success() {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            })
        }
        Some(other) => Err(format!("Unknown command: {other}. Use ./gpui-lings help.")),
    }
}

fn main() -> ExitCode {
    match execute(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("\n  {error}\n");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn checks(states: &[CheckState]) -> SessionChecks {
        SessionChecks {
            reports: states
                .iter()
                .map(|state| Report {
                    state: *state,
                    summary: String::new(),
                    details: String::new(),
                })
                .collect(),
        }
    }

    #[test]
    fn stashing_solutions_invalidates_saved_progress_and_returns_to_first_failure() {
        use CheckState::{Failed, Passed};
        let solved = checks(&[Passed; 7]);
        assert_eq!(solved.completed(), 7);
        assert_eq!(solved.resume_index(6), 6);
        let stashed = checks(&[Failed; 7]);
        assert_eq!(stashed.completed(), 0);
        assert_eq!(stashed.resume_index(6), 0);
        let earlier_regression = checks(&[Passed, Passed, Failed, Passed, Passed, Passed, Passed]);
        assert_eq!(earlier_regression.completed(), 2);
        assert_eq!(earlier_regression.resume_index(6), 2);
        let current_failure = checks(&[Passed, Passed, Failed]);
        assert_eq!(current_failure.completed(), 2);
        assert_eq!(current_failure.resume_index(2), 2);
        let complete = checks(&[Passed; LESSONS.len()]);
        assert_eq!(complete.completed(), LESSONS.len());
        assert_eq!(complete.resume_index(LESSONS.len()), LESSONS.len());
        assert_eq!(stashed.resume_index(LESSONS.len()), 0);
    }

    #[test]
    fn compilation_failure_clears_unverified_progress_without_navigating() {
        use CheckState::{BuildError, Passed};
        let checks = checks(&[Passed, Passed, Passed, BuildError, BuildError, BuildError]);
        assert_eq!(checks.completed(), 3);
        assert_eq!(checks.resume_index(5), 5);
    }

    #[test]
    fn batch_results_are_attributed_to_the_exact_exercise() {
        let output = "running 3 tests\ntest lesson::one ... ok\ntest lesson::two ... FAILED\ntest lesson::three ... ignored\n\nfailures:\n\n---- lesson::two stdout ----\nexpected a gap\n\nfailures:\n    lesson::two\n";
        assert_eq!(
            native_report("lesson::one", output).state,
            CheckState::Passed
        );
        let failed = native_report("lesson::two", output);
        assert_eq!(failed.state, CheckState::Failed);
        assert_eq!(failed.details.trim(), "expected a gap");
        assert_eq!(
            native_report("lesson::three", output).state,
            CheckState::BuildError
        );
        assert_eq!(
            native_report("lesson::missing", output).state,
            CheckState::BuildError
        );
        assert_eq!(
            native_report("lesson::one", "compilation failed").state,
            CheckState::BuildError
        );
    }

    #[cfg(unix)]
    #[test]
    fn failed_replacements_keep_the_old_process_alive() {
        let ready = env::temp_dir().join(format!("gpui-handoff-failure-{}", std::process::id()));
        let mut app = PlaygroundProcess {
            child: Some(Command::new("sleep").arg("30").spawn().unwrap()),
            directory: None,
            generation: 0,
            completed: 0,
        };
        let old = app.child.as_ref().unwrap().id();
        let replacement = Command::new("sleep").arg("30").spawn().unwrap();
        assert!(app.handoff(replacement, &ready, Duration::ZERO).is_err());
        assert_eq!(app.child.as_ref().unwrap().id(), old);
        assert!(app.child.as_mut().unwrap().try_wait().unwrap().is_none());
        let replacement = Command::new("sh").args(["-c", "exit 1"]).spawn().unwrap();
        let error = app
            .handoff(replacement, &ready, Duration::from_secs(2))
            .unwrap_err();
        assert!(error.contains("exited before opening"));
        assert!(app.child.as_mut().unwrap().try_wait().unwrap().is_none());
    }

    #[cfg(unix)]
    #[test]
    fn ready_replacement_takes_over_the_old_process() {
        let ready = env::temp_dir().join(format!("gpui-handoff-ready-{}", std::process::id()));
        let mut app = PlaygroundProcess {
            child: Some(Command::new("sleep").arg("30").spawn().unwrap()),
            directory: None,
            generation: 0,
            completed: 0,
        };
        let replacement = Command::new("sleep").arg("30").spawn().unwrap();
        let new = replacement.id();
        fs::write(&ready, "ready").unwrap();
        app.handoff(replacement, &ready, Duration::from_secs(2))
            .unwrap();
        assert_eq!(app.child.as_ref().unwrap().id(), new);
        assert!(app.child.as_mut().unwrap().try_wait().unwrap().is_none());
        assert!(
            !app.has_exited(),
            "intentional replacement must keep the session alive"
        );
        fs::remove_file(ready).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn closing_the_active_preview_cancels_even_a_ready_replacement() {
        let ready = env::temp_dir().join(format!("gpui-handoff-closed-{}", std::process::id()));
        let mut app = PlaygroundProcess::default();
        app.child = Some(Command::new("sh").args(["-c", "exit 0"]).spawn().unwrap());
        app.child.as_mut().unwrap().wait().unwrap();
        assert!(app.has_exited());
        assert!(app.exit_status().unwrap().unwrap().success());
        let replacement = Command::new("sleep").arg("30").spawn().unwrap();
        fs::write(&ready, "ready").unwrap();
        assert_eq!(
            app.handoff(replacement, &ready, Duration::from_secs(2))
                .unwrap_err(),
            "Playground closed"
        );
        assert!(
            app.has_exited(),
            "the closed preview must not be resurrected"
        );
        fs::remove_file(ready).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn closing_during_checks_cancels_the_command() {
        let mut app = PlaygroundProcess::default();
        assert!(
            !app.has_exited(),
            "initial build failure can still be repaired"
        );
        let mut child = Command::new("sh");
        app.child = Some(child.args(["-c", "sleep 0.1; exit 0"]).spawn().unwrap());
        let started = Instant::now();
        let result = Command::new("sh")
            .args(["-c", "sleep 30 & wait"])
            .interruptible_output(&mut || app.has_exited());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::Interrupted);
        assert!(started.elapsed() < Duration::from_secs(5));
    }

    #[cfg(unix)]
    #[test]
    fn closing_the_just_opened_replacement_also_ends_the_session() {
        let ready =
            env::temp_dir().join(format!("gpui-handoff-quick-close-{}", std::process::id()));
        let mut app = PlaygroundProcess::default();
        app.child = Some(Command::new("sleep").arg("30").spawn().unwrap());
        let mut replacement = Command::new("sh").args(["-c", "exit 0"]).spawn().unwrap();
        replacement.wait().unwrap();
        fs::write(&ready, "ready").unwrap();
        app.handoff(replacement, &ready, Duration::from_secs(2))
            .unwrap();
        assert!(app.has_exited());
        assert!(app.exit_status().unwrap().unwrap().success());
        fs::remove_file(ready).unwrap();
    }

    #[test]
    fn saved_progress_is_forward_compatible() {
        assert_eq!(progress_index("complete"), 5);
        assert_eq!(progress_index("complete:05"), 5);
        assert_eq!(progress_index("complete:06"), 6);
        assert_eq!(progress_index("complete:15"), 15);
        assert_eq!(progress_index("complete:16"), 16);
        assert_eq!(progress_index("complete:17"), 17);
        assert_eq!(progress_index("complete:18"), 18);
        assert_eq!(progress_index("complete:19"), 19);
        assert_eq!(progress_index("complete:20"), 20);
        assert_eq!(progress_index("complete:21"), 21);
        assert_eq!(progress_index("complete:22"), 22);
        assert_eq!(progress_index("complete:23"), 23);
        assert_eq!(progress_index("complete:24"), 24);
        assert_eq!(progress_index("complete:25"), 25);
        assert_eq!(progress_index("complete:26"), 26);
        assert_eq!(progress_index("complete:27"), 27);
        assert_eq!(progress_index("complete:28"), 28);
        assert_eq!(progress_index("complete:29"), 29);
        assert_eq!(progress_index("complete:30"), 30);
        assert_eq!(progress_index("complete:31"), 31);
        assert_eq!(progress_index("complete:32"), 32);
        assert_eq!(progress_index("complete:33"), 33);
        assert_eq!(progress_index("complete:34"), 34);
        assert_eq!(progress_index("complete:35"), 35);
        assert_eq!(progress_index("complete:36"), 36);
        assert_eq!(progress_index("complete:37"), 37);
        assert_eq!(progress_index("complete:38"), LESSONS.len());
        assert_eq!(progress_index("04\n"), 3);
        assert_eq!(progress_index("garbage"), 0);
        for index in 0..=LESSONS.len() {
            assert_eq!(progress_index(&progress_value(index)), index);
        }
    }

    #[test]
    fn empty_test_filter_is_never_a_pass() {
        assert_eq!(
            evaluate_output(true, "running 0 tests\ntest result: ok.".into()).state,
            CheckState::BuildError
        );
        assert_eq!(
            evaluate_output(
                true,
                "running 1 test\ntest result: ok. 1 passed; 0 failed; 0 ignored;".into()
            )
            .state,
            CheckState::Passed
        );
        assert_eq!(
            evaluate_output(false, "running 1 test\ntest result: FAILED.".into()).state,
            CheckState::Failed
        );
        assert_eq!(
            evaluate_output(false, "error[E0425]: unknown value".into()).state,
            CheckState::BuildError
        );
    }

    #[test]
    fn ignored_test_is_never_a_pass() {
        assert_eq!(
            evaluate_output(
                true,
                "running 1 test\ntest result: ok. 0 passed; 0 failed; 1 ignored;".into()
            )
            .state,
            CheckState::BuildError
        );
    }

    #[test]
    fn catalog_references_source_instructions_and_tests() {
        for (i, lesson) in LESSONS.iter().enumerate() {
            assert_eq!(lesson.id, format!("{:02}", i + 1));
            assert!(lesson.instructions().contains(lesson.id));
            assert!(!lesson.introduction().is_empty());
            let source = fs::read_to_string(root().join(lesson.file)).unwrap();
            assert!(source.contains(lesson.test.rsplit("::").next().unwrap()));
            assert!(lesson.hints.iter().all(|hint| !hint.is_empty()));
            assert!(!lesson.hints.is_empty());
        }
    }

    #[test]
    fn hints_reveal_one_level_at_a_time() {
        let single = LESSONS.iter().find(|l| l.hints.len() == 1).unwrap();
        assert_eq!(
            single.hint(3, "h again"),
            format!("Hint: {}", single.hints[0])
        );
        let layered = LESSONS.iter().find(|l| l.hints.len() > 1).unwrap();
        let count = layered.hints.len();
        let first = layered.hint(0, "h again");
        assert!(first.starts_with(&format!("Hint 1 of {count}: {}", layered.hints[0])));
        assert!(first.ends_with("h again for a more specific hint."));
        let last = format!("Hint {count} of {count}: {}", layered.hints[count - 1]);
        assert_eq!(layered.hint(count - 1, "h again"), last);
        assert_eq!(layered.hint(count + 4, "h again"), last);
    }

    #[test]
    fn source_instructions_stop_before_code_and_tests() {
        let source =
            "//! 07 — Notify\n//!\n//! A short explanation.\n\nfn code() {}\n//! Not instructions.";
        assert_eq!(
            lessons::instructions(source),
            "07 — Notify\n\nA short explanation."
        );
        assert_eq!(lessons::instructions("fn code() {}"), "");
    }

    #[test]
    fn watcher_detects_edits_additions_and_deletions() {
        let directory = env::temp_dir().join(format!("gpui-lings-watcher-{}", std::process::id()));
        fs::create_dir_all(&directory).unwrap();
        let first = fingerprint(&directory).unwrap();
        fs::write(directory.join("sample.rs"), "one").unwrap();
        let second = fingerprint(&directory).unwrap();
        assert_ne!(first, second);
        fs::write(directory.join("sample.rs"), "two").unwrap();
        assert_ne!(second, fingerprint(&directory).unwrap());
        fs::remove_file(directory.join("sample.rs")).unwrap();
        assert_eq!(first, fingerprint(&directory).unwrap());
        fs::remove_dir(directory).unwrap();
    }

    #[test]
    fn cargo_target_directory_decodes_paths() {
        assert_eq!(
            target_directory(r#"{"target_directory":"/tmp/my target"}"#).as_deref(),
            Some("/tmp/my target")
        );
        assert_eq!(
            target_directory(r#"{"target_directory":"C:\\dev\\target"}"#).as_deref(),
            Some("C:\\dev\\target")
        );
        assert!(target_directory("{}").is_none());
    }
}
