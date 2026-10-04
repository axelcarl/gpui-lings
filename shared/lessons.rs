//! Course order and optional hints shared by both interfaces. Each exercise file
//! holds its own introduction and TODOs; chapter READMEs hold the background.

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

pub const LESSONS: [Lesson; 42] = [
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
        hints: &[
            "The headline shows whatever `welcome_text` returns, so change the string it \
             returns. The check wants exactly `Hello, GPUI!`, comma and exclamation mark \
             included.\n\
             \n\
             Once it passes, try some other text and watch the preview change. Put it back \
             before you move on.\n\
             \n\
             Read more about views and elements in GPUI Kit's Getting Started guide:\n\
             https://gpui-kit.com/docs/getting-started/#add-a-view",
        ],
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
        test: "exercises::basics2::tests::exercise_02",
        file: "exercises/01_basics/basics2.rs",
        hints: &[
            "`saturating_sub` has a twin that adds instead, or you can write `this.count += \
             1`.\n\
             \n\
             Leave `cx.notify()` where it is. Lesson 07 shows what happens without it.\n\
             \n\
             Read more about views in the Render guide:\n\
             https://gpui-kit.com/docs/render/#the-view-owns-state-the-tree-describes-this-pass",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "03",
        name: "basics3",
        chapter: "Foundations",
        title: "Render derived state",
        concept: "State → interface",
        duration: "3 min",
        objective: "Celebrate when the count reaches three, and beyond.",
        test: "exercises::basics3::tests::exercise_03",
        file: "exercises/01_basics/basics3.rs",
        hints: &[
            "Which comparison is true for 3 and 4, but false for 2? The check clicks four \
             times and reads the message after each click.\n\
             \n\
             Read more about deriving the tree from state in the Render guide:\n\
             https://gpui-kit.com/docs/render/#render-builds-values-handlers-perform-work",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "04",
        name: "views1",
        chapter: "Views & layout",
        title: "Compose a row with flex",
        concept: "Flex direction",
        duration: "4 min",
        objective: "Place the two steps side by side on one row.",
        test: "exercises::views1::tests::exercise_04",
        file: "exercises/02_views/views1.rs",
        hints: &[
            "Flexbox has two directions: row and column. The check wants tile 02 to the right \
             of tile 01, at the same height.\n\
             \n\
             Read more about GPUI's layout methods in the Style guide:\n\
             https://gpui-kit.com/docs/style/#flexbox-and-grid",
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
        test: "exercises::views2::tests::exercise_05",
        file: "exercises/02_views/views2.rs",
        hints: &[
            "To flip a `bool`, set it to its opposite. Rust's `!` operator does that: `!true` \
             is `false`.\n\
             \n\
             The listener already calls `cx.notify()`, so the panel redraws after your change.\n\
             \n\
             Read more about views and entities in the Entity guide:\n\
             https://gpui-kit.com/docs/entity/#data-model-or-persistent-view",
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
        test: "exercises::views3::tests::exercise_06",
        file: "exercises/02_views/views3.rs",
        hints: &[
            "Only the parent's gap needs to change. `px(16.0)` is 16 pixels.\n\
             \n\
             Read more about gaps, padding and sizes in the Style guide:\n\
             https://gpui-kit.com/docs/style/#space-and-size",
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
        test: "exercises::contexts1::tests::exercise_07",
        file: "exercises/03_contexts/contexts1.rs",
        hints: &[
            "Inside the listener, `cx` is this panel's `Context<NotifyPanel>`. Call its \
             `notify` method after changing `active`.\n\
             \n\
             Read more about when to notify in the Render guide:\n\
             https://gpui-kit.com/docs/render/#notify-after-a-meaningful-state-change",
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
        test: "exercises::contexts2::tests::exercise_08",
        file: "exercises/03_contexts/contexts2.rs",
        hints: &[
            "`cx.listener` accepts a method as well as a closure. Pass the method itself, \
             without calling it, the way you'd pass a function to `map`: `Self::record_click`.\n\
             \n\
             Hover `on_click` in your editor to see the callback type it expects, and compare \
             it with the signature of `record_click`.\n\
             \n\
             Read more in the Context guide:\n\
             https://gpui-kit.com/docs/context/#a-callback-that-needs-its-view",
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
        test: "exercises::contexts3::tests::exercise_09",
        file: "exercises/03_contexts/contexts3.rs",
        hints: &[
            "`update` hands your closure the real `Score` and the score's own context. Inside \
             it, add one to `score.value` and call `cx.notify()` on that inner `cx`, since \
             it's the score that changed.\n\
             \n\
             Once that works, the `read(cx).clone()` line has nothing left to do, so remove \
             it.\n\
             \n\
             Read more in the Entity guide:\n\
             https://gpui-kit.com/docs/entity/#update-state",
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
        test: "exercises::contexts4::tests::exercise_10",
        file: "exercises/03_contexts/contexts4.rs",
        hints: &[
            "The observer's second argument, `reading`, is the model's `Entity<Reading>`. Read \
             it the same way as in lesson 09: `reading.read(cx)` gives you the `Reading`.\n\
             \n\
             The check also changes the model without clicking, so copy the model's value \
             rather than counting clicks.\n\
             \n\
             Read more about observing in the Entity guide:\n\
             https://gpui-kit.com/docs/entity/#observe-changes-and-subscribe-to-events",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "11",
        name: "contexts5",
        chapter: "Contexts & handlers",
        title: "Keep subscriptions alive",
        concept: "EventEmitter & Subscription",
        duration: "7 min",
        objective: "Deliver both kinds of child event to the parent.",
        test: "exercises::contexts5::tests::exercise_11",
        file: "exercises/03_contexts/contexts5.rs",
        hints: &[
            "For `Cleared`, copy the `Signal` subscription and change its event parameter to \
             `_: &Cleared`. Its callback sets `received` to 0 and notifies.\n\
             \n\
             Then keep both subscriptions in the panel. `vec![a, b]` builds a `Vec` from two \
             values.\n\
             \n\
             Read more about subscriptions in the Event guide:\n\
             https://gpui-kit.com/docs/event/#subscription-lifetime-and-ownership",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "12",
        name: "quiz1",
        chapter: "Contexts & handlers",
        title: "Quiz 1: tally counter",
        concept: "Render, layout & listeners",
        duration: "15 min",
        objective: "Build a view with a count, a Tap button and a Reset button.",
        test: "exercises::quiz1::tests::exercise_12",
        file: "exercises/quizzes/quiz1.rs",
        hints: &[
            "Save early and often: the check tells you the first thing it can't find. Build \
             from the outside in: a column, then the label, then a row that holds both \
             buttons.",
            "You'll reuse lessons 01 (`div().child(...)`), 04 (rows and columns), 02 (changing \
             state in a listener) and 07 (notifying after the change).",
            "A column `div().flex().flex_col()` with two children: a `div()` named with \
             `.debug_selector(|| format!(\"tally-{}\", self.count))` that shows `format!(\"{} \
             taps\", self.count)`, and a `div().flex()` row holding `button(\"tally-tap\", \
             \"Tap\", true)` and `button(\"tally-clear\", \"Clear\", false)`, each with its \
             own `debug_selector` and an `on_click(cx.listener(...))` that changes \
             `this.count` and calls `cx.notify()`.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "13",
        name: "keyboard1",
        chapter: "Keyboard input",
        title: "Move keyboard focus",
        concept: "FocusHandle & Window",
        duration: "6 min",
        objective: "Focus the pad, then make it receive the X key.",
        test: "exercises::keyboard1::tests::exercise_13",
        file: "exercises/04_keyboard/keyboard1.rs",
        hints: &[
            "The pad's handle is `this.pad`. Pass a reference to it to `window.focus`, along \
             with `cx`.\n\
             \n\
             Creating, attaching and moving focus are separate steps. The Focus guide lists \
             all of them:\n\
             https://gpui-kit.com/docs/focus/#four-separate-operations",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "14",
        name: "keyboard2",
        chapter: "Keyboard input",
        title: "Handle a key",
        concept: "on_key_down & KeyDownEvent",
        duration: "6 min",
        objective: "Move the stepper's value with the left and right arrow keys.",
        test: "exercises::keyboard2::tests::exercise_14",
        file: "exercises/04_keyboard/keyboard2.rs",
        hints: &[
            "`match` on `event.keystroke.key.as_str()`, with an arm for \"left\", one for \
             \"right\", and a `_` arm for every other key. `saturating_sub` stops at zero, as \
             in lesson 02.\n\
             \n\
             Notify only after the value has changed, so other keys don't redraw anything.\n\
             \n\
             Read more about keyboard events in the Event guide:\n\
             https://gpui-kit.com/docs/event/#pointer-and-keyboard-input-are-also-events",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "15",
        name: "keyboard3",
        chapter: "Keyboard input",
        title: "Bind a key to an action",
        concept: "actions!, KeyBinding & on_action",
        duration: "6 min",
        objective: "Make Ctrl-L run the same Like command as the button.",
        test: "exercises::keyboard3::tests::exercise_15",
        file: "exercises/04_keyboard/keyboard3.rs",
        hints: &["`on_action` takes a listener, just like `on_click` does: \
             `cx.listener(Self::like)`. Add it to the panel's element, where the TODO is.\n\
             \n\
             Read more about commands with several entry points in the Action guide:\n\
             https://gpui-kit.com/docs/action/#one-command-several-entry-points"],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "16",
        name: "keyboard4",
        chapter: "Keyboard input",
        title: "Scope a shortcut",
        concept: "Key contexts",
        duration: "6 min",
        objective: "Focus the shortcut area, then toggle the signal with Ctrl-K.",
        test: "exercises::keyboard4::tests::exercise_16",
        file: "exercises/04_keyboard/keyboard4.rs",
        hints: &[
            "Compare the last argument of `KeyBinding::new` in `new` with the string passed to \
             `key_context` in `render`. They need to match.\n\
             \n\
             Read more about key contexts in the KeyBinding guide:\n\
             https://gpui-kit.com/docs/keybinding/#declare-key-contexts",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "17",
        name: "lifetimes1",
        chapter: "Lifetimes & async",
        title: "Inspect a weak handle",
        concept: "WeakEntity & ownership",
        duration: "6 min",
        objective: "Inspect the target before and after releasing its owner.",
        test: "exercises::lifetimes1::tests::exercise_17",
        file: "exercises/05_lifetimes/lifetimes1.rs",
        hints: &[
            "`self.target.upgrade()` returns an `Option<Entity<Record>>`, so the `if \
             record.is_some()` below can stay as it is.\n\
             \n\
             Keep the upgraded handle in a local variable. If you stored it in the panel, the \
             panel would keep the record alive, and Release target couldn't release it.\n\
             \n\
             Read more in the Entity guide:\n\
             https://gpui-kit.com/docs/entity/#use-a-weakentity-for-back-references-and-callbacks",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "18",
        name: "lifetimes2",
        chapter: "Lifetimes & async",
        title: "Defer a follow-up update",
        concept: "cx.defer & borrow boundaries",
        duration: "7 min",
        objective: "Record Settled after Queued at the end of the current update.",
        test: "exercises::lifetimes2::tests::exercise_18",
        file: "exercises/05_lifetimes/lifetimes2.rs",
        hints: &[
            "`queue` still holds this view's mutable borrow, so the second step has to wait \
             until `queue` returns. `cx.defer` runs a closure at that point, and a weak handle \
             gets you back to the view from there.\n\
             \n\
             Read more in the Context guide:\n\
             https://gpui-kit.com/docs/context/#defer-until-the-current-update-ends",
            "Before the call, take `let weak = cx.weak_entity();`. Inside `cx.defer(move |cx| \
             ...)`, call `weak.update(cx, |this, cx| ...)`, push \"Settled\" and notify. \
             `update` returns a `Result`, so write `let _ = weak.update(...)`.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "19",
        name: "lifetimes3",
        chapter: "Lifetimes & async",
        title: "Keep async work alive",
        concept: "Task & AsyncApp",
        duration: "8 min",
        objective: "Finish loading after one second and cancel unfinished loads.",
        test: "exercises::lifetimes3::tests::exercise_19",
        file: "exercises/05_lifetimes/lifetimes3.rs",
        hints: &[
            "The panel already has a field for it: `task: Option<Task<()>>`. It's the same \
             move as keeping the subscription in lesson 11.\n\
             \n\
             Read more about when to keep, await or detach a task in the Task guide:\n\
             https://gpui-kit.com/docs/task/#keep-await-or-detach",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "20",
        name: "quiz2",
        chapter: "Lifetimes & async",
        title: "Quiz 2: inbox",
        concept: "Weak handles, observe, subscribe & tasks",
        duration: "15 min",
        objective: "Find and fix four reported bugs in a small inbox.",
        test: "exercises::quiz2::tests::exercise_20",
        file: "exercises/quizzes/quiz2.rs",
        hints: &[
            "Fix one symptom at a time, in the order the check reports them. For each one, ask \
             which handle, callback or task should have made it happen.",
            "The fixes reuse lessons 17 (strong and weak handles), 10 (what an observer \
             reads), 11 (keeping a subscription) and 19 (keeping a task), in that order.",
            "Store the badge as an `Entity<Badge>` and render it with \
             `.child(self.badge.clone())`. In the badge's observer, read `unread` from the \
             observed inbox. Keep the log's subscription in `_subscriptions`. In `fetch`, \
             store the task in `self.fetch` instead of detaching it.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "21",
        name: "layout_states1",
        chapter: "Layout & control states",
        title: "Adapt a layout to window width",
        concept: "Window bounds & responsive layout",
        duration: "6 min",
        objective: "Stack the cards in a narrow window; keep them side by side when wide.",
        test: "exercises::layout_states1::tests::exercise_21",
        file: "exercises/06_layout_states/layout_states1.rs",
        hints: &[
            "You picked a flex direction in lesson 04, too. The wide branch uses `flex_row()`. \
             Which one stacks children from top to bottom?\n\
             \n\
             The check resizes the window both ways, so leave the wide branch as it is.\n\
             \n\
             Read more in the Style guide:\n\
             https://gpui-kit.com/docs/style/#flexbox-and-grid",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "22",
        name: "layout_states2",
        chapter: "Layout & control states",
        title: "Keep content usable in a small window",
        concept: "Overflow & scroll handles",
        duration: "6 min",
        objective: "Scroll the list to its last item, by wheel and with Jump to last.",
        test: "exercises::layout_states2::tests::exercise_22",
        file: "exercises/06_layout_states/layout_states2.rs",
        hints: &[
            "Two things are missing. The list needs an overflow mode that lets it scroll \
             vertically, and it needs to track `self.scroll`, the handle that Jump to last \
             moves.\n\
             \n\
             Read more in the Style guide:\n\
             https://gpui-kit.com/docs/style/#choose-clipping-scrolling-or-positioning",
            "Replace `overflow_hidden()` with `overflow_y_scroll()`, then add \
             `.track_scroll(&self.scroll)` to the list.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "23",
        name: "layout_states3",
        chapter: "Layout & control states",
        title: "Keep a disabled control disabled",
        concept: "Disabled state & activation paths",
        duration: "7 min",
        objective: "Keep a disabled control visible but unable to change selection.",
        test: "exercises::layout_states3::tests::exercise_23",
        file: "exercises/06_layout_states/layout_states3.rs",
        hints: &[
            "Both ways to activate the control end up in `toggle_selection`, so one check \
             there covers the mouse and the keyboard. Leave `selected` alone while \
             `self.disabled` is true, either with an `if` around the change or by returning \
             early.\n\
             \n\
             Read more about input events in the Event guide:\n\
             https://gpui-kit.com/docs/event/#pointer-and-keyboard-input-are-also-events",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "24",
        name: "layout_states4",
        chapter: "Layout & control states",
        title: "Route a pointer gesture",
        concept: "Mouse down, move & release",
        duration: "10 min",
        objective: "Drag the value, then release outside without leaving it active.",
        test: "exercises::layout_states4::tests::exercise_24",
        file: "exercises/06_layout_states/layout_states4.rs",
        hints: &[
            "Each handler has the same shape as `on_mouse_down`: a mouse button (the move \
             handler takes none) and a `cx.listener` closure. Both release handlers do the \
             same thing: clear `drag_start` and notify.\n\
             \n\
             Read more about pointer events in the Event guide:\n\
             https://gpui-kit.com/docs/event/#pointer-and-keyboard-input-are-also-events",
            "Add `.on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| \
             this.move_pointer(event.position.x, cx)))`, then `.on_mouse_up(MouseButton::Left, \
             cx.listener(|this, _, _, cx| { this.drag_start = None; cx.notify(); }))`, and the \
             same again with `on_mouse_up_out`.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "25",
        name: "quiz3",
        chapter: "Layout & control states",
        title: "Quiz 3: compact inspector",
        concept: "Responsive layout, scrolling & input",
        duration: "20 min",
        objective: "Find and fix four reported bugs in a compact inspector.",
        test: "exercises::quiz3::tests::exercise_25",
        file: "exercises/quizzes/quiz3.rs",
        hints: &[
            "Fix one symptom at a time, in the order the check reports them. In the preview, \
             change the window's width and height separately.",
            "The fixes reuse lessons 21 (what a breakpoint measures), 13 (where key events \
             go), 23 (guarding every way in) and 22 (what lets a region scroll), in that \
             order.",
            "Compare the width, not the height. Move `on_key_down` from the detail to the \
             list, which tracks focus. Guard `next()` with `!self.disabled`. Give the list the \
             detail's fixed height.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "26",
        name: "dispatch1",
        chapter: "Focus & dispatch",
        title: "Move between focus regions",
        concept: "Focus handles & overlays",
        duration: "7 min",
        objective: "Return keyboard focus to Open after Close or Escape.",
        test: "exercises::dispatch1::tests::exercise_26",
        file: "exercises/07_dispatch/dispatch1.rs",
        hints: &[
            "Both ways out take the same three steps: hide the overlay, move focus to \
             `this.trigger`, and notify. The Open button's listener shows how to move focus, \
             and the pads show how to handle a key. Escape is called \"escape\".\n\
             \n\
             Read more about focus in overlays in the Focus guide:\n\
             https://gpui-kit.com/docs/focus/#trap-and-restore-focus-in-an-overlay",
            "In Close's listener, add `window.focus(&this.trigger, cx);`. On the overlay \
             `div()`, add `.on_key_down(cx.listener(|this, event: &gpui_kit::KeyDownEvent, \
             window, cx| { ... }))` that does the same when `event.keystroke.key == \
             \"escape\"`.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "27",
        name: "dispatch2",
        chapter: "Focus & dispatch",
        title: "Route a nested action",
        concept: "Action propagation",
        duration: "7 min",
        objective: "Handle Ctrl-R once in the focused child or fall back to its parent.",
        test: "exercises::dispatch2::tests::exercise_27",
        file: "exercises/07_dispatch/dispatch2.rs",
        hints: &[
            "Add an `else` branch to the `if`, and call `cx.propagate()` there. The check \
             presses Ctrl-R in both modes and counts each handler, so exactly one of them \
             should run each time.\n\
             \n\
             Read more in the Action guide:\n\
             https://gpui-kit.com/docs/action/#handler-order-and-propagation",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "28",
        name: "quiz4",
        chapter: "Focus & dispatch",
        title: "Quiz 4: command menu",
        concept: "Actions, focus & keyboard routing",
        duration: "15 min",
        objective: "Find and fix three reported bugs in a keyboard command menu.",
        test: "exercises::quiz4::tests::exercise_28",
        file: "exercises/quizzes/quiz4.rs",
        hints: &[
            "Fix one symptom at a time, in the order the check reports them. Use only the \
             keyboard in the preview, and watch where focus goes.",
            "The fixes reuse lessons 07 (telling GPUI a view changed), 16 (where a key context \
             has to sit for a binding to match) and 26 (returning focus when an overlay \
             closes), in that order.",
            "Notify after the arrow keys change `selected`. Move \
             `key_context(\"CommandMenuDemo\")` from the menu to the view's root. In \
             `cancel()`, focus the launcher, not the menu.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "29",
        name: "async1",
        chapter: "Async data & failure paths",
        title: "Run work in the background",
        concept: "BackgroundExecutor & Task",
        duration: "12 min",
        objective: "Show the computed result while keeping the UI responsive.",
        test: "exercises::async1::tests::exercise_29",
        file: "exercises/08_async/async1.rs",
        hints: &[
            "The background block can't use `cx`, so create the timer before it and move it \
             in: `let timer = cx.background_executor().timer(Duration::from_secs(1));`. Inside \
             `cx.background_executor().spawn(async move { ... })`, await the timer and end \
             with the sum. Then await that task in a `cx.spawn` task, as lesson 19's `load` \
             does.\n\
             \n\
             Read more about moving work off the UI thread in the Task guide:\n\
             https://gpui-kit.com/docs/task/#move-heavy-work-off-the-ui-thread",
            "`let work = cx.background_executor().spawn(async move { timer.await; \
             (1..=1_000).sum::<u32>() });`, then `self.task = Some(cx.spawn(async move |this, \
             cx| { let result = work.await; let _ = this.update(cx, |this, cx| { ... }); \
             }));`, setting `status`, `total` and notifying inside the update.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "30",
        name: "async2",
        chapter: "Async data & failure paths",
        title: "Show errors and retry",
        concept: "Async UI states & retry",
        duration: "8 min",
        objective: "Clear an error during retry, then show the returned data.",
        test: "exercises::async2::tests::exercise_30",
        file: "exercises/08_async/async2.rs",
        hints: &[
            "Compare `retry` with `load` just above it. `load` sets the state to Loading and \
             notifies before it calls `request`.\n\
             \n\
             Read more about failure and retry in the Task guide:\n\
             https://gpui-kit.com/docs/task/#handle-completion-failure-and-cancellation",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "31",
        name: "async3",
        chapter: "Async data & failure paths",
        title: "Ignore stale results",
        concept: "Out-of-order async completion",
        duration: "8 min",
        objective: "Keep the newer selection visible after an older request completes.",
        test: "exercises::async3::tests::exercise_31",
        file: "exercises/08_async/async3.rs",
        hints: &[
            "`this.selected` is an `Option<&str>`, so compare it with `Some(label)` before you \
             set `visible`.\n\
             \n\
             Another common approach is a request counter, or *generation*, that each request \
             captures and compares when it finishes.\n\
             \n\
             Read more in the Task guide:\n\
             https://gpui-kit.com/docs/task/#move-heavy-work-off-the-ui-thread",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "32",
        name: "quiz5",
        chapter: "Async data & failure paths",
        title: "Quiz 5: searchable results",
        concept: "Text input, background work & keyboard selection",
        duration: "20 min",
        objective: "Find and fix three reported bugs in an async search.",
        test: "exercises::quiz5::tests::exercise_32",
        file: "exercises/quizzes/quiz5.rs",
        hints: &[
            "Fix one symptom at a time, in the order the check reports them. For the async \
             ones, notice which request finishes last.",
            "The fixes reuse lessons 09 (which entity you hold), 31 (recognizing an older \
             completion) and 30 (what Retry has to do), in that order.",
            "Store and render the `InputState` you subscribe to. Ignore a completion whose \
             generation differs from `this.generation`. Remove the early return for a repeated \
             query, so Retry searches again.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "33",
        name: "application1",
        chapter: "Application structure",
        title: "Start a standalone GPUI app",
        concept: "Application, init & root window",
        duration: "9 min",
        objective: "Open a new window with WorkspaceRoot as its root view.",
        test: "exercises::application1::tests::exercise_33",
        file: "exercises/09_application/application1.rs",
        hints: &[
            "`open_window` takes window options (`WindowOptions::default()` will do), `cx`, \
             and a closure that creates the root view, such as `|_, cx| cx.new(|_| \
             WorkspaceRoot)`. It returns a `Result` holding the window and that root.\n\
             \n\
             Read more in the Window guide:\n\
             https://gpui-kit.com/docs/window/#open-and-own-a-window",
            "Call `gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| cx.new(|_| \
             WorkspaceRoot))`. It returns a `Result` of `(window, root)`: use `.ok()` and \
             `.map` to return the root as an `Option`.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "34",
        name: "application2",
        chapter: "Application structure",
        title: "Share application state",
        concept: "Global & observe_global",
        duration: "8 min",
        objective: "Update one global setting and show it in two child views.",
        test: "exercises::application2::tests::exercise_34",
        file: "exercises/09_application/application2.rs",
        hints: &[
            "It's the same fix as lesson 11: keep `Some(subscription)` instead of `None`.\n\
             \n\
             Read more in the Global guide:\n\
             https://gpui-kit.com/docs/global/#read-and-change-a-global",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "35",
        name: "application3",
        chapter: "Application structure",
        title: "Save and restore a setting",
        concept: "Persistence & fallback",
        duration: "8 min",
        objective: "Restore the saved setting and recover from invalid data.",
        test: "exercises::application3::tests::exercise_35",
        file: "exercises/09_application/application3.rs",
        hints: &[
            "`fs::read_to_string(path)` returns a `Result<String, io::Error>`. You can `match` \
             on it, or use a `Result` method like `is_ok_and`, which takes a closure for the \
             `Ok` value.\n\
             \n\
             This part is plain Rust, so the Rust book has you covered:\n\
             https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "36",
        name: "application4",
        chapter: "Application structure",
        title: "Open a second window",
        concept: "Windows, shared entities & cleanup",
        duration: "10 min",
        objective: "Share a count across windows and reopen after closing detail.",
        test: "exercises::application4::tests::exercise_36",
        file: "exercises/09_application/application4.rs",
        hints: &[
            "There's a field waiting for it: `_close_subscription`. It's the move from lessons \
             11 and 34 again.\n\
             \n\
             Read more about closing windows in the Multi Window guide:\n\
             https://gpui-kit.com/docs/multi-window/#close-and-clean-up",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "37",
        name: "application5",
        chapter: "Application structure",
        title: "Follow appearance changes",
        concept: "Window appearance & semantic colors",
        duration: "8 min",
        objective: "Keep text legible in light and dark appearance.",
        test: "exercises::application5::tests::exercise_37",
        file: "exercises/09_application/application5.rs",
        hints: &[
            "The text should use `palette.foreground` in both appearances, so you don't need \
             the `if` at all.\n\
             \n\
             Read more about themes in GPUI Kit's coding guide:\n\
             https://gpui-kit.com/docs/coding-guides/#theme-and-styling",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "38",
        name: "quality1",
        chapter: "Ship-quality GPUI",
        title: "Expose an accessible control",
        concept: "Role, name, state & keyboard activation",
        duration: "9 min",
        objective: "Give a switch an accessible name without losing its behavior.",
        test: "exercises::quality1::tests::exercise_38",
        file: "exercises/10_quality/quality1.rs",
        hints: &[
            "Add `.accessibility_label(\"Enable alerts\")` to the switch's builder chain, for \
             example right after `.checked(enabled)`.\n\
             \n\
             Read more about names, roles and states in the Accessibility guide:\n\
             https://gpui-kit.com/docs/accessibility/#names-states-and-relationships",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "39",
        name: "quality2",
        chapter: "Ship-quality GPUI",
        title: "Test behavior through GPUI",
        concept: "Headless interaction & simulated time",
        duration: "9 min",
        objective: "Write a GPUI test that clicks Load and waits on simulated time.",
        test: "exercises::quality2::tests::exercise_39",
        file: "exercises/10_quality/quality2.rs",
        hints: &[
            "The window context can click a point, and its executor can move simulated time \
             forward. After moving it, run the work that became ready. The checks in earlier \
             lessons do exactly this; lesson 19's is a good one to read.\n\
             \n\
             Read more in the Testing guide:\n\
             https://gpui-kit.com/docs/test/#interact-and-assert",
            "Use `window.simulate_click(load.center(), Modifiers::default())`, then \
             `window.executor().advance_clock(Duration::from_secs(1))` and \
             `window.run_until_parked()`.",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "40",
        name: "quality3",
        chapter: "Ship-quality GPUI",
        title: "Render a large collection efficiently",
        concept: "Virtual list & visible range",
        duration: "9 min",
        objective: "Build only visible rows while preserving selection and scroll.",
        test: "exercises::quality3::tests::exercise_40",
        file: "exercises/10_quality/quality3.rs",
        hints: &[
            "`range` is a `Range<usize>`, and ranges are iterators. `map` each index to \
             `self.row(ix, cx)` and `collect` the rows, the way the first line does with \
             `0..ROWS`.\n\
             \n\
             Read more about virtual lists in GPUI Kit:\n\
             https://gpui-kit.com/component/virtual-list",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "41",
        name: "quality4",
        chapter: "Ship-quality GPUI",
        title: "Compose a reusable component",
        concept: "Controlled input, output & theme",
        duration: "9 min",
        objective: "Keep two switches independent and the disabled one inert.",
        test: "exercises::quality4::tests::exercise_41",
        file: "exercises/10_quality/quality4.rs",
        hints: &[
            "The wrapper's colors and text already follow its `checked` parameter. Give the \
             same parameter to `Switch::checked`.\n\
             \n\
             Read more about why state belongs outside a reusable component:\n\
             https://gpui-kit.com/docs/render-once/#state-belongs-outside-the-component-value",
        ],
        verifier: Verifier::Native,
    },
    Lesson {
        id: "42",
        name: "quiz6",
        chapter: "Ship-quality GPUI",
        title: "Quiz 6 (capstone): small native workspace",
        concept: "Entities, layout, actions, async & persistence",
        duration: "25 min",
        objective: "Find and fix five reported bugs in a two-pane workspace.",
        test: "exercises::quiz6::tests::exercise_42",
        file: "exercises/quizzes/quiz6.rs",
        hints: &[
            "Fix one symptom at a time, in the order the check reports them. Reproduce each \
             one in the preview before you change any code.",
            "The fixes reuse lessons 19 (a task's lifetime), 30 (retry state), 21 \
             (breakpoints), 09 (the entity, not a copy) and 13 (attaching focus), in that \
             order.",
            "Store `request()`'s task in `self._task`. Set Loading and notify in `retry()`. \
             Flip the width comparison. Read the selected index from `self.model` in `save()`. \
             Add `track_focus(&self.focus)` to the workspace.",
        ],
        verifier: Verifier::Native,
    },
];

/// The position of the lesson with this ID, or the first lesson when none has it.
pub fn index_of(id: &str) -> usize {
    LESSONS
        .iter()
        .position(|lesson| lesson.id == id.trim())
        .unwrap_or(0)
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
                "Hint {} of {count}: {text}\n\n{more} for a more specific hint.",
                level + 1
            )
        } else {
            format!("Hint {count} of {count}: {text}")
        }
    }
}
