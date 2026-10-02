//! 15 — Keep async work alive
//!
//! cx.spawn creates work on GPUI's foreground executor and returns a Task.
//! Dropping Task cancels its work; store it when the view should own its lifetime.
//! The async callback receives a WeakEntity and AsyncApp, which can cross await.
//! Re-enter a synchronous update to change the view and call notify afterward.
//!
//! Goal: Load changes Loading to Ready after one second, while Cancel still works.
//! Store the task in self.task instead of dropping it. A new load replaces the
//! old task; Cancel takes and drops it. Use the executor's timer rather than
//! thread::sleep, which would block the UI. CPU-heavy work belongs on the
//! background executor; this simulated delay only needs an asynchronous timer.
//!
//! Example — Owning asynchronous work:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! let pending = cx.spawn(async move |view, cx| {
//!     cx.background_executor().timer(Duration::from_millis(100)).await;
//!     let _ = view.update(cx, |view, cx| {
//!         view.ready = true;
//!         cx.notify();
//!     });
//! });
//! self.pending = Some(pending); // Dropping this handle cancels the work.
//! ```

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
        let task = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(Duration::from_secs(1)).await;
            let _ = this.update(cx, |this, cx| {
                this.status = Some("Ready");
                cx.notify();
            });
        });
        // TODO: Retain the task so it can finish, and remain cancellable.
        drop(task);
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_15(cx: &mut TestAppContext) {
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
