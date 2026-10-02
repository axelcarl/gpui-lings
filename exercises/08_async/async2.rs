//! 26 — Show errors and retry
//!
//! A view should expose loading, success, and recoverable error states instead
//! of leaving an old error visible while a second request runs. This exercise
//! uses an injected queue of responses so the check never needs a network.
//! The first request fails; the next succeeds after an executor-controlled
//! delay. Keep the Task in the view as in lesson 15.
//!
//! Goal: after Offline, pressing Retry should immediately show Loading and
//! then display the returned report. Clear the old error before retrying.
//!
//! Example — Representing request states explicitly:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! enum RequestState { Loading, Ready(String), Error(String) }
//! let label = match &self.request {
//!     RequestState::Loading => "Loading",
//!     RequestState::Ready(text) | RequestState::Error(text) => text.as_str(),
//! };
//! div().child(label.to_owned())
//! ```

use crate::theme::button;
use gpui_kit::{Context, IntoElement, Render, Task, Window, div, prelude::*};
use std::{collections::VecDeque, time::Duration};

#[derive(Clone, Debug, PartialEq, Eq)]
enum LoadState {
    Idle,
    Loading,
    Error(&'static str),
    Success(&'static str),
}

pub struct RetryPanel {
    responses: VecDeque<Result<&'static str, &'static str>>,
    state: LoadState,
    task: Option<Task<()>>,
}

impl Default for RetryPanel {
    fn default() -> Self {
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

    fn request(&mut self, cx: &mut Context<Self>) {
        let response = self.responses.pop_front().unwrap_or(Err("No response"));
        self.task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(Duration::from_secs(1)).await;
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

    // TODO: Retry must replace the error with Loading before it requests again.
    fn retry(&mut self, cx: &mut Context<Self>) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_26(cx: &mut TestAppContext) {
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
