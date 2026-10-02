//! Rustlings-style named navigation and independent completion records.
use crate::lessons::{LESSONS, progress_index};
use std::{collections::BTreeSet, fs, io, path::Path};

const HEADER: &str = "# GPUI Lings state — managed by the guide";

/// Lessons that were renamed, so saved progress keeps finding them.
const RENAMED: [(&str, &str); 4] = [
    ("responsive5", "quiz1"),
    ("deeper4", "quiz2"),
    ("async4", "quiz3"),
    ("quality5", "quiz4"),
];

fn current_name(name: &str) -> &str {
    RENAMED
        .iter()
        .find(|(old, _)| *old == name)
        .map_or(name, |(_, new)| new)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LessonState {
    pub current: usize,
    done: BTreeSet<String>,
}

impl LessonState {
    pub fn load(root: &Path) -> Self {
        match fs::read_to_string(root.join(".gpui-lings-state.txt")) {
            Ok(text) => Self::parse(&text),
            Err(_) => Self::legacy(
                &fs::read_to_string(root.join(".gpui-lings-progress")).unwrap_or_default(),
            ),
        }
    }

    fn legacy(text: &str) -> Self {
        let current = progress_index(text);
        Self {
            current,
            done: LESSONS[..current].iter().map(|l| l.name.into()).collect(),
        }
    }

    fn parse(text: &str) -> Self {
        let mut lines = text.lines();
        if lines.next() != Some(HEADER) || lines.next() != Some("") {
            return Self::legacy("");
        }
        let current = current_name(lines.next().unwrap_or_default());
        if lines.next() != Some("") {
            return Self::legacy("");
        }
        let done = lines
            .map(current_name)
            .filter(|name| LESSONS.iter().any(|lesson| lesson.name == *name))
            .map(str::to_owned)
            .collect();
        let mut state = Self { current: 0, done };
        // A completed older course resumes at newly added, pending material.
        state.current = LESSONS
            .iter()
            .position(|l| l.name == current)
            .unwrap_or_else(|| {
                LESSONS
                    .iter()
                    .position(|l| !state.done.contains(l.name))
                    .unwrap_or(LESSONS.len())
            });
        state
    }

    fn encode(&self) -> String {
        let current = LESSONS.get(self.current).map_or("complete", |l| l.name);
        let mut text = format!("{HEADER}\n\n{current}\n\n");
        for lesson in &LESSONS {
            if self.is_done(lesson.name) {
                text.push_str(lesson.name);
                text.push('\n');
            }
        }
        text
    }

    pub fn save(&self, root: &Path) -> io::Result<()> {
        let temporary = root.join(".gpui-lings-state.txt.tmp");
        fs::write(&temporary, self.encode())?;
        fs::rename(temporary, root.join(".gpui-lings-state.txt"))
    }

    pub fn is_done(&self, name: &str) -> bool {
        self.done.contains(name)
    }

    pub fn set_done(&mut self, index: usize, done: bool) {
        let name = LESSONS[index].name;
        if done {
            self.done.insert(name.into());
        } else {
            self.done.remove(name);
        }
    }

    pub fn completed(&self) -> usize {
        self.done.len()
    }

    pub fn next_pending(&self) -> usize {
        ((self.current + 1).min(LESSONS.len())..LESSONS.len())
            .chain(0..self.current.min(LESSONS.len()))
            .find(|&i| !self.is_done(LESSONS[i].name))
            .unwrap_or(LESSONS.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_state_keeps_navigation_separate_from_completion() {
        let mut state = LessonState::legacy("");
        state.set_done(0, true);
        state.set_done(3, true);
        state.current = 2;
        assert_eq!(LessonState::parse(&state.encode()), state);
        state.current = 0;
        assert_eq!(state.completed(), 2);
        assert_eq!(state.next_pending(), 1);
        state.current = 2;
        assert_eq!(state.next_pending(), 4);
        state.set_done(3, false);
        assert_eq!(state.completed(), 1);
    }

    #[test]
    fn next_pending_wraps_and_complete_state_finds_added_lessons() {
        let mut state = LessonState::legacy("complete:38");
        state.current = 37;
        assert_eq!(state.next_pending(), LESSONS.len());
        state.set_done(2, false);
        assert_eq!(state.next_pending(), 2);
        state.current = LESSONS.len();
        assert_eq!(LessonState::parse(&state.encode()).current, 2);
    }

    #[test]
    fn migration_and_unknown_or_duplicate_records_are_safe() {
        assert_eq!(LessonState::legacy("08").current, 7);
        assert_eq!(LessonState::legacy("08").completed(), 7);
        assert_eq!(LessonState::legacy("complete").current, 5);
        let text = format!("{HEADER}\n\ncontexts2\n\nbasics1\nbasics1\nremoved_lesson\n");
        let state = LessonState::parse(&text);
        assert_eq!(state.current, 7);
        assert_eq!(state.completed(), 1);
        assert_eq!(LessonState::parse("truncated").completed(), 0);
        let text = format!("{HEADER}\n\nresponsive5\n\ndeeper4\n");
        let state = LessonState::parse(&text);
        assert_eq!(LESSONS[state.current].name, "quiz1");
        assert!(state.is_done("quiz2"));
    }

    #[test]
    fn atomic_save_round_trips_without_changing_legacy_file() {
        let root = std::env::temp_dir().join(format!("gpui-state-test-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join(".gpui-lings-progress"), "08\n").unwrap();
        let mut state = LessonState::load(&root);
        state.set_done(7, true);
        state.current = 0;
        state.save(&root).unwrap();
        assert_eq!(LessonState::load(&root), state);
        assert_eq!(
            fs::read_to_string(root.join(".gpui-lings-progress")).unwrap(),
            "08\n"
        );
        fs::remove_dir_all(root).unwrap();
    }
}
