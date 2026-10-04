// Slow work shouldn't run on the UI thread, or the whole window freezes until
// it's done. GPUI's background executor runs work on other threads instead:
// `cx.background_executor().spawn(async move { ... })` returns a `Task` that
// you can `.await` to get the block's result. Its `timer(duration)` gives you
// something to await, too.
//
// Background work can't touch views, though, because they live on the UI
// thread. So a foreground task, started with `cx.spawn` as in lesson 19,
// awaits the result and then updates the view through its weak handle. This
// time you write both tasks. Press Start, and while it runs, press Ping to
// check that the window still responds.

use crate::theme::button;
use gpui_kit::{Context, IntoElement, Render, Task, Window, div, prelude::*};
use std::time::Duration;

#[derive(Default)]
pub struct BackgroundPanel {
    // The foreground task from Start. The panel owns it, as in lesson 19, so
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
        // TODO: Nothing computes the result yet. Write the two tasks:
        // - On the background executor, wait for a one-second `timer` (it
        //   stands in for slow work), then return the sum of 1 through 1,000.
        // - In a foreground task, like lesson 19's, await that result, then
        //   set `status` to "Ready" and `total` to the sum, and notify.
        // Keep the foreground task in `self.task`.
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
                    .child(if self.status.is_empty() {
                        "Not started".to_string()
                    } else {
                        format!("{}: {}", self.status, self.total)
                    }),
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
    fn exercise_29(cx: &mut TestAppContext) {
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
            assert_eq!(
                state.status, "Ready",
                "once the background work finishes, show Ready"
            );
            assert_eq!(state.total, 500_500, "show the returned background result");
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("background-Ready-500500").is_some());
    }
}
