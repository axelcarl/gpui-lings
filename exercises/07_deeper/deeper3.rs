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
//!
//! Example — Scheduling an update after the current borrow ends:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! let view = cx.weak_entity();
//! cx.defer(move |cx| {
//!     let _ = view.update(cx, |view, cx| {
//!         view.ready = true;
//!         cx.notify();
//!     });
//! });
//! ```

use crate::theme::button;
use gpui_kit::{Context, IntoElement, Render, Window, div, prelude::*};

#[derive(Default)]
pub struct DeferredPanel {
    // Every step recorded so far, in order. The label shows the latest one.
    history: Vec<&'static str>,
}

impl DeferredPanel {
    // The Queue listener calls this while GPUI is updating this view, so the
    // view stays borrowed until `queue` returns. Updating it again through its
    // handle before then would panic.
    fn queue(&mut self, cx: &mut Context<Self>) {
        self.history.push("Queued");
        cx.notify();
        // TODO: Nothing records "Settled" yet. Schedule that second step with
        // `cx.defer`, which runs a closure once the current update has ended.
        // The closure gets only `&mut App`, not this view, so capture
        // `cx.weak_entity()` before the call and `update` the view through it.
        // Notify after the change.
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

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
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
