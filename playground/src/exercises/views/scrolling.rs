//! 17 — Keep content usable in a small window
//!
//! A fixed-height panel needs its own scrollable area when its content grows.
//! Keep the heading outside that area so it remains visible while the rows move.
//! A ScrollHandle retains scroll position across redraws and lets a check
//! inspect the list's scroll position.
//!
//! Goal: make the list scroll vertically so its last row can be reached. The
//! heading should stay still. Change only the overflow behavior of the list.
//! The check sends a scroll gesture and measures the viewport and scroll position.

use crate::theme::colors;
use gpui_kit::{Context, IntoElement, Render, ScrollHandle, Window, div, prelude::*, px};

#[derive(Default)]
pub struct ScrollingPanel {
    scroll: ScrollHandle,
}

impl Render for ScrollingPanel {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let c = colors();
        div()
            .w(px(240.0))
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .debug_selector(|| "scroll-heading".into())
                    .text_color(c.foreground)
                    .child("Recent items"),
            )
            .child(
                // TODO: Let this list scroll vertically inside its fixed height.
                div()
                    .id("recent-items")
                    .debug_selector(|| "scroll-viewport".into())
                    .h(px(160.0))
                    .w_full()
                    .border_1()
                    .border_color(c.border)
                    .rounded_lg()
                    .overflow_hidden()
                    .track_scroll(&self.scroll)
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{ScrollDelta, ScrollWheelEvent, TestAppContext};

    #[gpui::test]
    fn exercise_17(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, _| ScrollingPanel::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let heading = cx.debug_bounds("scroll-heading").expect("heading missing");
        let viewport = cx.debug_bounds("scroll-viewport").expect("list missing");
        let overflow = cx.update(|_, cx| panel.read(cx).scroll.max_offset().y);
        assert!(overflow > px(0.0), "the list must be vertically scrollable");

        cx.simulate_event(ScrollWheelEvent {
            position: viewport.center(),
            delta: ScrollDelta::Pixels(gpui::point(px(0.0), px(-500.0))),
            ..Default::default()
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let offset = cx.update(|_, cx| panel.read(cx).scroll.offset().y);
        assert!(offset < px(0.0), "a scroll gesture should move the list");
        let last = cx.debug_bounds("scroll-row-11").expect("last row missing");
        let heading_after = cx.debug_bounds("scroll-heading").expect("heading missing");
        assert_eq!(
            heading.origin, heading_after.origin,
            "heading must stay in place"
        );
        // The viewport's one-pixel border can overlap the row's final pixel.
        assert!(
            last.top() >= viewport.top() && last.bottom() <= viewport.bottom() + px(1.0),
            "the last row should fit inside the list after scrolling: last={last:?}, viewport={viewport:?}"
        );
    }
}
