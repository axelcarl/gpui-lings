//! 23 — Defer a follow-up update
//!
//! GPUI owns entity data. During a view's update, its mutable state is already
//! borrowed. `cx.defer` schedules a closure for the end of the current effect
//! cycle, after that borrow has ended. Capture a weak handle, then re-enter an
//! entity update inside the deferred closure; the view may have disappeared.
//!
//! Goal: clicking Queue first records Queued and then records Settled. Use
//! cx.defer for the second step and notify after changing the view. The check
//! verifies the order and the rendered final state without sleeping.

use crate::theme::button;
use gpui_kit::{Context, IntoElement, Render, Window, div, prelude::*};

#[derive(Default)]
pub struct DeferredPanel {
    history: Vec<&'static str>,
}

impl DeferredPanel {
    // TODO: After recording Queued, record Settled in a deferred update.
    fn queue(&mut self, cx: &mut Context<Self>) {
        self.history.push("Queued");
        cx.notify();
    }
}

impl Render for DeferredPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                button("deferred-queue", "Queue", true)
                    .debug_selector(|| "deferred-queue".into())
                    .on_click(cx.listener(|this, _, _, cx| this.queue(cx))),
            )
            .child(
                div()
                    .debug_selector(|| {
                        format!(
                            "deferred-{}",
                            self.history.last().copied().unwrap_or("Idle")
                        )
                    })
                    .child(self.history.last().copied().unwrap_or("Idle")),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_23(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, _| DeferredPanel::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let queue = cx.debug_bounds("deferred-queue").expect("Queue missing");
        cx.simulate_click(queue.center(), Modifiers::default());
        cx.run_until_parked();
        cx.update(|window, cx| {
            assert_eq!(
                panel.read(cx).history,
                ["Queued", "Settled"],
                "the second update should run after the first"
            );
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("deferred-Settled").is_some());
    }
}
