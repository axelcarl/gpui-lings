// Keyboard input goes to whichever element has *focus*. A `FocusHandle` names
// one place in the window that can have focus, and `track_focus` attaches it to
// an element. Creating a handle doesn't focus anything, though. Focus belongs
// to the window, so you move it with `window.focus(&handle, cx)`.
//
// The pad below counts the X key, but only while it has focus itself. Pressing
// Tab can focus the Focus pad button, and that isn't the pad. Every save opens
// a fresh preview window with nothing focused, so click Focus pad again after
// each change.

use crate::theme::{button, colors, focus_ring};
use gpui_kit::{Context, FocusHandle, IntoElement, Render, Window, div, prelude::*, px};

pub struct FocusPanel {
    // Created by the context; attached to the key pad with track_focus below.
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focused = self.pad.is_focused(window);
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                button("focus-pad-button", "Focus pad", true)
                    .debug_selector(|| "focus-pad-button".into())
                    .on_click(cx.listener(|this, _, window, cx| {
                        // TODO: Move keyboard focus to the pad, so it receives X.
                        // This listener gets the `window` that can do that.
                    })),
            )
            .child(
                div()
                    .id("key-pad")
                    .track_focus(&self.pad)
                    .debug_selector(|| "key-pad".into())
                    .w(px(280.0))
                    .p_4()
                    .border_1()
                    .border_color(colors().border)
                    .rounded_lg()
                    .text_center()
                    // Draw a ring while the pad has focus.
                    .bg(colors().card)
                    .focus(focus_ring)
                    // Key events go to the focused element, then its ancestors.
                    .on_key_down(cx.listener(|this, event: &gpui_kit::KeyDownEvent, _, cx| {
                        if event.keystroke.key == "x" {
                            this.presses += 1;
                            cx.notify();
                            cx.stop_propagation();
                        }
                    }))
                    .child(format!("Press X · received {}", self.presses))
                    .child(if focused {
                        "Pad focused"
                    } else {
                        "Pad not focused · click Focus pad"
                    }),
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
