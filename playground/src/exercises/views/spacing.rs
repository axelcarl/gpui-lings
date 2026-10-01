//! 06 — Give a layout room to breathe
//!
//! A parent's gap adds space between its children. Padding adds space inside
//! an element's edges. Keeping sibling spacing on the parent makes it
//! consistent as children are added or removed.
//!
//! Goal: add exactly 16 pixels between each pair of tiles, keeping all three
//! on one row. Set the gap with px(...); rem-based helpers depend on the
//! window's rem size. The check measures both gaps and the tiles' alignment.
//! Explore larger gaps after passing, then restore 16 pixels to continue.

use crate::theme::colors;
use gpui_kit::{IntoElement, div, prelude::*, px};

pub fn spaced_tiles() -> impl IntoElement {
    let c = colors();
    div()
        .flex()
        .flex_row()
        // TODO: Give both spaces in the row exactly 16 pixels.
        .gap(px(0.0))
        .children((0..3).map(|i| {
            div()
                .debug_selector(move || format!("spacing-tile-{i}"))
                .flex()
                .items_center()
                .justify_center()
                .size(px(64.0))
                .rounded_lg()
                .border_1()
                .border_color(c.border)
                .bg(c.muted)
                .child(format!("0{}", i + 1))
        }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use gpui_kit::{Context, Render, Window};

    struct SpacingHarness;
    impl Render for SpacingHarness {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            spaced_tiles()
        }
    }

    #[gpui::test]
    fn exercise_06(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| SpacingHarness);
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let tiles: Vec<_> = ["spacing-tile-0", "spacing-tile-1", "spacing-tile-2"]
            .into_iter()
            .map(|selector| cx.debug_bounds(selector).expect("tile not rendered"))
            .collect();
        for pair in tiles.windows(2) {
            assert_eq!(
                pair[0].origin.y, pair[1].origin.y,
                "keep the tiles in one row"
            );
            assert_eq!(
                pair[1].origin.x - pair[0].right(),
                px(16.0),
                "each pair of tiles should have a 16px gap"
            );
        }
    }
}
