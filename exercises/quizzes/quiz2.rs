//! 24 — Quiz 2: command menu
//!
//! This is a quiz for the following lessons:
//! - 07 Contexts & notify
//! - 12–13 Actions & focus
//! - 21–23 Deeper contexts & input
//!
//! A small command menu combines actions, focus, keyboard routing, and a
//! cancel path. Ctrl-P opens it only while this view has focus. Arrow keys
//! move the highlighted command, Enter chooses it, and Escape closes without
//! changing the previous choice. Both paths return focus to the launcher.
//!
//! Nothing marks the broken lines. Testers reported:
//!
//! - The highlight doesn't follow the arrow keys, though Enter picks the
//!   right command.
//! - Ctrl-P does nothing while the launcher has focus.
//! - After Escape, Ctrl-P stops working until you click the launcher.
//!
//! Goal: find and fix all three. The check stops at the first symptom it
//! sees; reproduce it in the preview using only the keyboard, then trace it
//! back to the code. Each fix reuses an idea from lessons 07–21.
//!
//! Example — Handling a key through the view context:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! div().track_focus(&self.menu)
//!     .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
//!         if event.keystroke.key == "escape" {
//!             this.dismiss(window, cx);
//!         }
//!     }))
//! ```

use crate::theme::{button, colors, focus_ring};
use gpui_kit::{
    Context, FocusHandle, IntoElement, KeyBinding, Render, Window, actions, div, prelude::*, px,
};

actions!(gpui_lings_menu, [OpenCommands]);

pub struct MenuPanel {
    launcher: FocusHandle,
    menu: FocusHandle,
    open: bool,
    selected: usize,
    chosen: Option<&'static str>,
}

impl MenuPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        cx.bind_keys([KeyBinding::new(
            "ctrl-p",
            OpenCommands,
            Some("CommandMenuDemo"),
        )]);
        Self {
            launcher: cx.focus_handle(),
            menu: cx.focus_handle(),
            open: false,
            selected: 0,
            chosen: None,
        }
    }

    fn open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.open = true;
        self.selected = 0;
        window.focus(&self.menu, cx);
        cx.notify();
    }

    fn choose(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.chosen = Some(["First", "Second"][self.selected]);
        self.open = false;
        window.focus(&self.launcher, cx);
        cx.notify();
    }

    fn cancel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.open = false;
        window.focus(&self.menu, cx);
        cx.notify();
    }
}

impl Render for MenuPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = colors();
        div()
            .id("command-demo")
            .on_action(cx.listener(|this, _: &OpenCommands, window, cx| {
                this.open(window, cx);
            }))
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                button("menu-launcher", "Open commands", true)
                    .track_focus(&self.launcher)
                    .debug_selector(|| "menu-launcher".into())
                    .on_click(cx.listener(|this, _, window, cx| {
                        window.focus(&this.launcher, cx);
                        this.open(window, cx);
                    })),
            )
            .when(self.open, |panel| {
                panel.child(
                    div()
                        .id("command-menu")
                        .key_context("CommandMenuDemo")
                        .track_focus(&self.menu)
                        .debug_selector(|| "command-menu".into())
                        .w(px(200.0))
                        .p_2()
                        .border_1()
                        .border_color(c.border)
                        .rounded_lg()
                        .focus(focus_ring)
                        .on_key_down(cx.listener(
                            |this, event: &gpui_kit::KeyDownEvent, window, cx| {
                                match event.keystroke.key.as_str() {
                                    "down" => this.selected = (this.selected + 1).min(1),
                                    "up" => this.selected = this.selected.saturating_sub(1),
                                    "enter" => this.choose(window, cx),
                                    "escape" => this.cancel(window, cx),
                                    _ => return,
                                }
                                cx.stop_propagation();
                            },
                        ))
                        .children(["First", "Second"].into_iter().enumerate().map(
                            |(index, label)| {
                                div()
                                    .debug_selector(move || format!("menu-option-{index}"))
                                    .p_2()
                                    .when(self.selected == index, |el| {
                                        el.bg(c.primary).text_color(c.primary_foreground)
                                    })
                                    .child(label)
                            },
                        )),
                )
            })
            .child(
                div()
                    .debug_selector(|| format!("menu-chosen-{}", self.chosen.unwrap_or("None")))
                    .child(format!("Chosen: {}", self.chosen.unwrap_or("None"))),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};
    use std::{cell::Cell, rc::Rc};

    #[gpui::test]
    fn exercise_24(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, cx| MenuPanel::new(cx));
        let notified = Rc::new(Cell::new(false));
        let observed = notified.clone();
        let _subscription = cx.update(|_, cx| cx.observe(&panel, move |_, _| observed.set(true)));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let launcher = cx.debug_bounds("menu-launcher").expect("launcher missing");
        cx.simulate_click(launcher.center(), Modifiers::default());
        cx.update(|window, cx| {
            assert!(panel.read(cx).menu.is_focused(window));
            assert!(panel.read(cx).open);
        });
        notified.set(false);
        cx.simulate_keystrokes("down");
        cx.update(|_, cx| assert_eq!(panel.read(cx).selected, 1));
        assert!(notified.get(), "The highlight should follow the arrow keys");
        cx.simulate_keystrokes("enter");
        cx.update(|window, cx| {
            assert_eq!(panel.read(cx).chosen, Some("Second"));
            assert!(!panel.read(cx).open);
            assert!(panel.read(cx).launcher.is_focused(window));
        });

        cx.simulate_keystrokes("ctrl-p");
        cx.update(|window, cx| {
            assert!(
                panel.read(cx).menu.is_focused(window),
                "Ctrl-P should open the menu while the launcher has focus"
            )
        });
        cx.simulate_keystrokes("escape");
        cx.update(|window, cx| {
            assert!(!panel.read(cx).open);
            assert_eq!(panel.read(cx).chosen, Some("Second"));
            assert!(
                panel.read(cx).launcher.is_focused(window),
                "Escape should return focus to the launcher"
            );
        });
        cx.simulate_keystrokes("ctrl-p");
        cx.update(|window, cx| {
            assert!(
                panel.read(cx).menu.is_focused(window),
                "Ctrl-P should reopen the menu after Escape"
            )
        });
    }
}
