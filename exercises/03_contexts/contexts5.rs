// `notify` says "I changed". Sometimes the receiver needs to know *what*
// happened, too. An entity declares each kind of event it emits with
// `impl EventEmitter<Signal> for Sender {}`, and sends one with `cx.emit(...)`.
// Anyone can listen with `cx.subscribe`, and the sender never needs to know
// who is listening. A subscription's callback names the one event type it
// wants, so a view that cares about two kinds of event subscribes twice.
//
// Like `observe`, `subscribe` returns a `Subscription`, and dropping it
// disconnects the callback. The panel below subscribes to `Signal` events but
// drops the subscription, and it doesn't listen for `Cleared` at all.

use crate::theme::button;
use gpui_kit::{
    Context, Entity, EventEmitter, IntoElement, Render, Subscription, Window, div, prelude::*,
};

// Sent by Send signal, with how many signals have been sent so far.
pub struct Signal(pub usize);
// Sent by Clear. It carries nothing: the event itself is the news.
pub struct Cleared;

#[derive(Default)]
pub struct Sender {
    sent: usize,
}
// Declares the events Sender emits, which allows `cx.emit(Signal(..))` and
// `cx.emit(Cleared)`.
impl EventEmitter<Signal> for Sender {}
impl EventEmitter<Cleared> for Sender {}
impl Render for Sender {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap_2()
            .child(
                button("event-send", "Send signal", true)
                    .debug_selector(|| "event-send".into())
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.sent += 1;
                        cx.emit(Signal(this.sent));
                        cx.notify();
                    })),
            )
            .child(
                button("event-clear", "Clear", false)
                    .debug_selector(|| "event-clear".into())
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.sent = 0;
                        cx.emit(Cleared);
                        cx.notify();
                    })),
            )
    }
}

pub struct EventsPanel {
    sender: Entity<Sender>,
    received: usize,
    _subscriptions: Vec<Subscription>,
}
impl EventsPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let sender = cx.new(|_| Sender::default());
        // `subscribe` connects this panel to the sender's `Signal` events and
        // returns a Subscription. The connection lasts as long as that value.
        let signals = cx.subscribe(&sender, |this, _, signal: &Signal, cx| {
            this.received = signal.0;
            cx.notify();
        });
        // TODO: Clear empties the sender, but the panel never hears about it.
        // Subscribe to `Cleared` events too, and set `received` back to 0.
        Self {
            sender,
            received: 0,
            // TODO: Nothing is kept here, so every subscription is dropped when
            // `new` returns, which disconnects it. Keep them all in the panel.
            _subscriptions: Vec::new(),
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

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
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
                    "keep the Signal subscription to receive the child's event"
                )
            });
        }
        let sender = cx.update(|_, cx| panel.read(cx).sender.clone());
        sender.update(cx, |_, cx| cx.emit(Signal(42)));
        cx.update(|_, cx| {
            assert_eq!(
                panel.read(cx).received,
                42,
                "use the payload, not a local click count"
            )
        });

        let clear = cx.debug_bounds("event-clear").unwrap();
        cx.simulate_click(clear.center(), Modifiers::default());
        cx.update(|window, cx| {
            assert_eq!(
                panel.read(cx).received,
                0,
                "subscribe to Cleared, and set received back to 0"
            );
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("received-0").is_some());
    }
}
