// This view stores only a count. Every time it renders, it works out its
// message from that count. Because the message is derived from the count on
// each render, the two can never disagree, and there's no second variable to
// keep in sync.
//
// The button already works, so you can click through 2, 3 and 4 to see where
// the message changes.

use crate::theme::{badge, button};
use gpui_kit::{Context, IntoElement, Render, Window, div, prelude::*, px};

#[derive(Default)]
pub struct Milestone {
    count: u32,
}

impl Render for Milestone {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Worked out fresh on every render, from the count alone.
        // TODO: This comparison is off by one. Celebrate as soon as the count
        // reaches three, not only after it passes three.
        let reached = self.count > 3;
        let message = if reached {
            "You reached three!"
        } else {
            "Keep clicking to reach three."
        };
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(div().text_size(px(48.0)).child(self.count.to_string()))
            .child(
                button("milestone-increment", "Increase count", true)
                    .debug_selector(|| "milestone-increment".into())
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.count += 1;
                        cx.notify();
                    })),
            )
            .child(
                badge(false)
                    // Named after the message, so the check can tell which
                    // one is on screen.
                    .debug_selector(move || {
                        if reached {
                            "milestone-reached".into()
                        } else {
                            "milestone-pending".into()
                        }
                    })
                    .px_3()
                    .py_1()
                    .text_sm()
                    .child(message),
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
    fn exercise_03(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| Milestone::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(cx.debug_bounds("milestone-pending").is_some());
        let button = cx.debug_bounds("milestone-increment").unwrap();
        // The count after each click, and whether it should celebrate.
        for (count, reached) in [(1, false), (2, false), (3, true), (4, true)] {
            cx.simulate_click(button.center(), Modifiers::default());
            cx.update(|window, cx| window.draw(cx).clear(cx));
            let selector = if reached {
                "milestone-reached"
            } else {
                "milestone-pending"
            };
            assert!(
                cx.debug_bounds(selector).is_some(),
                "at {count}, the message should be {}",
                if reached {
                    "You reached three!"
                } else {
                    "Keep clicking to reach three."
                }
            );
        }
    }
}
