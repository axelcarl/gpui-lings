//! 27 — Ignore stale results
//!
//! Async requests can finish in a different order from the one in which they
//! started. Keep a generation or selection key with each request, then check
//! it when the result returns to the entity. An old request must not replace
//! the view's newer choice. WeakEntity updates also let a released view go.
//!
//! Goal: selecting Slow, then Fast, should leave Fast visible even after Slow
//! completes. Guard the completion with the current selection. Both tasks are
//! retained deliberately so the check exercises out-of-order completion.
//!
//! Example — Associating work with the request that started it:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! self.generation += 1;
//! let request_id = self.generation;
//! // Capture request_id in the task and compare it on completion.
//! // Only the current generation may replace the displayed result.
//! ```

use crate::theme::button;
use gpui_kit::{Context, IntoElement, Render, Task, Window, div, prelude::*};
use std::time::Duration;

#[derive(Default)]
pub struct StalePanel {
    selected: Option<&'static str>,
    visible: Option<&'static str>,
    tasks: Vec<Task<()>>,
}

impl StalePanel {
    // TODO: An older, slower selection must not replace a newer one.
    fn select(&mut self, label: &'static str, delay: Duration, cx: &mut Context<Self>) {
        self.selected = Some(label);
        self.visible = None;
        cx.notify();
        self.tasks.push(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(delay).await;
            let _ = this.update(cx, |this, cx| {
                this.visible = Some(label);
                cx.notify();
            });
        }));
    }
}

impl Render for StalePanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                button("stale-slow", "Slow", true)
                    .debug_selector(|| "stale-slow".into())
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.select("Slow", Duration::from_secs(2), cx)
                    })),
            )
            .child(
                button("stale-fast", "Fast", false)
                    .debug_selector(|| "stale-fast".into())
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.select("Fast", Duration::from_secs(1), cx)
                    })),
            )
            .child(
                div()
                    .debug_selector(|| format!("stale-{}", self.visible.unwrap_or("Loading")))
                    .child(self.visible.unwrap_or("Loading")),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_27(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, _| StalePanel::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let slow = cx.debug_bounds("stale-slow").unwrap();
        let fast = cx.debug_bounds("stale-fast").unwrap();
        cx.simulate_click(slow.center(), Modifiers::default());
        cx.simulate_click(fast.center(), Modifiers::default());
        cx.run_until_parked();
        cx.update(|_, cx| {
            assert_eq!(panel.read(cx).selected, Some("Fast"));
            assert_eq!(panel.read(cx).visible, None);
        });
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        cx.update(|_, cx| assert_eq!(panel.read(cx).visible, Some("Fast")));
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        cx.update(|window, cx| {
            assert_eq!(
                panel.read(cx).visible,
                Some("Fast"),
                "ignore the old result"
            );
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("stale-Fast").is_some());
    }

    #[gpui::test]
    fn released_view_drops_pending_work(cx: &mut TestAppContext) {
        let panel = cx.new(|_| StalePanel::default());
        let weak = panel.downgrade();
        panel.update(cx, |panel, cx| {
            panel.select("Slow", Duration::from_secs(2), cx);
        });
        drop(panel);
        cx.run_until_parked();
        assert!(weak.upgrade().is_none());
        cx.executor().advance_clock(Duration::from_secs(2));
        cx.run_until_parked();
        assert!(weak.upgrade().is_none());
    }
}
