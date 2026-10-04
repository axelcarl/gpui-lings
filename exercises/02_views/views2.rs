// So far the playground has owned all the state. Now `TogglePanel` is a view of
// its own: a struct that implements `Render`. GPUI keeps it in an *entity* and
// calls its `render` whenever it needs to draw it, so the panel keeps its
// `active` flag from one frame to the next. Its parent only holds a handle to
// it, and doesn't need to know how the toggle works.
//
// Click the button a few times in the preview. The first click should show the
// signal, and the second should hide it again.

use crate::theme::{badge, button, colors};
use gpui_kit::{Context, IntoElement, Render, Window, div, prelude::*, px};

// A view is a struct that implements `Render`. GPUI keeps it in an entity and
// calls `render` whenever it needs the view's element tree.
#[derive(Default)]
pub struct TogglePanel {
    active: bool,
}

impl TogglePanel {
    pub fn new() -> Self {
        Self { active: false }
    }
}

impl Render for TogglePanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .items_center()
            .child(
                div()
                    .text_color(colors().muted_foreground)
                    .child("A little state, owned by a child view"),
            )
            .child(
                button(
                    "toggle-button",
                    if self.active {
                        "Hide the signal"
                    } else {
                        "Show the signal"
                    },
                    true,
                )
                .debug_selector(|| "toggle-button".into())
                .w(px(190.0))
                // `cx.listener` runs this closure on every click, with this
                // entity's state (`this`) and its context (`cx`).
                .on_click(cx.listener(|this, _, _, cx| {
                    // TODO: This always switches the signal off. Flip `active`
                    // instead, so clicks alternate between showing and hiding it.
                    this.active = false;
                    cx.notify(); // Ask GPUI to render this entity again.
                })),
            )
            // `when` adds the badge only while `active` is true.
            .when(self.active, |this| {
                this.child(
                    badge(false)
                        .debug_selector(|| "active-signal".into())
                        .px_3()
                        .py_1()
                        .text_sm()
                        .child("Signal visible"),
                )
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
    fn exercise_05(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, _| TogglePanel::new());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(cx.debug_bounds("active-signal").is_none());

        let button = cx
            .debug_bounds("toggle-button")
            .expect("button not rendered");
        cx.simulate_click(button.center(), Modifiers::default());
        cx.update(|window, cx| {
            assert!(panel.read(cx).active, "the click should update the entity");
            window.draw(cx).clear(cx);
        });
        assert!(
            cx.debug_bounds("active-signal").is_some(),
            "the active entity should render its signal"
        );

        cx.simulate_click(button.center(), Modifiers::default());
        cx.update(|window, cx| {
            assert!(!panel.read(cx).active, "the next click should toggle off");
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("active-signal").is_none());
    }
}
