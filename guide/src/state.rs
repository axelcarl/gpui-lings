//! Rustlings-style named navigation and independent completion records.
use crate::lessons::LESSONS;
use std::{collections::BTreeSet, fs, io, path::Path};

/// Version 2 numbers the 42-lesson course. Its names can mean other lessons
/// in version 1 (`quiz1`, `lifetimes2`), so renames apply to version 1 only.
const HEADER: &str = "# GPUI Lings state v2 — managed by the guide";
const HEADER_V1: &str = "# GPUI Lings state — managed by the guide";

/// The 38-lesson course of version 1, in order. The older, ID-based
/// `.gpui-lings-progress` file counts positions in this list.
const V1_ORDER: [&str; 38] = [
    "basics1",
    "basics2",
    "basics3",
    "views1",
    "views2",
    "views3",
    "contexts1",
    "contexts2",
    "contexts3",
    "contexts4",
    "contexts5",
    "interaction1",
    "interaction2",
    "lifetimes1",
    "lifetimes2",
    "responsive1",
    "responsive2",
    "responsive3",
    "responsive4",
    "quiz1",
    "deeper1",
    "deeper2",
    "deeper3",
    "quiz2",
    "async1",
    "async2",
    "async3",
    "quiz3",
    "application1",
    "application2",
    "application3",
    "application4",
    "application5",
    "quality1",
    "quality2",
    "quality3",
    "quality4",
    "quiz4",
];

/// Version 1 names whose exercise moved, including names from before the
/// quizzes got their own directory. Names not listed are unchanged.
const V1_RENAMED: [(&str, &str); 18] = [
    ("interaction1", "keyboard4"),
    ("interaction2", "keyboard1"),
    ("lifetimes2", "lifetimes3"),
    ("deeper3", "lifetimes2"),
    ("responsive1", "layout_states1"),
    ("responsive2", "layout_states2"),
    ("responsive3", "layout_states3"),
    ("responsive4", "layout_states4"),
    ("deeper1", "dispatch1"),
    ("deeper2", "dispatch2"),
    ("quiz1", "quiz3"),
    ("quiz2", "quiz4"),
    ("quiz3", "quiz5"),
    ("quiz4", "quiz6"),
    ("responsive5", "quiz3"),
    ("deeper4", "quiz4"),
    ("async4", "quiz5"),
    ("quality5", "quiz6"),
];

fn v1_name(name: &str) -> &str {
    V1_RENAMED
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

    /// The ID-based progress file: `NN` is the current lesson, `complete:NN`
    /// the last one completed, and plain `complete` the first five.
    fn legacy(text: &str) -> Self {
        let text = text.trim();
        let position = |id: &str| id.parse::<usize>().ok().filter(|n| (1..=38).contains(n));
        let completed = match text.strip_prefix("complete") {
            Some("") => 5,
            Some(rest) => rest.strip_prefix(':').and_then(position).unwrap_or(0),
            None => position(text).map_or(0, |n| n - 1),
        };
        let current = V1_ORDER.get(completed).copied().unwrap_or("complete");
        Self::named(
            v1_name(current),
            V1_ORDER[..completed].iter().map(|name| v1_name(name)),
        )
    }

    fn parse(text: &str) -> Self {
        let mut lines = text.lines();
        let rename: fn(&str) -> &str = match lines.next() {
            Some(HEADER) => |name| name,
            Some(HEADER_V1) => v1_name,
            _ => return Self::legacy(""),
        };
        if lines.next() != Some("") {
            return Self::legacy("");
        }
        let current = rename(lines.next().unwrap_or_default());
        if lines.next() != Some("") {
            return Self::legacy("");
        }
        Self::named(current, lines.map(rename))
    }

    fn named<'a>(current: &str, done: impl Iterator<Item = &'a str>) -> Self {
        let done = done
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

    fn index(name: &str) -> usize {
        LESSONS.iter().position(|l| l.name == name).unwrap()
    }

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
        assert_eq!(state.completed(), 38);
        // Version 1 had 38 lessons; the four added since are still pending.
        assert_eq!(LESSONS[state.current].name, "quiz1");
        state.current = LESSONS.len() - 1;
        state.set_done(index("quiz1"), true);
        state.set_done(index("keyboard2"), true);
        state.set_done(index("keyboard3"), true);
        state.set_done(index("quiz2"), true);
        assert_eq!(state.next_pending(), LESSONS.len());
        state.set_done(2, false);
        assert_eq!(state.next_pending(), 2);
        state.current = LESSONS.len();
        assert_eq!(LessonState::parse(&state.encode()).current, 2);
    }

    #[test]
    fn version_1_state_follows_moved_and_renamed_exercises() {
        let text = format!(
            "{HEADER_V1}\n\ninteraction1\n\nbasics1\nlifetimes2\ndeeper3\nquiz1\nresponsive5\nremoved\n"
        );
        let state = LessonState::parse(&text);
        assert_eq!(LESSONS[state.current].name, "keyboard4");
        for name in ["basics1", "lifetimes3", "lifetimes2", "quiz3"] {
            assert!(state.is_done(name), "{name} should stay done");
        }
        assert!(!state.is_done("quiz1"), "the new quiz 1 is new material");
        assert_eq!(state.completed(), 4);
        // Saving writes version 2, where names are taken as they are.
        let saved = LessonState::parse(&state.encode());
        assert_eq!(saved, state);
        assert!(state.encode().starts_with(HEADER));
    }

    #[test]
    fn migration_and_unknown_or_duplicate_records_are_safe() {
        assert_eq!(LessonState::legacy("08").current, 7);
        assert_eq!(LessonState::legacy("08").completed(), 7);
        assert_eq!(LessonState::legacy("complete").current, 5);
        // Version 1's lesson 13 was focus, which is lesson 13 again.
        let focus = LessonState::legacy("13");
        assert_eq!(LESSONS[focus.current].name, "keyboard1");
        assert!(focus.is_done("keyboard4"), "old 12 is done");
        assert_eq!(LessonState::legacy("garbage"), LessonState::legacy(""));
        let text = format!("{HEADER}\n\ncontexts2\n\nbasics1\nbasics1\nremoved_lesson\n");
        let state = LessonState::parse(&text);
        assert_eq!(state.current, 7);
        assert_eq!(state.completed(), 1);
        assert_eq!(LessonState::parse("truncated").completed(), 0);
        let text = format!("{HEADER}\n\nquiz1\n\nquiz2\n");
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
