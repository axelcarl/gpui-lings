// A request can be loading, done or failed, and the view should always show
// which. This panel keeps that in a `LoadState` and renders from it. Its
// "server" is a queue of canned answers, so nothing touches the network: the
// first request fails with Offline, and the next one succeeds. Each answer
// takes a second.
//
// Press Load, wait for Offline, then press Retry. Right now, the old error
// stays on screen for the whole second the retry takes.

use crate::theme::button;
use gpui_kit::{Context, IntoElement, Render, Task, Window, div, prelude::*};
use std::{collections::VecDeque, time::Duration};

// Everything the panel can show. Render turns the current state into its label.
#[derive(Clone, Debug, PartialEq, Eq)]
enum LoadState {
    Idle,
    Loading,
    Error(&'static str),
    Success(&'static str),
}

pub struct RetryPanel {
    // Canned answers that stand in for a server: each request takes the next.
    responses: VecDeque<Result<&'static str, &'static str>>,
    state: LoadState,
    // The request in flight. Storing a new one drops, and so cancels, the old.
    task: Option<Task<()>>,
}

impl Default for RetryPanel {
    fn default() -> Self {
        // The first request fails with "Offline"; the second succeeds.
        Self::new([Err("Offline"), Ok("Latest report")])
    }
}

impl RetryPanel {
    fn new(responses: impl IntoIterator<Item = Result<&'static str, &'static str>>) -> Self {
        Self {
            responses: responses.into_iter().collect(),
            state: LoadState::Idle,
            task: None,
        }
    }

    // Sends a request. It only changes `state` when the answer arrives, a
    // second later; until then, whatever was on screen stays there.
    fn request(&mut self, cx: &mut Context<Self>) {
        let response = self.responses.pop_front().unwrap_or(Err("No response"));
        self.task = Some(cx.spawn(async move |this, cx| {
            // A simulated network delay. The check moves the clock forward
            // rather than waiting.
            cx.background_executor().timer(Duration::from_secs(1)).await;
            // Re-enter the panel through its weak handle to store the outcome.
            let _ = this.update(cx, |this, cx| {
                this.state = match response {
                    Ok(value) => LoadState::Success(value),
                    Err(message) => LoadState::Error(message),
                };
                cx.notify();
            });
        }));
    }

    fn load(&mut self, cx: &mut Context<Self>) {
        self.state = LoadState::Loading;
        cx.notify();
        self.request(cx);
    }

    fn retry(&mut self, cx: &mut Context<Self>) {
        // TODO: Retry requests again, but the old error stays on screen for the
        // whole second. Show Loading first and notify, the way `load` does.
        self.request(cx);
    }
}

impl Render for RetryPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let label = match self.state {
            LoadState::Idle => "Idle",
            LoadState::Loading => "Loading",
            LoadState::Error(message) | LoadState::Success(message) => message,
        };
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                button("retry-load", "Load", true)
                    .debug_selector(|| "retry-load".into())
                    .on_click(cx.listener(|this, _, _, cx| this.load(cx))),
            )
            .child(
                button("retry-try-again", "Retry", false)
                    .debug_selector(|| "retry-try-again".into())
                    .on_click(cx.listener(|this, _, _, cx| this.retry(cx))),
            )
            .child(
                div()
                    .debug_selector(|| format!("retry-{label}"))
                    .child(label),
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
    fn exercise_30(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, _| RetryPanel::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let load = cx.debug_bounds("retry-load").unwrap();
        let retry = cx.debug_bounds("retry-try-again").unwrap();
        cx.simulate_click(load.center(), Modifiers::default());
        cx.update(|window, cx| {
            assert_eq!(panel.read(cx).state, LoadState::Loading);
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("retry-Loading").is_some());
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        cx.update(|window, cx| {
            assert_eq!(panel.read(cx).state, LoadState::Error("Offline"));
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("retry-Offline").is_some());
        cx.simulate_click(retry.center(), Modifiers::default());
        cx.update(|window, cx| {
            assert_eq!(
                panel.read(cx).state,
                LoadState::Loading,
                "retry must replace the old error immediately"
            );
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("retry-Loading").is_some());
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        cx.update(|window, cx| {
            assert_eq!(panel.read(cx).state, LoadState::Success("Latest report"));
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("retry-Latest report").is_some());
    }
}
