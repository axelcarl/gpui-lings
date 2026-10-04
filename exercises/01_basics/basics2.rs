// In GPUI, a view is a struct that owns some state and knows how to render it.
// `Counter` below owns a `count`, and its `render` method builds the elements
// that show it: a big number and a button. Every click runs the closure given
// to the button's `on_click`, which changes `count` and asks GPUI to render the
// view again. Chapter 03 looks closely at `cx.listener` and `cx.notify()`; for
// now, only what the closure does to `count` is wrong.
//
// Click the button in the preview to see what happens now. Reset preview sets
// the count back to zero without touching your code.

use crate::theme::button;
use gpui_kit::{Context, IntoElement, Render, Window, div, prelude::*, px};

// The view's state. `Default` starts the count at zero.
#[derive(Default)]
pub struct Counter {
    count: u32,
}

impl Render for Counter {
    // GPUI calls this to draw the view, and again whenever it's told the view
    // changed. `cx` is the view's context; the click handler below uses it.
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                div()
                    // `debug_selector` names an element so the check can find
                    // it. This name includes the count. It changes nothing on
                    // screen; leave it in place.
                    .debug_selector(|| format!("count-{}", self.count))
                    .text_size(px(48.0))
                    .child(self.count.to_string()),
            )
            .child(
                button("increment-button", "Increase count", true)
                    .debug_selector(|| "increment-button".into())
                    // Runs on every click, with this view (`this`) and its
                    // context (`cx`).
                    .on_click(cx.listener(|this, _, _, cx| {
                        // TODO: `saturating_sub(1)` subtracts one, stopping at
                        // zero. Make every click add one instead.
                        this.count = this.count.saturating_sub(1);
                        cx.notify(); // Ask GPUI to render the view again.
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
    fn exercise_02(cx: &mut TestAppContext) {
        let (counter, cx) = cx.add_window_view(|_, _| Counter::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let button = cx
            .debug_bounds("increment-button")
            .expect("button not rendered");
        cx.simulate_click(button.center(), Modifiers::default());
        cx.update(|_, cx| {
            assert_eq!(
                counter.read(cx).count,
                1,
                "one click should increase the count"
            )
        });
        cx.simulate_click(button.center(), Modifiers::default());
        cx.update(|window, cx| {
            assert_eq!(
                counter.read(cx).count,
                2,
                "the next click should increase it again"
            );
            window.draw(cx).clear(cx);
        });
        assert!(
            cx.debug_bounds("count-2").is_some(),
            "the view should show the new count"
        );
    }
}
