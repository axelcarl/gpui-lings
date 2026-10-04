// In lesson 15, the Ctrl-L binding named no key context, so it matched
// wherever focus was. Often a shortcut should only work in one part of the
// window. A binding can name a *key context*, and then it only matches while
// the focused element, or one of its parents, has that `key_context`. That
// way Ctrl-K can mean one thing here and something else elsewhere in the app.
//
// Click "Focus shortcut area" before you press Ctrl-K; the preview says
// whether the area has focus.

use crate::theme::{button, colors, focus_ring};
use gpui_kit::{
    Context, FocusHandle, IntoElement, KeyBinding, Render, Window, actions, div, prelude::*, px,
};

// Declares an action: a named command that a key binding can trigger.
actions!(gpui_lings_commands, [ToggleSignal]);

pub struct ActionsPanel {
    // Identifies the shortcut area in the window's focus tree.
    focus: FocusHandle,
    active: bool,
}
impl ActionsPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        // Ctrl-K dispatches ToggleSignal, but only while focus is inside an
        // element whose key context is "CommandPanel".
        cx.bind_keys([KeyBinding::new(
            "ctrl-k",
            ToggleSignal,
            Some("CommandPanel"),
        )]);
        Self {
            focus: cx.focus_handle(),
            active: false,
        }
    }
    fn toggle(&mut self, _: &ToggleSignal, _: &mut Window, cx: &mut Context<Self>) {
        self.active = !self.active;
        cx.notify();
    }
}
impl Render for ActionsPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focused = self.focus.contains_focused(window, cx);
        div()
            .id("action-surface")
            .w(px(360.0))
            // Attach the focus handle. Keyboard events and actions travel from
            // the focused element up through its ancestors, checking their key
            // contexts against the binding.
            .track_focus(&self.focus)
            // TODO: The binding only matches inside "CommandPanel". Give the
            // surface the key context the binding expects.
            .key_context("OtherPanel")
            // When ToggleSignal reaches this element, call Self::toggle.
            .on_action(cx.listener(Self::toggle))
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .p_6()
            .rounded_xl()
            .border_1()
            .border_color(colors().border)
            .bg(colors().card)
            .focus(focus_ring)
            .child(
                button("action-focus", "Focus shortcut area", true)
                    .debug_selector(|| "action-focus".into())
                    .on_click(cx.listener(|this, _, window, cx| window.focus(&this.focus, cx))),
            )
            .child(if focused {
                "Focus is in the shortcut area · press Ctrl-K"
            } else {
                "Not focused · click the button first"
            })
            .child(
                div()
                    .debug_selector(|| {
                        if self.active {
                            "action-on"
                        } else {
                            "action-off"
                        }
                        .into()
                    })
                    .child(if self.active {
                        "Signal on"
                    } else {
                        "Signal off"
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
    fn exercise_16(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, cx| ActionsPanel::new(cx));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let button = cx.debug_bounds("action-focus").unwrap();
        cx.simulate_click(button.center(), Modifiers::default());
        cx.simulate_keystrokes("ctrl-k");
        cx.update(|window, cx| {
            assert!(
                panel.read(cx).active,
                "the focused key context must route ToggleSignal"
            );
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("action-on").is_some());
        cx.simulate_keystrokes("ctrl-k");
        cx.update(|_, cx| assert!(!panel.read(cx).active));
        cx.update(|window, cx| window.blur(cx));
        cx.simulate_keystrokes("ctrl-k");
        cx.update(|_, cx| {
            assert!(
                !panel.read(cx).active,
                "the shortcut must remain scoped to its focus context"
            )
        });
    }
}
