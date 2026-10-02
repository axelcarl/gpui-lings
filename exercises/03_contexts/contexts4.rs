//! 10 — Observe a model
//!
//! An entity does not have to render anything. Reading stores data; its parent
//! view observes it. cx.observe runs when that model calls notify. It signals
//! that something changed, without an event payload: read the current model.
//!
//! Goal: make the mirrored reading follow the model, including external updates.
//! In the observer, read Reading through the supplied entity and copy its value
//! into mirrored. The stored Subscription keeps the observer connected. Call
//! notify on the observer's context so the parent also refreshes.
//!
//! Example — Observing another entity:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! let subscription = cx.observe(&model, |this, model, cx| {
//!     this.label = model.read(cx).label.clone();
//!     cx.notify();
//! });
//! // Store subscription in the view so this observer remains active.
//! ```

use crate::theme::button;
use gpui_kit::{Context, Entity, IntoElement, Render, Subscription, Window, div, prelude::*};

#[derive(Default)]
pub struct Reading {
    value: usize,
}
pub struct ObservePanel {
    reading: Entity<Reading>,
    mirrored: usize,
    _observation: Subscription,
}
impl ObservePanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let reading = cx.new(|_| Reading::default());
        let observation = cx.observe(&reading, |this, _reading, cx| {
            // TODO: Read the new value from the entity that notified us.
            this.mirrored = 0;
            cx.notify();
        });
        Self {
            reading,
            mirrored: 0,
            _observation: observation,
        }
    }
}
impl Render for ObservePanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                div()
                    .debug_selector(|| format!("mirrored-{}", self.mirrored))
                    .child(format!("Mirrored reading: {}", self.mirrored)),
            )
            .child(
                button("observe-increment", "Update model", true)
                    .debug_selector(|| "observe-increment".into())
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.reading.update(cx, |reading, cx| {
                            reading.value += 1;
                            cx.notify();
                        });
                    })),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_10(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, cx| ObservePanel::new(cx));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let button = cx.debug_bounds("observe-increment").unwrap();
        cx.simulate_click(button.center(), Modifiers::default());
        cx.update(|_, cx| {
            assert_eq!(
                panel.read(cx).mirrored,
                1,
                "observe must read the model's value"
            )
        });
        let reading = cx.update(|_, cx| panel.read(cx).reading.clone());
        reading.update(cx, |reading, cx| {
            reading.value = 7;
            cx.notify();
        });
        cx.update(|window, cx| {
            assert_eq!(
                panel.read(cx).mirrored,
                7,
                "updates can originate outside the button"
            );
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("mirrored-7").is_some());
    }
}
