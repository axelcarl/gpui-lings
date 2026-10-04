// You've already met `cx` in the listeners of lessons 02 and 05. It's a
// `Context<Self>`: access to the whole app, plus the identity of the entity
// being updated. (`cx` is only a name. Some code calls it `ctx`, and it's the
// same type.)
//
// Changing a field on your view doesn't tell GPUI anything. GPUI renders a view
// again when it's told the view changed, and `cx.notify()` is how you tell it.
// Other entities that observe this one hear about the change through the same
// call, as you'll see in lesson 10.
//
// In the preview, the signal may still show up when you move the mouse, because
// hovering happens to redraw the window. Don't rely on that: observers and
// views elsewhere only hear about a change through `notify`.

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
                    // The listener's arguments: this view (`this`), the click
                    // event, the window and `cx`, this view's `Context<NotifyPanel>`.
                    .on_click(cx.listener(|this, _, _, cx| {
                        // Changing a field doesn't redraw anything by itself.
                        this.active = true;
                        // TODO: Tell GPUI this entity changed, so it renders again.
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

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
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
