// An element's `on_click` callback gets three arguments: the click event, the
// window and the app. Your view isn't one of them, so a plain closure can't
// change the view's fields. `cx.listener(...)` fixes that. It wraps a closure,
// or a method, that takes the view (`&mut Self`) and its `Context<Self>`, and
// turns it into the callback `on_click` expects. When the click comes, GPUI
// looks up the view and hands it to your code.
//
// You've seen `cx.listener` wrap closures in lessons 05 and 07. It accepts a
// method with the same four arguments too, like `record_click` below.

use crate::theme::button;
use gpui_kit::{ClickEvent, Context, IntoElement, Render, Window, div, prelude::*};

#[derive(Default)]
pub struct ListenerPanel {
    clicks: usize,
}

impl ListenerPanel {
    // A method with the listener signature: this view, the event, the window
    // and this view's context. It already updates the count and notifies.
    pub fn record_click(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.clicks += 1;
        cx.notify();
    }
}

impl Render for ListenerPanel {
    // Inside `render`, `cx` is this view's `Context<ListenerPanel>`.
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(format!("Handler calls: {}", self.clicks))
            .child(
                button("listener-record", "Call the handler", true)
                    .debug_selector(|| "listener-record".into())
                    // An element callback only receives `|event, window, app|`,
                    // so it can't reach this view on its own.
                    // TODO: Clicking does nothing yet. Make the click call
                    // this view's `record_click` method instead.
                    .on_click(|_, _, _| {}),
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
    fn exercise_08(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, _| ListenerPanel::default());
        for expected in 1..=2 {
            cx.update(|window, cx| window.draw(cx).clear(cx));
            let button = cx.debug_bounds("listener-record").unwrap();
            cx.simulate_click(button.center(), Modifiers::default());
            cx.update(|_, cx| {
                assert_eq!(
                    panel.read(cx).clicks,
                    expected,
                    "the handler should run once per click"
                )
            });
        }
    }
}
