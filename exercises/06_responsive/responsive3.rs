//! 18 — Keep a disabled control disabled
//!
//! Looking disabled is not the same as being disabled. While disabled, this
//! control already dims itself, drops its hover style and leaves the Tab order
//! (opacity, when(!disabled, hover), tab_index(-1)). Its handlers still run,
//! though: a click still arrives, and code can still focus the control and
//! send it Enter. Styling only tells the user; the view's state must decide.
//!
//! Goal: clicking or pressing Enter toggles selection while enabled, and does
//! nothing while disabled. The click and key handlers both call
//! toggle_selection, so one guard there covers every input. Try the control
//! with the mouse, Tab and Enter, then disable it and try again.
//!
//! Example — Guarding an action where every input path meets:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! fn submit(&mut self, cx: &mut Context<Self>) {
//!     if self.busy {
//!         return; // Ignore clicks and shortcuts alike while busy.
//!     }
//!     self.submitted += 1;
//!     cx.notify();
//! }
//! ```

use crate::theme::{button, colors, focus_ring};
use gpui_kit::{Context, FocusHandle, IntoElement, Render, Role, Window, div, prelude::*, px};

pub struct StatesPanel {
    focus: FocusHandle,
    selected: bool,
    disabled: bool,
    hovered: bool,
}

impl StatesPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus: cx.focus_handle(),
            selected: false,
            disabled: false,
            hovered: false,
        }
    }

    // Both the click handler and the Enter/Space handler below call this.
    // TODO: While `self.disabled` is true, leave the selection unchanged.
    fn toggle_selection(&mut self, cx: &mut Context<Self>) {
        self.selected = !self.selected;
        cx.notify();
    }
}

impl Render for StatesPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = colors();
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                div()
                    .id("state-toggle")
                    .debug_selector(|| "state-toggle".into())
                    .role(Role::Button)
                    .aria_label("Toggle selection")
                    .track_focus(&self.focus)
                    .tab_index(if self.disabled { -1 } else { 0 })
                    .w(px(224.0))
                    .h(px(72.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_lg()
                    .border_1()
                    .border_color(c.border)
                    .bg(if self.selected { c.primary } else { c.muted })
                    .text_color(if self.selected {
                        c.primary_foreground
                    } else {
                        c.foreground
                    })
                    // How the control *looks* in each state. None of this stops
                    // the handlers below from running.
                    .when(!self.disabled, |el| el.hover(|style| style.bg(c.accent)))
                    .focus_visible(focus_ring)
                    .when(self.disabled, |el| el.opacity(0.45))
                    .on_hover(cx.listener(|this, hovered: &bool, _, cx| {
                        if this.hovered != *hovered {
                            this.hovered = *hovered;
                            cx.notify();
                        }
                    }))
                    // Two ways to activate the control: the mouse and the keyboard.
                    .on_click(cx.listener(|this, _, _, cx| this.toggle_selection(cx)))
                    .on_key_down(cx.listener(|this, event: &gpui_kit::KeyDownEvent, _, cx| {
                        if event.keystroke.key == "enter" || event.keystroke.key == "space" {
                            this.toggle_selection(cx);
                            cx.stop_propagation();
                        }
                    }))
                    .child(if self.selected {
                        "Selected"
                    } else {
                        "Select me"
                    }),
            )
            .child(
                button(
                    "state-disable",
                    if self.disabled { "Enable" } else { "Disable" },
                    false,
                )
                .debug_selector(|| "state-disable".into())
                .on_click(cx.listener(|this, _, _, cx| {
                    this.disabled = !this.disabled;
                    cx.notify();
                })),
            )
            .child(
                div()
                    .debug_selector(|| {
                        format!(
                            "state-{}",
                            if self.disabled {
                                "disabled"
                            } else if self.hovered {
                                "hovered"
                            } else {
                                "idle"
                            }
                        )
                    })
                    .text_color(c.muted_foreground)
                    .child(if self.disabled {
                        "Disabled"
                    } else if self.hovered {
                        "Hovered"
                    } else {
                        "Ready"
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
    fn exercise_18(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, cx| StatesPanel::new(cx));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let toggle = cx.debug_bounds("state-toggle").expect("toggle missing");

        cx.simulate_mouse_move(toggle.center(), None, Modifiers::default());
        cx.update(|_, cx| assert!(panel.read(cx).hovered, "hover should be visible"));
        cx.simulate_click(toggle.center(), Modifiers::default());
        cx.update(|_, cx| assert!(panel.read(cx).selected, "click should select"));

        let disable = cx
            .debug_bounds("state-disable")
            .expect("disable button missing");
        cx.simulate_click(disable.center(), Modifiers::default());
        cx.update(|window, cx| {
            assert!(panel.read(cx).disabled);
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("state-disabled").is_some());
        cx.simulate_click(toggle.center(), Modifiers::default());
        cx.update(|_, cx| {
            assert!(
                panel.read(cx).selected,
                "disabled click must not change selection"
            )
        });

        let focus = cx.update(|_, cx| panel.read(cx).focus.clone());
        cx.update(|window, cx| window.focus(&focus, cx));
        cx.simulate_keystrokes("enter");
        cx.update(|_, cx| {
            assert!(
                panel.read(cx).selected,
                "disabled key must not change selection"
            )
        });

        cx.simulate_click(disable.center(), Modifiers::default());
        cx.update(|window, cx| window.focus(&focus, cx));
        cx.simulate_keystrokes("enter");
        cx.update(|_, cx| {
            assert!(
                !panel.read(cx).selected,
                "enabled key should toggle selection"
            )
        });
    }
}
