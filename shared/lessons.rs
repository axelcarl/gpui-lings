//! Course order, optional hints, and source instructions shared by both interfaces.
//! Exercise sources are read at runtime, so editing one never rebuilds the guide.

#[derive(Clone, Copy)]
pub enum Verifier {
    Lightweight,
    Native,
}

pub struct Lesson {
    pub id: &'static str,
    pub name: &'static str,
    pub chapter: &'static str,
    pub title: &'static str,
    pub concept: &'static str,
    pub duration: &'static str,
    pub objective: &'static str,
    pub test: &'static str,
    pub file: &'static str,
    /// Optional hints, from a strategy to the exact change. Each request reveals the next.
    pub hints: &'static [&'static str],
    pub verifier: Verifier,
}

pub const LESSONS: [Lesson; 38] = [
    Lesson {
        id: "01",
        name: "basics1",
        chapter: "Foundations",
        title: "Render a greeting",
        concept: "Elements & rendering",
        duration: "2 min",
        objective: "Make the headline say Hello, GPUI!",
        test: "tests::exercise_01",
        file: "exercises/01_basics/basics1.rs",
        hints: &["Look at welcome_text(). Render places its return value in div().child(...)."],
        verifier: Verifier::Lightweight,
    },
    Lesson {
        id: "02",
        name: "basics2",
        chapter: "Foundations",
        title: "Update state on click",
        concept: "State & listeners",
        duration: "3 min",
        objective: "Increase the count by one with every click.",
        test: "tests::exercise_02",
        file: "exercises/01_basics/basics2.rs",
        hints: &[
            "The click handler calls increment(). Check which direction the count moves; keep cx.notify() so GPUI redraws.",
        ],
        verifier: Verifier::Lightweight,
    },
    Lesson {
        id: "03",
        name: "basics3",
        chapter: "Foundations",
        title: "Render derived state",
        concept: "State → interface",
        duration: "3 min",
        objective: "Celebrate when the count reaches three, and beyond.",
        test: "tests::exercise_03",
        file: "exercises/01_basics/basics3.rs",
        hints: &["Test the boundary: what should milestone_text() return at exactly three?"],
        verifier: Verifier::Lightweight,
    },
    Lesson {
        id: "04",
        name: "views1",
        chapter: "Views & layout",
        title: "Compose a row with flex",
        concept: "Flex direction",
        duration: "4 min",
        objective: "Place the two steps side by side on one row.",
        test: "exercises::views::layout::tests::exercise_04",
        file: "exercises/02_views/views1.rs",
        hints: &[
            "In progress_strip(), the two children need the same y-position. Which flex direction achieves that?",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "05",
        name: "views2",
        chapter: "Views & layout",
        title: "Update a child entity",
        concept: "Entities & notification",
        duration: "5 min",
        objective: "Toggle the signal on, then off, with two clicks.",
        test: "exercises::views::entity::tests::exercise_05",
        file: "exercises/02_views/views2.rs",
        hints: &[
            "The click reaches TogglePanel. Make active change on every click; cx.notify() already requests a redraw.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "06",
        name: "views3",
        chapter: "Views & layout",
        title: "Give a layout room to breathe",
        concept: "Spacing & rhythm",
        duration: "4 min",
        objective: "Add exactly 16 pixels of space between the three tiles.",
        test: "exercises::views::spacing::tests::exercise_06",
        file: "exercises/02_views/views3.rs",
        hints: &[
            "The row already flows correctly. Set its gap using px(...) so both spaces measure exactly 16 pixels.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "07",
        name: "contexts1",
        chapter: "Contexts & handlers",
        title: "Tell GPUI what changed",
        concept: "Context<Self> & notify",
        duration: "5 min",
        objective: "Enable the signal and notify GPUI after changing state.",
        test: "exercises::contexts::notify::tests::exercise_07",
        file: "exercises/03_contexts/contexts1.rs",
        hints: &[
            "The context in the click handler belongs to this view. Call its notify method after changing active.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "08",
        name: "contexts2",
        chapter: "Contexts & handlers",
        title: "Connect a handler to its view",
        concept: "cx.listener",
        duration: "5 min",
        objective: "Make each click call record_click once.",
        test: "exercises::contexts::listener::tests::exercise_08",
        file: "exercises/03_contexts/contexts2.rs",
        hints: &[
            "Pass the record_click method to the context's listener adapter, then pass that result to on_click.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "09",
        name: "contexts3",
        chapter: "Contexts & handlers",
        title: "Update the entity, not a copy",
        concept: "Entity::read & update",
        duration: "6 min",
        objective: "Increment the existing child entity on every click.",
        test: "exercises::contexts::update::tests::exercise_09",
        file: "exercises/03_contexts/contexts3.rs",
        hints: &[
            "Call this.score.update(cx, |score, cx| ...). Mutate score inside the closure and notify that inner context.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "10",
        name: "contexts4",
        chapter: "Contexts & handlers",
        title: "Observe a model",
        concept: "cx.observe",
        duration: "6 min",
        objective: "Keep the mirrored reading synchronized with the model.",
        test: "exercises::contexts::observe::tests::exercise_10",
        file: "exercises/03_contexts/contexts4.rs",
        hints: &[
            "The observer receives the entity that changed. Read its value using the callback's context.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "11",
        name: "contexts5",
        chapter: "Contexts & handlers",
        title: "Keep a subscription alive",
        concept: "EventEmitter & Subscription",
        duration: "7 min",
        objective: "Deliver each child signal to its parent.",
        test: "exercises::contexts::events::tests::exercise_11",
        file: "exercises/03_contexts/contexts5.rs",
        hints: &[
            "Store Some(subscription) in _subscription. Dropping the returned Subscription disconnects the callback.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "12",
        name: "interaction1",
        chapter: "Actions & focus",
        title: "Route a keyboard action",
        concept: "Action & key_context",
        duration: "6 min",
        objective: "Focus the shortcut area, then toggle the signal with Ctrl-K.",
        test: "exercises::interaction::actions::tests::exercise_12",
        file: "exercises/04_interaction/interaction1.rs",
        hints: &[
            "The key_context string must satisfy the predicate in KeyBinding::new. Compare the two strings.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "13",
        name: "interaction2",
        chapter: "Actions & focus",
        title: "Move keyboard focus",
        concept: "FocusHandle & Window",
        duration: "6 min",
        objective: "Focus the pad, then make it receive the X key.",
        test: "exercises::interaction::focus::tests::exercise_13",
        file: "exercises/04_interaction/interaction2.rs",
        hints: &["Use the handler's Window to focus this.pad with the supplied context."],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "14",
        name: "lifetimes1",
        chapter: "Lifetimes & async",
        title: "Inspect a weak handle",
        concept: "WeakEntity & ownership",
        duration: "6 min",
        objective: "Inspect the target before and after releasing its owner.",
        test: "exercises::lifetimes::weak::tests::exercise_14",
        file: "exercises/05_lifetimes/lifetimes1.rs",
        hints: &[
            "self.target.upgrade() returns Option<Entity<Record>>. Keep that strong handle local to the inspection.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "15",
        name: "lifetimes2",
        chapter: "Lifetimes & async",
        title: "Keep async work alive",
        concept: "Task & AsyncApp",
        duration: "8 min",
        objective: "Finish loading after one second and cancel unfinished loads.",
        test: "exercises::lifetimes::tasks::tests::exercise_15",
        file: "exercises/05_lifetimes/lifetimes2.rs",
        hints: &[
            "Store Some(task) in self.task. Replacing or taking that handle drops the old task and cancels its work.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "16",
        name: "responsive1",
        chapter: "Responsive views",
        title: "Adapt a layout to window width",
        concept: "Window bounds & responsive layout",
        duration: "6 min",
        objective: "Stack the cards in a narrow window; keep them side by side when wide.",
        test: "exercises::views::responsive::tests::exercise_16",
        file: "exercises/06_responsive/responsive1.rs",
        hints: &[
            "The wide branch already uses flex_row. Which flex direction makes the narrow branch vertical?",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "17",
        name: "responsive2",
        chapter: "Responsive views",
        title: "Keep content usable in a small window",
        concept: "Overflow & scroll handles",
        duration: "6 min",
        objective: "Scroll the list to its last item, by wheel and with Jump to last.",
        test: "exercises::views::scrolling::tests::exercise_17",
        file: "exercises/06_responsive/responsive2.rs",
        hints: &[
            "Two things are missing: an overflow mode that lets the list scroll, and a link between the list and self.scroll, which Jump to last moves.",
            "Replace overflow_hidden() with overflow_y_scroll(), then add .track_scroll(&self.scroll) to the list.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "18",
        name: "responsive3",
        chapter: "Responsive views",
        title: "Keep a disabled control disabled",
        concept: "Disabled state & activation paths",
        duration: "7 min",
        objective: "Keep a disabled control visible but unable to change selection.",
        test: "exercises::views::states::tests::exercise_18",
        file: "exercises/06_responsive/responsive3.rs",
        hints: &[
            "Both mouse and keyboard use toggle_selection. Guard the state change there when disabled is true.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "19",
        name: "responsive4",
        chapter: "Responsive views",
        title: "Route a pointer gesture",
        concept: "Mouse down, move & release",
        duration: "7 min",
        objective: "Drag the value, then release outside without leaving it active.",
        test: "exercises::views::drag::tests::exercise_19",
        file: "exercises/06_responsive/responsive4.rs",
        hints: &[
            "The inside-release callback clears drag_start. The outside-release callback needs the same cleanup.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "20",
        name: "quiz1",
        chapter: "Responsive views",
        title: "Quiz 1: compact inspector",
        concept: "Responsive layout, scrolling & input",
        duration: "20 min",
        objective: "Find and fix four reported bugs in a compact inspector.",
        test: "exercises::views::inspector::tests::exercise_20",
        file: "exercises/quizzes/quiz1.rs",
        hints: &[
            "Fix one symptom at a time, in the order the check reports them. Change the preview's width and height separately.",
            "The fixes reuse lessons 16 (what a breakpoint measures), 13 (where key events go), 18 (guarding every activation path), and 17 (what lets a region scroll), in that order.",
            "Compare width, not height. Move on_key_down from the detail to the list, which tracks focus. Guard next() with !self.disabled. Give the list the detail's fixed height.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "21",
        name: "deeper1",
        chapter: "Deeper contexts & input",
        title: "Move between focus regions",
        concept: "Focus handles & overlays",
        duration: "7 min",
        objective: "Return keyboard focus to Open after closing the overlay.",
        test: "exercises::interaction::regions::tests::exercise_21",
        file: "exercises/07_deeper/deeper1.rs",
        hints: &[
            "Close hides the overlay; use Window::focus with the stored trigger handle before notifying.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "22",
        name: "deeper2",
        chapter: "Deeper contexts & input",
        title: "Route a nested action",
        concept: "Action propagation",
        duration: "7 min",
        objective: "Handle Ctrl-R once in the focused child or fall back to its parent.",
        test: "exercises::interaction::propagation::tests::exercise_22",
        file: "exercises/07_deeper/deeper2.rs",
        hints: &[
            "Action handlers consume by default. What should the child call when it does not handle the action?",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "23",
        name: "deeper3",
        chapter: "Deeper contexts & input",
        title: "Defer a follow-up update",
        concept: "cx.defer & borrow boundaries",
        duration: "7 min",
        objective: "Record Settled after Queued at the end of the current update.",
        test: "exercises::contexts::deferred::tests::exercise_23",
        file: "exercises/07_deeper/deeper3.rs",
        hints: &[
            "queue() still holds this view's mutable borrow. Schedule the second step for after it ends, and reach the view again through a handle that may have expired.",
            "Capture cx.weak_entity(), then call cx.defer and update the entity inside that closure.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "24",
        name: "quiz2",
        chapter: "Deeper contexts & input",
        title: "Quiz 2: command menu",
        concept: "Actions, focus & keyboard routing",
        duration: "15 min",
        objective: "Find and fix three reported bugs in a keyboard command menu.",
        test: "exercises::interaction::menu::tests::exercise_24",
        file: "exercises/quizzes/quiz2.rs",
        hints: &[
            "Fix one symptom at a time, in the order the check reports them. Use only the keyboard in the preview, and watch where focus goes.",
            "The fixes reuse lessons 07 (telling GPUI a view changed), 12 (where a key context must sit for a binding to match), and 21 (returning focus when an overlay closes), in that order.",
            "Notify after the arrow keys change selected. Move key_context(\"CommandMenuDemo\") from the menu to the view's root. Focus the launcher, not the menu, in cancel().",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "25",
        name: "async1",
        chapter: "Async data & failure paths",
        title: "Run work in the background",
        concept: "BackgroundExecutor & Task",
        duration: "8 min",
        objective: "Show the computed result while keeping the UI responsive.",
        test: "exercises::lifetimes::background::tests::exercise_25",
        file: "exercises/08_async/async1.rs",
        hints: &["The foreground task already awaits work. Use its result when updating the view."],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "26",
        name: "async2",
        chapter: "Async data & failure paths",
        title: "Show errors and retry",
        concept: "Async UI states & retry",
        duration: "8 min",
        objective: "Clear an error during retry, then show the returned data.",
        test: "exercises::lifetimes::retry::tests::exercise_26",
        file: "exercises/08_async/async2.rs",
        hints: &["retry() must set Loading and notify before starting the second request."],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "27",
        name: "async3",
        chapter: "Async data & failure paths",
        title: "Ignore stale results",
        concept: "Out-of-order async completion",
        duration: "8 min",
        objective: "Keep the newer selection visible after an older request completes.",
        test: "exercises::lifetimes::stale::tests::exercise_27",
        file: "exercises/08_async/async3.rs",
        hints: &["Compare the completion's label with selected inside the entity update."],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "28",
        name: "quiz3",
        chapter: "Async data & failure paths",
        title: "Quiz 3: searchable results",
        concept: "Text input, background work & keyboard selection",
        duration: "20 min",
        objective: "Find and fix three reported bugs in an async search.",
        test: "exercises::lifetimes::search::tests::exercise_28",
        file: "exercises/quizzes/quiz3.rs",
        hints: &[
            "Fix one symptom at a time, in the order the check reports them. For the async ones, note which request finishes last.",
            "The fixes reuse lessons 09 (which entity you hold), 27 (recognizing an older completion), and 26 (what Retry must do), in that order.",
            "Store and render the InputState you subscribe to. Ignore a completion whose generation differs from this.generation. Remove the early return for a repeated query so Retry searches again.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "29",
        name: "application1",
        chapter: "Application structure",
        title: "Start a standalone GPUI app",
        concept: "Application, init & root window",
        duration: "9 min",
        objective: "Open a new window with WorkspaceRoot as its root view.",
        test: "exercises::app::startup::tests::exercise_29",
        file: "exercises/09_application/application1.rs",
        hints: &[
            "GPUI Kit opens windows for you. Its open_window builds the root view in a callback and returns both the window and that root.",
            "Call gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| cx.new(|_| WorkspaceRoot)). It returns a Result of (window, root): use .ok() and .map to return the root as an Option.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "30",
        name: "application2",
        chapter: "Application structure",
        title: "Share application state",
        concept: "Global & observe_global",
        duration: "8 min",
        objective: "Update one global setting and show it in two child views.",
        test: "exercises::app::shared::tests::exercise_30",
        file: "exercises/09_application/application2.rs",
        hints: &["Store the Subscription returned by cx.observe_global in SpacingSummary."],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "31",
        name: "application3",
        chapter: "Application structure",
        title: "Save and restore a setting",
        concept: "Persistence & fallback",
        duration: "8 min",
        objective: "Restore the saved setting and recover from invalid data.",
        test: "exercises::app::persistence::tests::exercise_31",
        file: "exercises/09_application/application3.rs",
        hints: &[
            "Read the file with std::fs::read_to_string. Only trimmed `compact` should map to true.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "32",
        name: "application4",
        chapter: "Application structure",
        title: "Open a second window",
        concept: "Windows, shared entities & cleanup",
        duration: "10 min",
        objective: "Share a count across windows and reopen after closing detail.",
        test: "exercises::app::windows::tests::exercise_32",
        file: "exercises/09_application/application4.rs",
        hints: &["Keep the Subscription from cx.on_window_closed in _close_subscription."],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "33",
        name: "application5",
        chapter: "Application structure",
        title: "Follow appearance changes",
        concept: "Window appearance & semantic colors",
        duration: "8 min",
        objective: "Keep text legible in light and dark appearance.",
        test: "exercises::app::appearance::tests::exercise_33",
        file: "exercises/09_application/application5.rs",
        hints: &["The palette already has a foreground field for each appearance."],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "34",
        name: "quality1",
        chapter: "Ship-quality GPUI",
        title: "Expose an accessible control",
        concept: "Role, name, state & keyboard activation",
        duration: "9 min",
        objective: "Give a switch an accessible name without losing its behavior.",
        test: "exercises::app::accessibility::tests::exercise_34",
        file: "exercises/10_quality/quality1.rs",
        hints: &[
            "Switch already supplies its role and toggled state. Call accessibility_label on it.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "35",
        name: "quality2",
        chapter: "Ship-quality GPUI",
        title: "Test behavior through GPUI",
        concept: "Headless interaction & simulated time",
        duration: "9 min",
        objective: "Write a GPUI test that clicks Load and waits on simulated time.",
        test: "exercises::app::behavior_test::tests::exercise_35",
        file: "exercises/10_quality/quality2.rs",
        hints: &[
            "The window context can send a click to a point. The executor can move simulated time forward; afterwards, run the work that became ready.",
            "Use window.simulate_click(load.center(), Modifiers::default()), then window.executor().advance_clock(Duration::from_secs(1)) and window.run_until_parked().",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "36",
        name: "quality3",
        chapter: "Ship-quality GPUI",
        title: "Render a large collection efficiently",
        concept: "Virtual list & visible range",
        duration: "9 min",
        objective: "Build only visible rows while preserving selection and scroll.",
        test: "exercises::app::large_list::tests::exercise_36",
        file: "exercises/10_quality/quality3.rs",
        hints: &[
            "The virtual-list callback gives rows() a Range. Iterate over that range directly instead of constructing every row and skipping most of them.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "37",
        name: "quality4",
        chapter: "Ship-quality GPUI",
        title: "Compose a reusable component",
        concept: "Controlled input, output & theme",
        duration: "9 min",
        objective: "Keep two switches independent and the disabled one inert.",
        test: "exercises::app::reusable::tests::exercise_37",
        file: "exercises/10_quality/quality4.rs",
        hints: &[
            "The wrapper receives checked but gives Switch::checked a constant false. Pass the input through.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "38",
        name: "quiz4",
        chapter: "Ship-quality GPUI",
        title: "Quiz 4 (capstone): small native workspace",
        concept: "Entities, layout, actions, async & persistence",
        duration: "25 min",
        objective: "Find and fix five reported bugs in a two-pane workspace.",
        test: "exercises::app::capstone::tests::exercise_38",
        file: "exercises/quizzes/quiz4.rs",
        hints: &[
            "Fix one symptom at a time, in the order the check reports them. Reproduce each one in the preview before changing code.",
            "The fixes reuse lessons 15 (task lifetime), 26 (retry state), 16 (breakpoints), 09 (the entity, not a copy), and 13 (attaching focus), in that order.",
            "Store request()'s task in self._task. Set Loading and notify in retry(). Flip the width comparison. Read the selected index from self.model in save(). Add track_focus(&self.focus) to the workspace.",
        ],
        verifier: Verifier::Native,
    },
];

/// Store the last completed ID so adding a lesson doesn't skip new material.
pub fn progress_index(saved: &str) -> usize {
    let saved = saved.trim();
    if saved == "complete" {
        // The original five-lesson release used this sentinel.
        return 5.min(LESSONS.len());
    }
    if let Some(id) = saved.strip_prefix("complete:") {
        return LESSONS
            .iter()
            .position(|lesson| lesson.id == id)
            .map_or(0, |i| i + 1);
    }
    LESSONS
        .iter()
        .position(|lesson| lesson.id == saved)
        .unwrap_or(0)
}

pub fn progress_value(index: usize) -> String {
    LESSONS.get(index).map_or_else(
        || format!("complete:{}", LESSONS.last().unwrap().id),
        |lesson| lesson.id.to_owned(),
    )
}

/// Read only the leading module documentation, never the learner's code/tests.
pub fn instructions(source: &str) -> String {
    source
        .lines()
        .take_while(|line| line.trim_start().starts_with("//!"))
        .map(|line| {
            let text = line.trim_start().strip_prefix("//!").unwrap();
            text.strip_prefix(' ').unwrap_or(text)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

impl Lesson {
    /// Hint `level` (from zero), repeating the most specific one. `more` says how
    /// to reveal the next level while one remains.
    pub fn hint(&self, level: usize, more: &str) -> String {
        let count = self.hints.len();
        let level = level.min(count - 1);
        let text = self.hints[level];
        if count == 1 {
            format!("Hint: {text}")
        } else if level + 1 < count {
            format!(
                "Hint {} of {count}: {text}\n{more} for a more specific hint.",
                level + 1
            )
        } else {
            format!("Hint {count} of {count}: {text}")
        }
    }
}
