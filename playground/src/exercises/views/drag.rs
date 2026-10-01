//! 19 — Route a pointer gesture
//!
//! A drag has a beginning, movement, and an end. The mouse may be released
//! outside the element where the drag began, so the gesture must clean up on
//! both the inside and outside release paths.
//!
//! Goal: drag the value track left or right, then release outside it. The value
//! should update only while the button is held, and the track should return to
//! its idle state after release. Add the missing outside-release cleanup.

use crate::theme::colors;
use gpui_kit::{
    Context, IntoElement, MouseButton, MouseDownEvent, MouseMoveEvent, Pixels, Render, Window, div,
    prelude::*, px, relative,
};

pub struct DragPanel {
    value: u8,
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
                    .child(
                        div()
                            .h_full()
                            .w(relative(f32::from(self.value) / 100.0))
                            .rounded_full()
                            .bg(c.primary),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, event: &MouseDownEvent, _, cx| {
                            this.drag_start = Some((event.position.x, this.value));
                            cx.notify();
                        }),
                    )
                    .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                        this.move_pointer(event.position.x, cx);
                    }))
                    // TODO: Releasing the button anywhere, inside or outside, must end the drag.
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            this.drag_start = None;
                            cx.notify();
                        }),
                    )
                    .on_mouse_up_out(MouseButton::Left, cx.listener(|_this, _, _, _cx| {})),
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_19(cx: &mut TestAppContext) {
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
                "movement should change the value"
            );
            assert!(panel.read(cx).drag_start.is_some(), "drag should be active");
        });

        let outside = gpui::point(track.right() + px(30.0), middle.y);
        cx.simulate_mouse_up(outside, MouseButton::Left, Modifiers::default());
        cx.update(|_, cx| {
            assert!(
                panel.read(cx).drag_start.is_none(),
                "outside release must end the drag"
            )
        });
        let released_value = cx.update(|_, cx| panel.read(cx).value);
        cx.simulate_mouse_move(middle, None, Modifiers::default());
        cx.update(|_, cx| {
            assert_eq!(
                panel.read(cx).value,
                released_value,
                "idle movement must not change value"
            )
        });
    }
}
