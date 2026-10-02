//! 16 — Adapt a layout to window width
//!
//! A Render implementation receives the current Window. Its bounds can guide
//! which element tree to build each frame. This panel chooses a row for a wide
//! window and a column for a narrow one, while the parent still owns the layout.
//!
//! Goal: place the cards side by side in a wide window and stack them in a
//! narrow window. The wide layout already works. Fix the narrow branch below.
//! Resize the playground to see both arrangements. The check measures rendered
//! card bounds at both sizes and after resizing back.
//!
//! Example — Reading the available width:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! let width = window.bounds().size.width;
//! let label = if width < px(600.0) { "Compact" } else { "Wide" };
//! div().child(label)
//! ```

use crate::theme::colors;
use gpui_kit::{Context, IntoElement, Render, Window, div, prelude::*, px};

pub struct ResponsivePanel;

impl Render for ResponsivePanel {
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let narrow = window.bounds().size.width < px(760.0);
        responsive_cards(narrow)
    }
}

// TODO: Stack the cards when the window is narrow.
fn responsive_cards(narrow: bool) -> impl IntoElement {
    let c = colors();
    let cards = div().flex().gap(px(16.0)).children((0..2).map(|index| {
        div()
            .debug_selector(move || format!("responsive-card-{index}"))
            .w(px(144.0))
            .h(px(96.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_lg()
            .border_1()
            .border_color(c.border)
            .bg(c.muted)
            .child(format!("Panel {}", index + 1))
    }));

    if narrow {
        cards.gap(px(8.0)).flex_row()
    } else {
        cards.flex_row()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    fn exercise_16(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| ResponsivePanel);
        for (width, should_stack) in [(960.0, false), (640.0, true), (960.0, false)] {
            cx.simulate_resize(gpui::size(px(width), px(560.0)));
            cx.update(|window, cx| window.draw(cx).clear(cx));
            let first = cx
                .debug_bounds("responsive-card-0")
                .expect("first card not rendered");
            let second = cx
                .debug_bounds("responsive-card-1")
                .expect("second card not rendered");
            if should_stack {
                assert_eq!(second.origin.x, first.origin.x, "narrow cards should align");
                assert!(
                    second.origin.y > first.origin.y,
                    "narrow cards should stack vertically"
                );
            } else {
                assert_eq!(second.origin.y, first.origin.y, "wide cards should align");
                assert!(
                    second.origin.x > first.origin.x,
                    "wide cards should sit side by side"
                );
            }
        }
    }
}
