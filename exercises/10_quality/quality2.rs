// This view already works. This time the exercise is its test, at the bottom
// of the file. The test drives the view the way a user would: it draws a
// frame, clicks Load, and checks what's on screen. It never calls the view's
// methods directly.
//
// A `#[gpui::test]` runs on a simulated clock, and time stands still until the
// test moves it, so a one-second timer never fires on its own. `advance_clock`
// moves the clock forward and fires the timers that come due, and then
// `run_until_parked` runs every task that can make progress. No real waiting
// needed. The checks in earlier lessons, such as lesson 19's, work this way.

use crate::theme::button;
use gpui_kit::{Context, IntoElement, Render, Task, Window, div, prelude::*};
use std::time::Duration;

#[derive(Default)]
pub struct BehaviorTestPanel {
    loading: bool,
    loaded: bool,
    // The running load. Dropping it would cancel the work (lesson 19).
    task: Option<Task<()>>,
}

impl BehaviorTestPanel {
    // The test can't call this method: it is private to the view. It runs only
    // when Load is clicked.
    fn load(&mut self, cx: &mut Context<Self>) {
        self.loading = true;
        self.loaded = false;
        cx.notify();
        self.task = Some(cx.spawn(async move |this, cx| {
            // A real second in the app. In a test, it waits for the simulated
            // clock to pass one second.
            cx.background_executor().timer(Duration::from_secs(1)).await;
            let _ = this.update(cx, |this, cx| {
                this.loading = false;
                this.loaded = true;
                cx.notify();
            });
        }));
    }
}

impl Render for BehaviorTestPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let status = if self.loading {
            "Loading"
        } else if self.loaded {
            "Ready"
        } else {
            "Idle"
        };
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                button("behavior-load", "Load", true)
                    .debug_selector(|| "behavior-load".into())
                    .on_click(cx.listener(|this, _, _, cx| this.load(cx))),
            )
            .child(
                // Named "behavior-Idle", "behavior-Loading" or "behavior-Ready",
                // so the test can check what is on screen.
                div()
                    .debug_selector(|| format!("behavior-{status}"))
                    .child(status),
            )
    }
}

// This test is the exercise: ./gpui-lings runs it, and you complete it. The
// window setup and every assertion are given; don't change them. Add the two
// missing steps marked TODO, so that each assertion sees the state it expects.
#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_39(cx: &mut TestAppContext) {
        // Opens a headless window showing the panel. `panel` is the view's
        // entity; `window` is a VisualTestContext, which can draw the window
        // and send it input.
        let (panel, window) = cx.add_window_view(|_, _| BehaviorTestPanel::default());
        // Draw one frame, so elements have bounds to look up.
        window.update(|window, cx| window.draw(cx).clear(cx));
        // Where the Load button was drawn, found through its debug_selector.
        let load = window.debug_bounds("behavior-load").unwrap();

        // TODO: Click Load with `window.simulate_click`. It takes a point,
        // such as the center of `load`, and the modifier keys held down.

        // Right after the click, the view is loading.
        window.update(|window, cx| {
            assert!(panel.read(cx).loading, "Load should show Loading first");
            window.draw(cx).clear(cx);
        });
        assert!(window.debug_bounds("behavior-Loading").is_some());

        // TODO: The load waits on a one-second timer, and simulated time stands
        // still. Move the clock forward by one second with `advance_clock` on
        // the window's executor, then let the woken task finish with
        // `run_until_parked`.

        // After one simulated second, the result has arrived.
        window.update(|window, cx| {
            assert!(panel.read(cx).loaded, "the simulated result should arrive");
            window.draw(cx).clear(cx);
        });
        assert!(window.debug_bounds("behavior-Ready").is_some());
    }
}
