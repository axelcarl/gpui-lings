//! 09 — Update the entity, not a copy
//!
//! Entity<T> is a handle to state owned by GPUI. Cloning the handle refers to
//! the same entity; cloning the value returned by read creates separate data.
//! Use entity.update(cx, |value, cx| ...) to mutate the original. The inner
//! context belongs to that entity, so notify it after changing its state.
//!
//! Goal: make the parent button increment the child's score on every click.
//! Replace the detached-copy block with an update of this.score. Keep score
//! creation in new: creating it during render would reset it on each frame.

use crate::theme::button;
use gpui_kit::{Context, Entity, IntoElement, Render, Window, div, prelude::*};

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
    score: Entity<Score>,
}
impl UpdatePanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
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
                        // TODO: Update the stored entity and notify its context.
                        let mut detached = this.score.read(cx).clone();
                        detached.value += 1;
                        let _ = detached.value;
                    })),
            )
    }
}

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
