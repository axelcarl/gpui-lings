// A drag has three parts: the mouse button goes down, the pointer moves, and
// the button comes back up. Each part has its own handler, and each runs with
// the view, as `cx.listener` arranges. The release doesn't have to happen over
// the element where the drag began. If the user lets go somewhere else, the
// drag has to end anyway, or the control stays stuck in its dragging state.
//
// This time you write most of the gesture. Try it in the preview: drag along
// the track, let go over it, then drag again and let go outside it.

use crate::theme::colors;
use gpui_kit::{
    Context, IntoElement, MouseButton, MouseDownEvent, MouseMoveEvent, Pixels, Render, Window, div,
    prelude::*, px, relative,
};

pub struct DragPanel {
    value: u8,
    // While dragging: where the pointer went down, and the value at that time.
    // `None` means idle.
    drag_start: Option<(Pixels, u8)>,
}

impl Default for DragPanel {
    fn default() -> Self {
        Self {
            value: 25,
            drag_start: None,
        }
    }
}

impl DragPanel {
    // Moving changes the value only during a drag: two pixels per point.
    fn move_pointer(&mut self, x: Pixels, cx: &mut Context<Self>) {
        if let Some((start, initial)) = self.drag_start {
            let delta = (x - start).as_f32() / 2.0;
            self.value = (f32::from(initial) + delta).clamp(0.0, 100.0).round() as u8;
            cx.notify();
        }
    }
}

impl Render for DragPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = colors();
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                div()
                    .id("drag-track")
                    .debug_selector(|| "drag-track".into())
                    .w(px(240.0))
                    .h(px(44.0))
                    .rounded_full()
                    .bg(c.muted)
                    .border_1()
                    .border_color(c.border)
                    // The drag begins: remember where, and the value at the time.
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, event: &MouseDownEvent, _, cx| {
                            this.drag_start = Some((event.position.x, this.value));
                            cx.notify();
                        }),
                    )
                    // TODO: The track only notices the button going down, so the
                    // value never moves and the drag never ends. Finish the
                    // gesture with three more handlers, shaped like the one above:
                    // - `on_mouse_move`: pass the pointer's x position to
                    //   `move_pointer`. Its event is a `MouseMoveEvent`.
                    // - `on_mouse_up`: a release over the track ends the drag.
                    // - `on_mouse_up_out`: so does a release anywhere else.
                    // The filled part of the track shows the value.
                    .child(
                        div()
                            .h_full()
                            .w(relative(f32::from(self.value) / 100.0))
                            .rounded_full()
                            .bg(c.primary),
                    ),
            )
            .child(
                div()
                    .debug_selector(|| format!("drag-value-{}", self.value))
                    .child(format!("Value: {}", self.value)),
            )
            .child(
                div()
                    .debug_selector(|| {
                        if self.drag_start.is_some() {
                            "drag-active".into()
                        } else {
                            "drag-idle".into()
                        }
                    })
                    .text_color(c.muted_foreground)
                    .child(if self.drag_start.is_some() {
                        "Dragging"
                    } else {
                        "Idle"
                    }),
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
    fn exercise_24(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, _| DragPanel::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let track = cx.debug_bounds("drag-track").expect("track missing");
        let middle = track.center();
        let moved = gpui::point(middle.x + px(60.0), middle.y);
        cx.simulate_mouse_down(middle, MouseButton::Left, Modifiers::default());
        cx.simulate_mouse_move(moved, Some(MouseButton::Left), Modifiers::default());
        cx.update(|_, cx| {
            assert!(
                panel.read(cx).value > 25,
                "moving while the button is down should change the value"
            );
            assert!(panel.read(cx).drag_start.is_some(), "drag should be active");
        });
        cx.simulate_mouse_up(moved, MouseButton::Left, Modifiers::default());
        cx.update(|_, cx| {
            assert!(
                panel.read(cx).drag_start.is_none(),
                "releasing over the track must end the drag"
            )
        });

        cx.simulate_mouse_down(middle, MouseButton::Left, Modifiers::default());
        let outside = gpui::point(track.right() + px(30.0), middle.y);
        cx.simulate_mouse_up(outside, MouseButton::Left, Modifiers::default());
        cx.update(|_, cx| {
            assert!(
                panel.read(cx).drag_start.is_none(),
                "releasing outside the track must end the drag too"
            )
        });
        let released_value = cx.update(|_, cx| panel.read(cx).value);
        cx.simulate_mouse_move(moved, None, Modifiers::default());
        cx.update(|window, cx| {
            assert_eq!(
                panel.read(cx).value,
                released_value,
                "moving after the release must not change the value"
            );
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("drag-idle").is_some());
    }
}
