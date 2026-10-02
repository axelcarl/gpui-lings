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

#[derive(Default)]
pub struct SpacingSetting {
    compact: bool,
}
impl Global for SpacingSetting {}

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
                        let setting = cx.global_mut::<SpacingSetting>();
                        setting.compact = !setting.compact;
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

pub struct SpacingSummary {
    updates: usize,
    _subscription: Option<Subscription>,
}
impl SpacingSummary {
    fn new(cx: &mut Context<Self>) -> Self {
        let _subscription = cx.observe_global::<SpacingSetting>(|this, cx| {
            this.updates += 1;
            cx.notify();
        });
        Self {
            updates: 0,
            _subscription: None,
        }
    }
}
impl Render for SpacingSummary {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                assert_eq!(panel.read(cx).summary.read(cx).updates, updates);
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
            assert!(cx.debug_bounds(summary).is_some());
        }
    }
}
