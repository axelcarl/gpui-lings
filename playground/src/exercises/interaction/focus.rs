//! 13 — Move keyboard focus deliberately
//!
//! FocusHandle identifies a place in the window's focus tree. track_focus
//! attaches it to an element. A Context can create the handle, but Window
//! decides where keyboard events go: window.focus(&handle, cx) moves focus.
//!
//! Goal: Focus pad must move focus to the key pad; pressing X then counts.
//! The pad is already attached to self.pad and handles X. Add the missing
//! Window call in the button's handler. Tab focus on the button itself is not
//! focus on the pad. Blurring the window must stop the pad receiving keys.

use crate::theme::{button, colors, focus_ring};
use gpui_kit::{Context, FocusHandle, IntoElement, Render, Window, div, prelude::*, px};

pub struct FocusPanel {
    pad: FocusHandle,
    presses: usize,
}
impl FocusPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            pad: cx.focus_handle(),
            presses: 0,
        }
    }
}
impl Render for FocusPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                button("focus-pad-button", "Focus pad", true)
                    .debug_selector(|| "focus-pad-button".into())
                    .on_click(cx.listener(|_this, _, _window, _cx| {
                        // TODO: Move focus to this.pad using the window.
                    })),
            )
            .child(
                div()
                    .id("key-pad")
                    .track_focus(&self.pad)
                    .debug_selector(|| "key-pad".into())
                    .w(px(220.0))
                    .p_4()
                    .border_1()
                    .border_color(colors().border)
                    .rounded_lg()
                    .text_center()
                    .focus(focus_ring)
                    .on_key_down(cx.listener(|this, event: &gpui_kit::KeyDownEvent, _, cx| {
                        if event.keystroke.key == "x" {
                            this.presses += 1;
                            cx.notify();
                            cx.stop_propagation();
                        }
                    }))
                    .child(format!("Press X · received {}", self.presses)),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_13(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, cx| FocusPanel::new(cx));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let button = cx.debug_bounds("focus-pad-button").unwrap();
        cx.simulate_click(button.center(), Modifiers::default());
        cx.update(|window, cx| {
            assert!(
                panel.read(cx).pad.is_focused(window),
                "focus the pad, not the button"
            )
        });
        cx.simulate_keystrokes("x x");
        cx.update(|_, cx| assert_eq!(panel.read(cx).presses, 2));
        cx.update(|window, cx| window.blur(cx));
        cx.simulate_keystrokes("x");
        cx.update(|_, cx| {
            assert_eq!(
                panel.read(cx).presses,
                2,
                "unfocused pads must not receive input"
            )
        });
    }
}
