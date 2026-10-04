// `cx.spawn` starts async work on GPUI's UI thread and gives you back a `Task`.
// The task *is* the work: dropping it cancels the work. So whoever decides when
// the work should stop keeps the task. Here, that's the panel. Cancel drops it,
// and starting a new load replaces the old one, which cancels it too.
//
// Inside the async block, `this` is a weak handle to the panel, and `cx` is an
// async context that you can hold across `.await`. After waiting, the block
// updates the panel through `this`. That fails harmlessly if the panel has
// closed in the meantime.

use crate::theme::button;
use gpui_kit::{Context, IntoElement, Render, Task, Window, div, prelude::*};
use std::time::Duration;

#[derive(Default)]
pub struct TasksPanel {
    task: Option<Task<()>>,
    status: Option<&'static str>,
}
impl TasksPanel {
    fn load(&mut self, cx: &mut Context<Self>) {
        self.status = Some("Loading…");
        cx.notify();
        // `cx.spawn` starts the async block on GPUI's foreground executor and
        // returns a Task, a handle that owns the work. The block receives a weak
        // handle to this view (`this`) and an async context (`cx`).
        let task = cx.spawn(async move |this, cx| {
            // Wait a second without blocking the UI.
            cx.background_executor().timer(Duration::from_secs(1)).await;
            // The view may have closed meanwhile, so `update` returns a Result.
            let _ = this.update(cx, |this, cx| {
                this.status = Some("Ready");
                cx.notify();
            });
        });
        // TODO: `task` is dropped when `load` returns, which cancels the work
        // before the timer fires. Keep it in the panel instead.
    }
}
impl Render for TasksPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                button("task-load", "Load", true)
                    .debug_selector(|| "task-load".into())
                    .on_click(cx.listener(|this, _, _, cx| this.load(cx))),
            )
            .child(
                button("task-cancel", "Cancel", false)
                    .debug_selector(|| "task-cancel".into())
                    .on_click(cx.listener(|this, _, _, cx| {
                        // Taking the task out of the panel drops it: cancelled.
                        this.task.take();
                        this.status = Some("Cancelled");
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .debug_selector(|| format!("task-{}", self.status.unwrap_or("Idle")))
                    .child(self.status.unwrap_or("Idle")),
            )
    }
}

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_19(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, _| TasksPanel::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let load = cx.debug_bounds("task-load").unwrap();
        cx.simulate_click(load.center(), Modifiers::default());
        cx.run_until_parked();
        cx.update(|_, cx| {
            assert_eq!(
                panel.read(cx).status,
                Some("Loading…"),
                "loading must be observable before the task completes"
            )
        });
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        cx.update(|window, cx| {
            assert_eq!(
                panel.read(cx).status,
                Some("Ready"),
                "retain the Task; dropping it cancels the future"
            );
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("task-Ready").is_some());
        cx.simulate_click(load.center(), Modifiers::default());
        cx.run_until_parked();
        let cancel = cx.debug_bounds("task-cancel").unwrap();
        cx.simulate_click(cancel.center(), Modifiers::default());
        cx.executor().advance_clock(Duration::from_secs(2));
        cx.run_until_parked();
        cx.update(|_, cx| {
            assert_eq!(
                panel.read(cx).status,
                Some("Cancelled"),
                "a cancelled task must not replace the current state"
            )
        });
    }
}
