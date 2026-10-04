// When there's more content than room, an element's overflow mode decides what
// happens to the rest. `overflow_hidden()` clips it out of sight.
// `overflow_y_scroll()`, `overflow_x_scroll()` and `overflow_scroll()` clip it
// too, but let the user scroll to it. A scrolling element needs an `id`, so
// GPUI can remember how far it has scrolled from one frame to the next.
//
// To scroll from code, keep a `ScrollHandle` in your view and attach it to the
// element with `track_scroll`. Then a button can move the list, as Jump to last
// does with `scroll_to_item`. The heading sits outside the list, so it should
// stay put while the rows move.

use crate::theme::{button, colors};
use gpui_kit::{Context, IntoElement, Render, ScrollHandle, Window, div, prelude::*, px};

#[derive(Default)]
pub struct ScrollingPanel {
    // The list's scroll position, readable and changeable from this view.
    scroll: ScrollHandle,
}

impl Render for ScrollingPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = colors();
        div()
            .w(px(240.0))
            .flex()
            .flex_col()
            .gap_2()
            .child(
                // The heading sits outside the list, so it never scrolls away.
                div()
                    .debug_selector(|| "scroll-heading".into())
                    .text_color(c.foreground)
                    .child("Recent items"),
            )
            .child(
                div()
                    .id("recent-items")
                    .debug_selector(|| "scroll-viewport".into())
                    // Twelve 36px rows don't fit in 160px.
                    .h(px(160.0))
                    .w_full()
                    .border_1()
                    .border_color(c.border)
                    .rounded_lg()
                    // TODO: Rows below the bottom edge are clipped and can't be
                    // reached. Pick the overflow mode that scrolls vertically.
                    .overflow_hidden()
                    // TODO: Jump to last moves `self.scroll`, but nothing is
                    // attached to it yet. Make this list track the handle.
                    .children((0..12).map(|index| {
                        div()
                            .debug_selector(move || format!("scroll-row-{index}"))
                            .h(px(36.0))
                            .px_3()
                            .flex()
                            .items_center()
                            .child(format!("Item {}", index + 1))
                    })),
            )
            .child(
                button("scroll-last", "Jump to last", false)
                    .debug_selector(|| "scroll-last".into())
                    .on_click(cx.listener(|this, _, _, cx| {
                        // Ask whichever list tracks the handle to reveal row 12.
                        this.scroll.scroll_to_item(11);
                        cx.notify();
                    })),
            )
    }
}

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Bounds, Modifiers, Pixels, ScrollDelta, ScrollWheelEvent, TestAppContext};

    fn inside(row: Bounds<Pixels>, viewport: Bounds<Pixels>) -> bool {
        // The viewport's one-pixel border can overlap the row's final pixel.
        row.top() >= viewport.top() && row.bottom() <= viewport.bottom() + px(1.0)
    }

    #[gpui::test]
    fn exercise_22(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, _| ScrollingPanel::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let heading = cx.debug_bounds("scroll-heading").expect("heading missing");
        let viewport = cx.debug_bounds("scroll-viewport").expect("list missing");
        let first = cx.debug_bounds("scroll-row-0").expect("first row missing");

        let wheel = |cx: &mut gpui::VisualTestContext, y: f32| {
            cx.simulate_event(ScrollWheelEvent {
                position: viewport.center(),
                delta: ScrollDelta::Pixels(gpui::point(px(0.0), px(y))),
                ..Default::default()
            });
            cx.update(|window, cx| window.draw(cx).clear(cx));
        };
        wheel(cx, -500.0);
        let moved = cx.debug_bounds("scroll-row-0").expect("first row missing");
        assert!(
            moved.top() < first.top(),
            "a scroll gesture should move the rows: use an overflow mode that scrolls vertically"
        );
        let last = cx.debug_bounds("scroll-row-11").expect("last row missing");
        assert!(
            inside(last, viewport),
            "scrolling should reach the last row: last={last:?}, viewport={viewport:?}"
        );
        let heading_after = cx.debug_bounds("scroll-heading").expect("heading missing");
        assert_eq!(
            heading.origin, heading_after.origin,
            "heading must stay in place"
        );

        // Back to the top, then let the button scroll through the handle.
        wheel(cx, 500.0);
        let jump = cx
            .debug_bounds("scroll-last")
            .expect("Jump to last missing");
        cx.simulate_click(jump.center(), Modifiers::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let last = cx.debug_bounds("scroll-row-11").expect("last row missing");
        assert!(
            inside(last, viewport),
            "Jump to last scrolls self.scroll: attach the handle to the list with track_scroll"
        );
        let offset = cx.update(|_, cx| panel.read(cx).scroll.offset().y);
        assert!(
            offset < px(0.0),
            "the handle should report the new position"
        );
    }
}
