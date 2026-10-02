//! 30 — Share application state
//!
//! A GPUI Global belongs to the application, not to one view. A control can
//! change it through App, and another entity can read the same value in its
//! render method. To repaint when the global changes, that entity observes
//! the global and keeps the returned Subscription alive.
//!
//! Goal: clicking Toggle spacing switches both child views to Compact, then
//! back to Comfortable. The summary already reads the global; keep its
//! observer connected so changes trigger its render.
//!
//! Example — Observing an application-wide setting:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! let subscription = cx.observe_global::<Settings>(|_, cx| cx.notify());
//! let compact = cx.global::<Settings>().compact;
//! // Keep subscription in the view that renders compact.
//! ```

use crate::theme::button;
use gpui_kit::{
    Context, Entity, Global, IntoElement, Render, Subscription, Window, div, prelude::*,
};

// The shared setting. Implementing the marker trait `Global` lets the app store
// one value of this type, which any context can read with
// `cx.global::<SpacingSetting>()`. SharedPanel installs it below.
#[derive(Default)]
pub struct SpacingSetting {
    compact: bool,
}
impl Global for SpacingSetting {}

// The child that changes the setting.
pub struct SpacingControl;
impl Render for SpacingControl {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let compact = cx.global::<SpacingSetting>().compact;
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_2()
            .child(
                button("shared-toggle", "Toggle spacing", true)
                    .debug_selector(|| "shared-toggle".into())
                    .on_click(cx.listener(|_, _, _, cx| {
                        // `global_mut` borrows the global mutably and tells
                        // every observer of SpacingSetting that it changed.
                        let setting = cx.global_mut::<SpacingSetting>();
                        setting.compact = !setting.compact;
                        // This tells GPUI the control changed. Nothing tells
                        // the summary, except its observer.
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .debug_selector(move || format!("shared-control-{compact}"))
                    .child(if compact { "Compact" } else { "Comfortable" }),
            )
    }
}

// The child that only reads the setting.
pub struct SpacingSummary {
    // How many times the observer has run. The check reads it.
    updates: usize,
    _subscription: Option<Subscription>,
}
impl SpacingSummary {
    fn new(cx: &mut Context<Self>) -> Self {
        // `observe_global` runs this closure after every change to
        // SpacingSetting, with this summary (`this`) and its context. Like
        // `observe` and `subscribe`, it returns a Subscription: the observer
        // stays connected only as long as that value is kept.
        let subscription = cx.observe_global::<SpacingSetting>(|this, cx| {
            this.updates += 1;
            cx.notify();
        });
        Self {
            updates: 0,
            // TODO: With `None`, `subscription` is dropped when `new` returns,
            // which disconnects the observer: the summary is never told about a
            // change, and `updates` stays at 0. Keep it in the summary instead.
            _subscription: None,
        }
    }
}
impl Render for SpacingSummary {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Reading a global doesn't subscribe to it. Here the summary happens
        // to redraw with the control, because both are in one window. A view
        // in another window, or one GPUI caches, would keep the old value.
        let compact = cx.global::<SpacingSetting>().compact;
        div()
            .debug_selector(move || format!("shared-summary-{compact}"))
            .child(format!(
                "Summary: {}",
                if compact { "Compact" } else { "Comfortable" }
            ))
    }
}

pub struct SharedPanel {
    control: Entity<SpacingControl>,
    summary: Entity<SpacingSummary>,
}
impl SharedPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        // Install the global before the children read it in `render`.
        cx.set_global(SpacingSetting::default());
        Self {
            control: cx.new(|_| SpacingControl),
            summary: cx.new(SpacingSummary::new),
        }
    }
}
impl Render for SharedPanel {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(self.control.clone())
            .child(self.summary.clone())
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
        let (panel, cx) = cx.add_window_view(|_, cx| SharedPanel::new(cx));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let toggle = cx.debug_bounds("shared-toggle").unwrap();
        for (compact, updates) in [(true, 1), (false, 2)] {
            cx.simulate_click(toggle.center(), Modifiers::default());
            cx.update(|window, cx| {
                assert_eq!(cx.global::<SpacingSetting>().compact, compact);
                assert_eq!(
                    panel.read(cx).summary.read(cx).updates,
                    updates,
                    "the summary's observe_global callback should run once per toggle"
                );
                window.draw(cx).clear(cx);
            });
            let control = if compact {
                "shared-control-true"
            } else {
                "shared-control-false"
            };
            let summary = if compact {
                "shared-summary-true"
            } else {
                "shared-summary-false"
            };
            assert!(cx.debug_bounds(control).is_some());
            assert!(
                cx.debug_bounds(summary).is_some(),
                "the summary should show the new spacing"
            );
        }
    }
}
