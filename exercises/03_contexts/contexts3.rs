// An `Entity<T>` is a handle to state that GPUI owns. Clone the handle and you
// get a second handle to the *same* state. But `entity.read(cx)` gives you a
// reference to the value itself, and cloning *that* makes a separate copy that
// GPUI knows nothing about.
//
// To change the real thing, call `entity.update(cx, |value, cx| ...)`. GPUI
// hands your closure a mutable reference to the value, along with that
// entity's own context. That's the context to notify.

use crate::theme::button;
use gpui_kit::{Context, Entity, IntoElement, Render, Window, div, prelude::*};

// A child view. Deriving Clone lets you copy its *value*, separate from the
// entity GPUI owns: that copy is the trap in this exercise.
#[derive(Clone, Default)]
pub struct Score {
    value: usize,
}
impl Render for Score {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .debug_selector(|| format!("child-score-{}", self.value))
            .child(format!("Child score: {}", self.value))
    }
}

pub struct UpdatePanel {
    // A handle to the Score entity. Cloning the handle, as render does below,
    // still refers to the same score.
    score: Entity<Score>,
}
impl UpdatePanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            // Created once, here. Creating it in `render` would start a
            // fresh score on every frame.
            score: cx.new(|_| Score::default()),
        }
    }
}
impl Render for UpdatePanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(self.score.clone())
            .child(
                button("update-child", "Increment child", true)
                    .debug_selector(|| "update-child".into())
                    .on_click(cx.listener(|this, _, _, cx| {
                        // `read` borrows the score; `clone` then copies its value.
                        // TODO: This increments the copy and throws it away, so the
                        // score on screen never changes. Update the entity itself,
                        // and notify the score's context afterwards:
                        //     this.score.update(cx, |score, cx| ???);
                        let mut detached = this.score.read(cx).clone();
                        detached.value += 1;
                    })),
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
    fn exercise_09(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, cx| UpdatePanel::new(cx));
        let original = cx.update(|_, cx| panel.read(cx).score.entity_id());
        for expected in 1..=2 {
            cx.update(|window, cx| window.draw(cx).clear(cx));
            let button = cx.debug_bounds("update-child").unwrap();
            cx.simulate_click(button.center(), Modifiers::default());
            cx.update(|window, cx| {
                let score = &panel.read(cx).score;
                assert_eq!(score.entity_id(), original, "update the existing entity");
                assert_eq!(
                    score.read(cx).value,
                    expected,
                    "mutating a clone does not change the child"
                );
                window.draw(cx).clear(cx);
            });
        }
        assert!(cx.debug_bounds("child-score-2").is_some());
    }
}
