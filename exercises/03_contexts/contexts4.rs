// An entity doesn't have to render anything. `Reading` is just data, a *model*,
// and the panel shows a copy of its value. To keep that copy up to date, the
// panel observes the model: `cx.observe(&model, callback)` runs the callback
// every time the model calls `notify`.
//
// The callback is only told *that* the model changed, not what changed. It
// does get the observed entity, though, so read the current value from there.
// Observing returns a `Subscription`, and the observer only runs while that
// value is kept, which is why the panel stores it in a field.

use crate::theme::button;
use gpui_kit::{Context, Entity, IntoElement, Render, Subscription, Window, div, prelude::*};

#[derive(Default)]
pub struct Reading {
    value: usize,
}
pub struct ObservePanel {
    reading: Entity<Reading>,
    mirrored: usize,
    _observation: Subscription,
}
impl ObservePanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let reading = cx.new(|_| Reading::default());
        // `observe` runs this closure each time `reading` notifies. It receives
        // this panel (`this`), the observed entity (`reading`) and the context.
        let observation = cx.observe(&reading, |this, reading, cx| {
            // TODO: `mirrored` is always reset to 0. Copy the model's current
            // value into it instead.
            this.mirrored = 0;
            cx.notify();
        });
        Self {
            reading,
            mirrored: 0,
            // Dropping the Subscription would stop the observer, so keep it.
            _observation: observation,
        }
    }
}
impl Render for ObservePanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                div()
                    .debug_selector(|| format!("mirrored-{}", self.mirrored))
                    .child(format!("Mirrored reading: {}", self.mirrored)),
            )
            .child(
                button("observe-increment", "Update model", true)
                    .debug_selector(|| "observe-increment".into())
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.reading.update(cx, |reading, cx| {
                            reading.value += 1;
                            cx.notify();
                        });
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
    fn exercise_10(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, cx| ObservePanel::new(cx));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let button = cx.debug_bounds("observe-increment").unwrap();
        cx.simulate_click(button.center(), Modifiers::default());
        cx.update(|_, cx| {
            assert_eq!(
                panel.read(cx).mirrored,
                1,
                "observe must read the model's value"
            )
        });
        let reading = cx.update(|_, cx| panel.read(cx).reading.clone());
        reading.update(cx, |reading, cx| {
            reading.value = 7;
            cx.notify();
        });
        cx.update(|window, cx| {
            assert_eq!(
                panel.read(cx).mirrored,
                7,
                "updates can originate outside the button"
            );
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("mirrored-7").is_some());
    }
}
