//! 12 — Route a keyboard action
//!
//! A KeyBinding maps a keystroke to a named Action. The binding's context is a
//! predicate over the focused element's ancestor path. on_action handles the
//! resulting command. This separates "what to do" from "which key was pressed".
//!
//! Goal: focus the shortcut area, then use Ctrl-K to toggle its signal.
//! Match the surface's key_context to the context used by the KeyBinding.
//! The handler and focus button already work. Keep the binding scoped so it
//! does not fire when focus is elsewhere in the app.

use crate::theme::button;
use gpui_kit::{
    Context, FocusHandle, IntoElement, KeyBinding, Render, Window, actions, div, prelude::*,
};

actions!(gpui_lings_commands, [ToggleSignal]);

pub struct ActionsPanel {
    focus: FocusHandle,
    active: bool,
}
impl ActionsPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
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
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("action-surface")
            .track_focus(&self.focus)
            // TODO: Match the key binding's context.
            .key_context("OtherPanel")
            .on_action(cx.listener(Self::toggle))
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                button("action-focus", "Focus shortcut area", true)
                    .debug_selector(|| "action-focus".into())
                    .on_click(cx.listener(|this, _, window, cx| window.focus(&this.focus, cx))),
            )
            .child("Then press Ctrl-K")
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_12(cx: &mut TestAppContext) {
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
