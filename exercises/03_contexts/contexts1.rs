//! 07 — Tell GPUI what changed
//!
//! Context<Self> is the app context plus the identity of this entity. The name
//! cx (or ctx) is just a variable name. Changing a Rust field does not announce
//! a change to GPUI: cx.notify() invalidates the view and informs its observers.
//!
//! Goal: switch on the signal and notify GPUI from the click handler.
//! Add the missing context call after changing active. An incidental repaint
//! from hovering is not a substitute for notification: other views also need it.
//! App manages entities; Context<T> knows which entity T is changing; Window
//! manages window-local things such as focus. The callback provides both.
//!
//! Example — Notifying after a field changes:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! fn rename(&mut self, name: String, cx: &mut Context<Self>) {
//!     self.name = name;
//!     cx.notify();
//! }
//! ```

use crate::theme::button;
use gpui_kit::{Context, IntoElement, Render, Window, div, prelude::*};

#[derive(Default)]
pub struct NotifyPanel {
    active: bool,
}

impl Render for NotifyPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                button("notify-enable", "Enable signal", true)
                    .debug_selector(|| "notify-enable".into())
                    .on_click(cx.listener(|this, _, _, _cx| {
                        this.active = true;
                        // TODO: Tell GPUI this entity changed.
                    })),
            )
            .child(if self.active {
                div()
                    .debug_selector(|| "notified-signal".into())
                    .child("Signal enabled")
            } else {
                div().child("Signal off")
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};
    use std::{cell::Cell, rc::Rc};

    #[gpui::test]
    fn exercise_07(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, _| NotifyPanel::default());
        let notified = Rc::new(Cell::new(false));
        let observed = notified.clone();
        let _subscription = cx.update(|_, cx| cx.observe(&panel, move |_, _| observed.set(true)));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let button = cx.debug_bounds("notify-enable").unwrap();
        cx.simulate_click(button.center(), Modifiers::default());
        cx.update(|_, cx| assert!(panel.read(cx).active, "the handler must update state"));
        assert!(
            notified.get(),
            "call notify on the changed entity's context; updating a field alone is not enough"
        );
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(cx.debug_bounds("notified-signal").is_some());
    }
}
