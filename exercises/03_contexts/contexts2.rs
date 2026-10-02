//! 08 — Connect a handler to its view
//!
//! An element's on_click callback receives an event, Window, and App. It does
//! not receive your view. cx.listener adapts a method into that callback: GPUI
//! retrieves the entity and gives the method &mut Self and Context<Self>.
//!
//! Goal: wire the button to ListenerPanel::record_click using cx.listener.
//! The method already updates state and notifies. Replace the empty callback;
//! do not move the state into the element or capture &mut self across renders.
//! Each click should increase the displayed count once, including after redraw.
//!
//! Example — Adapting methods and closures with cx.listener:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! fn clear(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
//!     self.clicks = 0;
//!     cx.notify();
//! }
//! // Inside render, cx is &mut Context<Self>:
//! button("clear", "Clear", false).on_click(cx.listener(Self::clear))
//! // A closure gets the same four arguments (view, event, window, context):
//! button("clear-inline", "Clear", false).on_click(cx.listener(|this, _, _, cx| {
//!     this.clicks = 0;
//!     cx.notify();
//! }))
//! ```

use crate::theme::button;
use gpui_kit::{ClickEvent, Context, IntoElement, Render, Window, div, prelude::*};

#[derive(Default)]
pub struct ListenerPanel {
    clicks: usize,
}

impl ListenerPanel {
    pub fn record_click(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.clicks += 1;
        cx.notify();
    }
}

impl Render for ListenerPanel {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(format!("Handler calls: {}", self.clicks))
            .child(
                button("listener-record", "Call the handler", true)
                    .debug_selector(|| "listener-record".into())
                    // TODO: Adapt record_click into an element callback.
                    .on_click(|_, _, _| {}),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_08(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, _| ListenerPanel::default());
        for expected in 1..=2 {
            cx.update(|window, cx| window.draw(cx).clear(cx));
            let button = cx.debug_bounds("listener-record").unwrap();
            cx.simulate_click(button.center(), Modifiers::default());
            cx.update(|_, cx| {
                assert_eq!(
                    panel.read(cx).clicks,
                    expected,
                    "the handler should run once per click"
                )
            });
        }
    }
}
