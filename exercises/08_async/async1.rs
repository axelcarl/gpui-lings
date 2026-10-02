//! 25 — Run work in the background
//!
//! GPUI's background executor runs Send work away from the foreground UI.
//! The returned Task can be awaited by a foreground cx.spawn task, which
//! re-enters the view through its WeakEntity before changing rendered state.
//! Keep the foreground task in the view so it is cancelled when the view goes.
//!
//! Goal: Start computes the sum of 1 through 1,000 and displays it after a
//! simulated one-second delay. Ping must still work while the calculation is
//! pending. The background work is already scheduled; use its returned value
//! when updating the view instead of showing the placeholder result.
//!
//! Example — Awaiting a background result:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! let work = cx.background_executor().spawn(async { 6 * 7 });
//! self.pending = Some(cx.spawn(async move |view, cx| {
//!     let answer = work.await;
//!     let _ = view.update(cx, |view, cx| {
//!         view.answer = answer;
//!         cx.notify();
//!     });
//! }));
//! ```

use crate::theme::button;
use gpui_kit::{Context, IntoElement, Render, Task, Window, div, prelude::*};
use std::time::Duration;

#[derive(Default)]
pub struct BackgroundPanel {
    // The foreground task from Start. The panel owns it, as in lesson 15, so
    // it runs until it finishes or the panel goes away.
    task: Option<Task<()>>,
    status: &'static str,
    // The number shown after the status.
    total: u32,
    // Counts Ping clicks, which prove the UI responds while the sum is pending.
    pings: u32,
}

impl BackgroundPanel {
    fn start(&mut self, cx: &mut Context<Self>) {
        self.status = "Loading";
        self.total = 0;
        cx.notify();

        // A one-second timer that stands in for slow work.
        let timer = cx.background_executor().timer(Duration::from_secs(1));
        // `background_executor().spawn` runs this block on another thread and
        // returns a Task that resolves to the block's value, the sum. Away from
        // the UI thread, the block can't reach views or `cx`: it only computes.
        let work = cx.background_executor().spawn(async move {
            timer.await;
            (1..=1_000).sum::<u32>()
        });
        // `cx.spawn` runs on the UI thread, where views can be updated. Awaiting
        // `work` waits for the sum without blocking clicks such as Ping.
        self.task = Some(cx.spawn(async move |this, cx| {
            let result = work.await;
            // `this` is a weak handle to the panel; `update` fails if it closed.
            let _ = this.update(cx, |this, cx| {
                this.status = "Ready";
                // TODO: `total` is reset to zero and `result` is ignored. Show
                // the sum the background task returned.
                this.total = 0;
                cx.notify();
            });
        }));
    }
}

impl Render for BackgroundPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                button("background-start", "Start", true)
                    .debug_selector(|| "background-start".into())
                    .on_click(cx.listener(|this, _, _, cx| this.start(cx))),
            )
            .child(
                button("background-ping", "Ping", false)
                    .debug_selector(|| "background-ping".into())
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.pings += 1;
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .debug_selector(|| format!("background-{}-{}", self.status, self.total))
                    .child(format!("{}: {}", self.status, self.total)),
            )
            .child(div().child(format!("Pings: {}", self.pings)))
    }
}

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_25(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, _| BackgroundPanel::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let start = cx.debug_bounds("background-start").unwrap();
        let ping = cx.debug_bounds("background-ping").unwrap();
        cx.simulate_click(start.center(), Modifiers::default());
        cx.run_until_parked();
        cx.simulate_click(ping.center(), Modifiers::default());
        cx.update(|window, cx| {
            assert_eq!(panel.read(cx).status, "Loading");
            assert_eq!(panel.read(cx).pings, 1, "the UI should remain interactive");
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("background-Loading-0").is_some());
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        cx.update(|window, cx| {
            let state = panel.read(cx);
            assert_eq!(state.status, "Ready");
            assert_eq!(state.total, 500_500, "show the returned background result");
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("background-Ready-500500").is_some());
    }
}
