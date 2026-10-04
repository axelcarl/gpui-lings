// `render` gets the `Window` as well, so a view can look at the window's size
// and build a different element tree for it. GPUI renders again after every
// resize, so the choice is made fresh each time.
//
// This panel puts its two cards side by side in a wide window, and should
// stack them in a narrow one. The wide layout already works. Resize the
// preview window across the breakpoint to see both.

use crate::theme::colors;
use gpui_kit::{Context, IntoElement, Render, Window, div, prelude::*, px};

pub struct ResponsivePanel;

impl Render for ResponsivePanel {
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        // The window's current size. GPUI renders again after every resize, so
        // this choice is made fresh each frame.
        let narrow = window.bounds().size.width < px(760.0);
        responsive_cards(narrow)
    }
}

// Builds the same two cards either way; only the parent's direction differs.
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

    // TODO: The narrow branch uses `flex_row()` too, so the cards never stack.
    // Give it the flex direction that places them one above the other.
    if narrow {
        cards.gap(px(8.0)).flex_row()
    } else {
        cards.flex_row()
    }
}

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test]
    fn exercise_21(cx: &mut TestAppContext) {
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
