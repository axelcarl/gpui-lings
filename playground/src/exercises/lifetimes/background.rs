//! 25 — Run expensive work off the UI thread
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

use crate::theme::button;
use gpui_kit::{Context, IntoElement, Render, Task, Window, div, prelude::*};
use std::time::Duration;

#[derive(Default)]
pub struct BackgroundPanel {
    task: Option<Task<()>>,
    status: &'static str,
    total: u32,
    pings: u32,
}

impl BackgroundPanel {
    // TODO: Show the background sum when the work finishes.
    fn start(&mut self, cx: &mut Context<Self>) {
        self.status = "Loading";
        self.total = 0;
        cx.notify();

        let timer = cx.background_executor().timer(Duration::from_secs(1));
        let work = cx.background_executor().spawn(async move {
            timer.await;
            (1..=1_000).sum::<u32>()
        });
        self.task = Some(cx.spawn(async move |this, cx| {
            let result = work.await;
            let _ = this.update(cx, |this, cx| {
                this.status = "Ready";
                this.total = 0;
                let _ = result;
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
