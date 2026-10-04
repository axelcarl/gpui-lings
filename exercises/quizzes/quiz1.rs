// This is a quiz for the following lessons:
// - 01–03 Foundations
// - 04–06 Views & layout
// - 07–09 Contexts & handlers
//
// This time there's nothing to fix: you write the view yourself. `Tally`
// counts taps, like a clicker at a door. Its `render` should build:
//
// - a label that shows the count, such as "3 taps",
// - below it, a row of two buttons side by side: Tap, which adds one, and
//   Clear, which sets the count back to zero.
//
// The check finds elements by their debug selectors, so name the label
// `tally-` followed by the count (`tally-3`), and the buttons `tally-tap` and
// `tally-clear`. The course theme's `button(id, label, primary)` makes a
// button; give it a `debug_selector` as well, the way earlier lessons do.

use crate::theme::button;
use gpui_kit::{Context, IntoElement, Render, Window, div, prelude::*};

#[derive(Default)]
pub struct Tally {
    count: u32,
}

impl Render for Tally {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // TODO: Build the view described at the top of this file. The text
        // below only marks the spot in the preview; replace it.
        div().child("Your tally view goes here")
    }
}

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_12(cx: &mut TestAppContext) {
        let (tally, cx) = cx.add_window_view(|_, _| Tally::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let label = cx
            .debug_bounds("tally-0")
            .expect("show the count in a label named tally-0");
        let tap = cx
            .debug_bounds("tally-tap")
            .expect("add a Tap button named tally-tap");
        let clear = cx
            .debug_bounds("tally-clear")
            .expect("add a Clear button named tally-clear");
        assert_eq!(
            tap.origin.y, clear.origin.y,
            "Tap and Clear should sit side by side"
        );
        assert!(
            tap.origin.x < clear.origin.x,
            "Tap should come before Clear"
        );
        assert!(
            label.bottom() <= tap.top(),
            "the label should sit above the buttons"
        );

        for expected in 1..=3 {
            let tap = cx.debug_bounds("tally-tap").unwrap();
            cx.simulate_click(tap.center(), Modifiers::default());
            cx.update(|window, cx| {
                assert_eq!(tally.read(cx).count, expected, "Tap should add one");
                window.draw(cx).clear(cx);
            });
        }
        assert!(
            cx.debug_bounds("tally-3").is_some(),
            "the label should show the new count"
        );

        let clear = cx.debug_bounds("tally-clear").unwrap();
        cx.simulate_click(clear.center(), Modifiers::default());
        cx.update(|window, cx| {
            assert_eq!(
                tally.read(cx).count,
                0,
                "Clear should set the count back to zero"
            );
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("tally-0").is_some());
    }
}
