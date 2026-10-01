//! 35 — Test behavior through GPUI
//!
//! This view already works. This time the exercise is the test below. Drive it
//! through GPUI rather than calling the view's private method: render the
//! preview, click Load, assert the intermediate state, advance the simulated
//! clock, and assert the rendered result. No real sleep is needed.
//!
//! Goal: make exercise_35 pass by completing its interaction test. Leave the
//! view behavior in place. The starter test fails because it never clicks Load.

use crate::theme::button;
use gpui_kit::{Context, IntoElement, Render, Task, Window, div, prelude::*};
use std::time::Duration;

#[derive(Default)]
pub struct BehaviorTestPanel {
    loading: bool,
    loaded: bool,
    task: Option<Task<()>>,
}

impl BehaviorTestPanel {
    fn load(&mut self, cx: &mut Context<Self>) {
        self.loading = true;
        self.loaded = false;
        cx.notify();
        self.task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor().timer(Duration::from_secs(1)).await;
            let _ = this.update(cx, |this, cx| {
                this.loading = false;
                this.loaded = true;
                cx.notify();
            });
        }));
    }
}

impl Render for BehaviorTestPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let status = if self.loading {
            "Loading"
        } else if self.loaded {
            "Ready"
        } else {
            "Idle"
        };
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                button("behavior-load", "Load", true)
                    .debug_selector(|| "behavior-load".into())
                    .on_click(cx.listener(|this, _, _, cx| this.load(cx))),
            )
            .child(
                div()
                    .debug_selector(|| format!("behavior-{status}"))
                    .child(status),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    // TODO: Click Load, and advance the simulated clock by one second at the right
    // point, so that each assertion sees the state it expects.
    #[gpui::test]
    fn exercise_35(cx: &mut TestAppContext) {
        let (panel, window) = cx.add_window_view(|_, _| BehaviorTestPanel::default());
        window.update(|window, cx| window.draw(cx).clear(cx));
        let load = window.debug_bounds("behavior-load").unwrap();

        let _ = (load, Modifiers::default());

        window.update(|window, cx| {
            assert!(panel.read(cx).loading, "Load should show Loading first");
            window.draw(cx).clear(cx);
        });
        assert!(window.debug_bounds("behavior-Loading").is_some());

        window.update(|window, cx| {
            assert!(panel.read(cx).loaded, "the simulated result should arrive");
            window.draw(cx).clear(cx);
        });
        assert!(window.debug_bounds("behavior-Ready").is_some());
    }
}
