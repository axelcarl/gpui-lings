//! 11 — Keep an event subscription alive
//!
//! notify means "this entity changed"; emit sends a typed event with a payload.
//! EventEmitter<E> declares which event an entity emits. A parent can subscribe
//! without teaching the child anything about its parent. Dropping Subscription
//! disconnects the callback; store it for the lifetime of the receiving view.
//!
//! Goal: each Send signal click delivers its sequence number to the parent.
//! The child already emits Signal and the callback already handles it. Keep the
//! returned subscription in _subscription instead of dropping it. Observe is
//! not a substitute: it cannot carry the Signal payload.
//!
//! Example — Sending and receiving typed events:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! impl EventEmitter<Message> for Model {}
//! // Inside a Model update:
//! cx.emit(Message { text: "Hello".into() });
//! // Inside the receiving view constructor:
//! let subscription = cx.subscribe(&model, |this, _, event: &Message, cx| {
//!     this.label = event.text.clone();
//!     cx.notify();
//! });
//! // Store subscription in a field of the receiving view.
//! ```

use crate::theme::button;
use gpui_kit::{
    Context, Entity, EventEmitter, IntoElement, Render, Subscription, Window, div, prelude::*,
};

pub struct Signal(pub usize);
#[derive(Default)]
pub struct Sender {
    sent: usize,
}
impl EventEmitter<Signal> for Sender {}
impl Render for Sender {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        button("event-send", "Send signal", true)
            .debug_selector(|| "event-send".into())
            .on_click(cx.listener(|this, _, _, cx| {
                this.sent += 1;
                cx.emit(Signal(this.sent));
                cx.notify();
            }))
    }
}

pub struct EventsPanel {
    sender: Entity<Sender>,
    received: usize,
    _subscription: Option<Subscription>,
}
impl EventsPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let sender = cx.new(|_| Sender::default());
        let subscription = cx.subscribe(&sender, |this, _, signal: &Signal, cx| {
            this.received = signal.0;
            cx.notify();
        });
        // TODO: Keep this connection alive in the panel.
        drop(subscription);
        Self {
            sender,
            received: 0,
            _subscription: None,
        }
    }
}
impl Render for EventsPanel {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(self.sender.clone())
            .child(
                div()
                    .debug_selector(|| format!("received-{}", self.received))
                    .child(format!("Parent received: {}", self.received)),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_11(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, cx| EventsPanel::new(cx));
        for expected in 1..=2 {
            cx.update(|window, cx| window.draw(cx).clear(cx));
            let button = cx.debug_bounds("event-send").unwrap();
            cx.simulate_click(button.center(), Modifiers::default());
            cx.update(|_, cx| {
                assert_eq!(
                    panel.read(cx).received,
                    expected,
                    "retain the subscription to receive the child's event"
                )
            });
        }
        let sender = cx.update(|_, cx| panel.read(cx).sender.clone());
        sender.update(cx, |_, cx| cx.emit(Signal(42)));
        cx.update(|window, cx| {
            assert_eq!(
                panel.read(cx).received,
                42,
                "use the payload, not a local click count"
            );
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("received-42").is_some());
    }
}
