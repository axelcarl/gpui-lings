// Once an element has focus, it hears the keys you press. `on_key_down` runs a
// handler for each one, with a `KeyDownEvent` that says which key it was:
// `event.keystroke.key` is a name such as "a", "enter", "left" or "right".
// Keys the element doesn't handle carry on to its parents.
//
// This stepper takes focus when you click it. Then the left and right arrow
// keys should move its value down and up, but nothing handles them yet. Write
// that handler.

use crate::theme::{colors, focus_ring};
use gpui_kit::{
    Context, FocusHandle, IntoElement, KeyDownEvent, Render, Window, div, prelude::*, px,
};

pub struct StepperPanel {
    focus: FocusHandle,
    value: u32,
}

impl StepperPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus: cx.focus_handle(),
            value: 0,
        }
    }
}

impl Render for StepperPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focused = self.focus.is_focused(window);
        div()
            .id("stepper")
            .debug_selector(|| "stepper".into())
            .track_focus(&self.focus)
            // Clicking the stepper focuses it, the way lesson 13's button did.
            .on_click(cx.listener(|this, _, window, cx| window.focus(&this.focus, cx)))
            .w(px(240.0))
            .p_4()
            .flex()
            .flex_col()
            .items_center()
            .gap_2()
            .rounded_lg()
            .border_1()
            .border_color(colors().border)
            .bg(colors().card)
            .focus(focus_ring)
            // Runs for every key pressed while the stepper has focus.
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                // TODO: Nothing happens yet. Make "left" lower the value by one,
                // stopping at zero, and "right" raise it by one. Notify after
                // a change, and leave every other key alone.
            }))
            .child(
                div()
                    .debug_selector(|| format!("stepper-value-{}", self.value))
                    .text_size(px(36.0))
                    .child(self.value.to_string()),
            )
            .child(if focused {
                "Focused · press ← or →"
            } else {
                "Click to focus"
            })
    }
}

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_14(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, cx| StepperPanel::new(cx));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let stepper = cx.debug_bounds("stepper").expect("stepper missing");
        cx.simulate_click(stepper.center(), Modifiers::default());
        cx.update(|window, cx| {
            assert!(
                panel.read(cx).focus.is_focused(window),
                "clicking should focus the stepper"
            )
        });

        cx.simulate_keystrokes("right right right");
        cx.update(|_, cx| {
            assert_eq!(
                panel.read(cx).value,
                3,
                "the right arrow should raise the value"
            )
        });
        cx.simulate_keystrokes("left");
        cx.update(|_, cx| {
            assert_eq!(
                panel.read(cx).value,
                2,
                "the left arrow should lower the value"
            )
        });
        cx.simulate_keystrokes("left left left");
        cx.update(|_, cx| assert_eq!(panel.read(cx).value, 0, "the value should stop at zero"));
        cx.simulate_keystrokes("right x up");
        cx.update(|window, cx| {
            assert_eq!(
                panel.read(cx).value,
                1,
                "other keys shouldn't change the value"
            );
            window.draw(cx).clear(cx);
        });
        assert!(
            cx.debug_bounds("stepper-value-1").is_some(),
            "the stepper should show its new value"
        );
    }
}
