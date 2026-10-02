//! 22 — Route a nested action
//!
//! Actions travel through the focused element's tree. A child can handle an
//! action locally or leave it for an ancestor. An action handler consumes the
//! action by default; a child that declines it must explicitly propagate it.
//!
//! Goal: Ctrl-R counts once in the child while local handling is enabled.
//! With local handling off, the same key should reach the parent. The action,
//! binding, focus handles, and fallback are already present. Let the action
//! continue only when the child declines it.
//!
//! Example — Letting an ancestor handle a command:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! fn save(&mut self, _: &Save, _: &mut Window, cx: &mut Context<Self>) {
//!     if self.read_only {
//!         cx.propagate();
//!         return;
//!     }
//!     self.save_document();
//! }
//! ```

use crate::theme::{button, colors};
use gpui_kit::{
    Context, FocusHandle, IntoElement, KeyBinding, Render, Window, actions, div, prelude::*,
};

actions!(gpui_lings_routes, [RouteCommand]);

pub struct PropagationPanel {
    parent_focus: FocusHandle,
    child_focus: FocusHandle,
    child_enabled: bool,
    parent_calls: usize,
    child_calls: usize,
}

impl PropagationPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        cx.bind_keys([KeyBinding::new("ctrl-r", RouteCommand, Some("RoutePanel"))]);
        Self {
            parent_focus: cx.focus_handle(),
            child_focus: cx.focus_handle(),
            child_enabled: true,
            parent_calls: 0,
            child_calls: 0,
        }
    }

    // TODO: When local handling is off, leave the action for the parent.
    fn route_in_child(&mut self, _: &RouteCommand, _: &mut Window, cx: &mut Context<Self>) {
        if self.child_enabled {
            self.child_calls += 1;
            cx.notify();
        }
    }

    fn route_in_parent(&mut self, _: &RouteCommand, _: &mut Window, cx: &mut Context<Self>) {
        self.parent_calls += 1;
        cx.notify();
    }
}

impl Render for PropagationPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = colors();
        div()
            .id("route-parent")
            .track_focus(&self.parent_focus)
            .key_context("RoutePanel")
            .on_action(cx.listener(Self::route_in_parent))
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                button("route-focus-parent", "Focus parent", false)
                    .debug_selector(|| "route-focus-parent".into())
                    .on_click(cx.listener(|this, _, window, cx| {
                        window.focus(&this.parent_focus, cx);
                    })),
            )
            .child(
                div()
                    .id("route-child")
                    .track_focus(&self.child_focus)
                    .on_action(cx.listener(Self::route_in_child))
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_3()
                    .p_4()
                    .border_1()
                    .border_color(c.border)
                    .rounded_lg()
                    .child(
                        button("route-focus-child", "Focus child", true)
                            .debug_selector(|| "route-focus-child".into())
                            .on_click(cx.listener(|this, _, window, cx| {
                                window.focus(&this.child_focus, cx);
                            })),
                    )
                    .child(
                        button(
                            "route-toggle-child",
                            if self.child_enabled {
                                "Use parent"
                            } else {
                                "Use child"
                            },
                            false,
                        )
                        .debug_selector(|| "route-toggle-child".into())
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.child_enabled = !this.child_enabled;
                            cx.notify();
                        })),
                    ),
            )
            .child(
                div()
                    .debug_selector(|| {
                        format!("route-counts-{}-{}", self.child_calls, self.parent_calls)
                    })
                    .child(format!(
                        "Child: {} · Parent: {}",
                        self.child_calls, self.parent_calls
                    )),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_22(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, cx| PropagationPanel::new(cx));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let child = cx
            .debug_bounds("route-focus-child")
            .expect("child focus missing");
        cx.simulate_click(child.center(), Modifiers::default());
        cx.simulate_keystrokes("ctrl-r");
        cx.update(|_, cx| {
            assert_eq!(
                panel.read(cx).child_calls,
                1,
                "child should handle the command"
            );
            assert_eq!(panel.read(cx).parent_calls, 0, "parent must not repeat it");
        });

        let toggle = cx
            .debug_bounds("route-toggle-child")
            .expect("toggle missing");
        cx.simulate_click(toggle.center(), Modifiers::default());
        cx.simulate_click(child.center(), Modifiers::default());
        cx.simulate_keystrokes("ctrl-r");
        cx.update(|_, cx| {
            assert_eq!(panel.read(cx).child_calls, 1);
            assert_eq!(
                panel.read(cx).parent_calls,
                1,
                "parent should handle fallback"
            );
        });

        let parent = cx
            .debug_bounds("route-focus-parent")
            .expect("parent focus missing");
        cx.simulate_click(parent.center(), Modifiers::default());
        cx.simulate_keystrokes("ctrl-r");
        cx.update(|_, cx| assert_eq!(panel.read(cx).parent_calls, 2));
    }
}
