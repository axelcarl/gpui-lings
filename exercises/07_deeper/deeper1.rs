//! 21 — Move between focus regions
//!
//! Focus belongs to the window. An overlay can have several focusable regions,
//! but opening and closing it should leave the keyboard in a predictable place.
//! Store handles for the caller and each region, then use Window to move focus.
//!
//! Goal: Open moves focus to the first pad, Move focus moves it to the second,
//! and Close returns it to Open. The first two moves already work. Add the
//! missing focus restoration when the overlay closes. The check also sends X
//! to each pad and makes sure closed pads stop receiving keys.
//!
//! Example — Returning focus after dismissing an overlay:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! fn dismiss(&mut self, window: &mut Window, cx: &mut Context<Self>) {
//!     self.visible = false;
//!     window.focus(&self.launcher, cx);
//!     cx.notify();
//! }
//! ```

use crate::theme::{button, colors, focus_ring};
use gpui_kit::{Context, FocusHandle, IntoElement, Render, Window, div, prelude::*, px};

pub struct RegionsPanel {
    // One handle per place the keyboard can be: the Open button that opens the
    // overlay (the caller) and the overlay's two pads.
    trigger: FocusHandle,
    first: FocusHandle,
    second: FocusHandle,
    // While true, the overlay and its pads are part of the element tree.
    open: bool,
    // How many times each pad has received X.
    first_presses: usize,
    second_presses: usize,
}

impl RegionsPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            trigger: cx.focus_handle(),
            first: cx.focus_handle(),
            second: cx.focus_handle(),
            open: false,
            first_presses: 0,
            second_presses: 0,
        }
    }
}

impl Render for RegionsPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = colors();
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                button("regions-open", "Open", true)
                    // The Open button is where focus should return on Close.
                    .track_focus(&self.trigger)
                    .debug_selector(|| "regions-open".into())
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.open = true;
                        // Focus belongs to the window: `window.focus` moves it to
                        // the element that tracks the handle you pass.
                        window.focus(&this.first, cx);
                        cx.notify();
                    })),
            )
            .when(self.open, |panel| {
                panel.child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap_3()
                        .p_4()
                        .rounded_lg()
                        .border_1()
                        .border_color(c.border)
                        .child(
                            div()
                                .id("region-first")
                                .track_focus(&self.first)
                                .debug_selector(|| "region-first".into())
                                .w(px(180.0))
                                .p_3()
                                .rounded_md()
                                .bg(c.muted)
                                .focus(focus_ring)
                                // Keys go to the focused pad. `stop_propagation`
                                // keeps X from reaching the pad's ancestors too.
                                // The second pad works the same way.
                                .on_key_down(cx.listener(
                                    |this, event: &gpui_kit::KeyDownEvent, _, cx| {
                                        if event.keystroke.key == "x" {
                                            this.first_presses += 1;
                                            cx.notify();
                                            cx.stop_propagation();
                                        }
                                    },
                                ))
                                .child(format!("First · X: {}", self.first_presses)),
                        )
                        .child(
                            div()
                                .id("region-second")
                                .track_focus(&self.second)
                                .debug_selector(|| "region-second".into())
                                .w(px(180.0))
                                .p_3()
                                .rounded_md()
                                .bg(c.muted)
                                .focus(focus_ring)
                                .on_key_down(cx.listener(
                                    |this, event: &gpui_kit::KeyDownEvent, _, cx| {
                                        if event.keystroke.key == "x" {
                                            this.second_presses += 1;
                                            cx.notify();
                                            cx.stop_propagation();
                                        }
                                    },
                                ))
                                .child(format!("Second · X: {}", self.second_presses)),
                        )
                        .child(
                            button("regions-next", "Move focus", false)
                                .debug_selector(|| "regions-next".into())
                                .on_click(cx.listener(|this, _, window, cx| {
                                    // A focus change redraws the window by itself.
                                    // `cx.notify()` is for changes to the view's fields.
                                    window.focus(&this.second, cx);
                                })),
                        )
                        .child(
                            button("regions-close", "Close", false)
                                .debug_selector(|| "regions-close".into())
                                .on_click(cx.listener(|this, _, window, cx| {
                                    // Hiding the overlay removes both pads from the tree.
                                    this.open = false;
                                    // TODO: Focus is still on a pad that is no longer
                                    // drawn. Return it to the Open button: call
                                    // `window.focus(...)` with `this.trigger` and `cx`.
                                    cx.notify();
                                })),
                        ),
                )
            })
    }
}

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_21(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, cx| RegionsPanel::new(cx));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let open = cx.debug_bounds("regions-open").expect("Open missing");
        cx.simulate_click(open.center(), Modifiers::default());
        cx.update(|window, cx| assert!(panel.read(cx).first.is_focused(window)));
        cx.simulate_keystrokes("x");
        cx.update(|_, cx| assert_eq!(panel.read(cx).first_presses, 1));

        cx.update(|window, cx| window.draw(cx).clear(cx));
        let next = cx.debug_bounds("regions-next").expect("Move focus missing");
        cx.simulate_click(next.center(), Modifiers::default());
        cx.update(|window, cx| assert!(panel.read(cx).second.is_focused(window)));
        cx.simulate_keystrokes("x");
        cx.update(|_, cx| assert_eq!(panel.read(cx).second_presses, 1));

        let close = cx.debug_bounds("regions-close").expect("Close missing");
        cx.simulate_click(close.center(), Modifiers::default());
        cx.update(|window, cx| {
            assert!(!panel.read(cx).open);
            assert!(
                panel.read(cx).trigger.is_focused(window),
                "restore caller focus"
            );
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("region-first").is_none());
        cx.simulate_keystrokes("x");
        cx.update(|_, cx| {
            assert_eq!(panel.read(cx).first_presses, 1);
            assert_eq!(panel.read(cx).second_presses, 1);
        });
    }
}
