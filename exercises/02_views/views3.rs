// Spacing can live in two places. A parent's `gap` puts space *between* its
// children. Padding puts space *inside* an element, between its edges and its
// content. Keeping the space between siblings on the parent keeps it even when
// you add or remove a tile.
//
// Helpers like `gap_4()` scale with the window's font size, while `gap(px(..))`
// takes an exact number of pixels. This exercise wants an exact size. Once it
// passes, try bigger gaps in the preview, then set it back to continue.

use crate::theme::colors;
use gpui_kit::{IntoElement, div, prelude::*, px};

pub fn spaced_tiles() -> impl IntoElement {
    let c = colors();
    div()
        .flex()
        .flex_row()
        // Space *between* siblings is the parent's `gap`. Padding, such as
        // `.p(px(8.0))`, would add space *inside* an element's edges instead.
        // TODO: Make both spaces exactly 16 pixels. `px(...)` takes a pixel count.
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

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
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
