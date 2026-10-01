//! 04 — Compose a row with flex
//!
//! progress_strip returns an element tree. The parent div owns the layout;
//! its two children are numbered squares. Flex is enabled, but its direction
//! currently stacks the children vertically.
//!
//! Goal: put tile 02 to the right of tile 01, at the same height.
//! Change the parent's flex direction. The check measures rendered bounds.
//! Explore a different gap afterward: direction and spacing are separate.

use crate::theme::colors;
use gpui_kit::{IntoElement, div, prelude::*, px};

pub fn progress_strip() -> impl IntoElement {
    let c = colors();
    div()
        .flex()
        // TODO: Choose the flex direction that puts the steps on one row.
        .flex_col()
        .gap_3()
        .child(
            div()
                .debug_selector(|| "step-one".into())
                .size(px(64.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded_lg()
                .bg(c.primary)
                .text_color(c.primary_foreground)
                .child("01"),
        )
        .child(
            div()
                .debug_selector(|| "step-two".into())
                .size(px(64.0))
                .flex()
                .items_center()
                .justify_center()
                .rounded_lg()
                .border_1()
                .border_color(c.border)
                .bg(c.muted)
                .child("02"),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use gpui_kit::{Context, Render, Window};

    struct ProgressHarness;

    impl Render for ProgressHarness {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            progress_strip()
        }
    }

    #[gpui::test]
    fn exercise_04(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| ProgressHarness);
        cx.update(|window, cx| window.draw(cx).clear(cx));

        let first = cx
            .debug_bounds("step-one")
            .expect("first step not rendered");
        let second = cx
            .debug_bounds("step-two")
            .expect("second step not rendered");
        assert!(
            second.origin.x > first.origin.x,
            "steps should flow to the right"
        );
        assert_eq!(second.origin.y, first.origin.y, "steps should share a row");
    }
}
