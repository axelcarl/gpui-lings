// An action starts at the focused element and travels up through its parents
// until a handler takes it. Here, the child panel handles `RouteCommand`
// itself, and the parent handles it as a fallback. Running a handler
// *consumes* the action by default, so it stops there. A handler that decides
// not to deal with it has to say so with `cx.propagate()`, and then the action
// continues to the next handler up.
//
// Focus the child and press Ctrl-R. Then press Use parent and try again: now
// the parent should count it.

use crate::theme::{button, colors};
use gpui_kit::{
    Context, FocusHandle, IntoElement, KeyBinding, Render, Window, actions, div, prelude::*,
};

// Declares the RouteCommand action, as in lesson 15.
actions!(gpui_lings_routes, [RouteCommand]);

pub struct PropagationPanel {
    parent_focus: FocusHandle,
    child_focus: FocusHandle,
    // Whether the child handles RouteCommand itself. The Use parent / Use child
    // button flips it.
    child_enabled: bool,
    // How many times each handler has handled the action.
    parent_calls: usize,
    child_calls: usize,
}

impl PropagationPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        // Ctrl-R dispatches RouteCommand while focus is anywhere inside the
        // "RoutePanel" key context, which the parent element sets in render.
        cx.bind_keys([KeyBinding::new("ctrl-r", RouteCommand, Some("RoutePanel"))]);
        Self {
            parent_focus: cx.focus_handle(),
            child_focus: cx.focus_handle(),
            child_enabled: true,
            parent_calls: 0,
            child_calls: 0,
        }
    }

    // Action handlers receive the action, the window and this view's context.
    // An action travels from the focused element up through its ancestors.
    // While focus is inside the child, the child's handler gets it first.
    fn route_in_child(&mut self, _: &RouteCommand, _: &mut Window, cx: &mut Context<Self>) {
        // TODO: With local handling off, this handler does nothing, but the
        // action still stops here. In that case, let it continue to the parent.
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
            // The context the binding needs. It sits on the parent, so Ctrl-R
            // matches whether the parent or the child has focus.
            .key_context("RoutePanel")
            // The fallback: runs when the parent itself has focus, or when the
            // child lets the action continue.
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
                    // Nearer the focus than the parent, so it runs first.
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

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_27(cx: &mut TestAppContext) {
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
