// While GPUI runs your listener, it's in the middle of updating your view: the
// view stays mutably borrowed until the listener returns. Updating the same
// view again through its handle before then would panic.
//
// `cx.defer(...)` gets around that. It schedules a closure to run once the
// current update has finished. The closure only receives the app (`&mut App`),
// not your view, so it needs a handle to reach the view again. Take a weak one
// with `cx.weak_entity()`, as in lesson 17: the view might be gone by the time
// the closure runs.

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
        // TODO: Nothing records "Settled" yet. Push it onto `history` in a
        // second step, once this update has ended, and notify after that.
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
    fn exercise_18(cx: &mut TestAppContext) {
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
