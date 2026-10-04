// This is a quiz for the following lessons:
// - 13–16 Keyboard input
// - 21–24 Layout & control states
//
// This inspector puts recent lessons together in a new view: a layout that
// changes with the window's width, a list that scrolls inside a fixed frame, a
// selected row, and keyboard focus. Click a row or Tab into the list, then
// press J to select the next row. Disable freezes the selection, for the mouse
// and the keyboard alike.
//
// This time, nothing marks the broken lines. Testers reported:
//
// - Narrowing the window never stacks the panels, but making it shorter does.
// - J does nothing, even after clicking a row.
// - While disabled, clicks are ignored but J still moves the selection.
// - The list grows past its frame instead of scrolling.
//
// Find and fix all four. The check stops at the first symptom it finds, so
// reproduce that one in the preview (change the width and the height
// separately), trace it back to the code, and fix it before moving on.

use crate::theme::{button, colors, focus_ring};
use gpui_kit::{
    Context, FocusHandle, IntoElement, Render, ScrollHandle, Window, div, prelude::*, px,
};

pub struct InspectorPanel {
    selected: usize,
    disabled: bool,
    focus: FocusHandle,
    scroll: ScrollHandle,
}

impl InspectorPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            selected: 0,
            disabled: false,
            focus: cx.focus_handle(),
            scroll: ScrollHandle::new(),
        }
    }

    fn select(&mut self, index: usize, cx: &mut Context<Self>) {
        if !self.disabled {
            self.selected = index;
            cx.notify();
        }
    }

    fn next(&mut self, cx: &mut Context<Self>) {
        self.selected = (self.selected + 1).min(11);
        cx.notify();
    }
}

impl Render for InspectorPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = colors();
        let narrow = window.bounds().size.height < px(760.0);
        let list = div()
            .id("inspector-list")
            .debug_selector(|| "inspector-list".into())
            .track_focus(&self.focus)
            .tab_index(0)
            .bg(colors().card)
            .focus_visible(focus_ring)
            .w(px(220.0))
            .overflow_y_scroll()
            .track_scroll(&self.scroll)
            .border_1()
            .border_color(c.border)
            .rounded_lg()
            .children((0..12).map(|index| {
                div()
                    .id(("inspector-row", index))
                    .debug_selector(move || format!("inspector-row-{index}"))
                    .h(px(36.0))
                    .px_3()
                    .flex()
                    .items_center()
                    .when(self.selected == index, |el| {
                        el.debug_selector(move || format!("inspector-selected-{index}"))
                            .bg(c.primary)
                            .text_color(c.primary_foreground)
                    })
                    .when(self.disabled, |el| el.opacity(0.45))
                    .when(!self.disabled && self.selected != index, |el| {
                        el.hover(|style| style.bg(c.accent))
                    })
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.select(index, cx);
                        window.focus(&this.focus, cx);
                    }))
                    .child(format!("Item {}", index + 1))
            }));
        let detail = div()
            .debug_selector(|| "inspector-detail".into())
            .w(px(220.0))
            .h(px(168.0))
            .p_4()
            .rounded_lg()
            .border_1()
            .border_color(c.border)
            .on_key_down(cx.listener(|this, event: &gpui_kit::KeyDownEvent, _, cx| {
                if event.keystroke.key == "j" {
                    this.next(cx);
                    cx.stop_propagation();
                }
            }))
            .child(format!("Selected: Item {}", self.selected + 1));
        let panels = div().flex().gap_4().child(list).child(detail);
        let panels = if narrow {
            panels.flex_col()
        } else {
            panels.flex_row()
        };
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(panels)
            .child(
                button(
                    "inspector-disable",
                    if self.disabled { "Enable" } else { "Disable" },
                    false,
                )
                .debug_selector(|| "inspector-disable".into())
                .on_click(cx.listener(|this, _, _, cx| {
                    this.disabled = !this.disabled;
                    cx.notify();
                })),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, ScrollDelta, ScrollWheelEvent, TestAppContext};

    #[gpui::test]
    fn exercise_25(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, cx| InspectorPanel::new(cx));
        cx.simulate_resize(gpui::size(px(960.0), px(600.0)));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let list = cx.debug_bounds("inspector-list").expect("list missing");
        let detail = cx.debug_bounds("inspector-detail").expect("detail missing");
        assert!(
            list.origin.y == detail.origin.y && detail.origin.x > list.origin.x,
            "In a wide window the list and detail should share a row"
        );

        let second = cx.debug_bounds("inspector-row-1").expect("row missing");
        cx.simulate_click(second.center(), Modifiers::default());
        cx.update(|_, cx| assert_eq!(panel.read(cx).selected, 1));
        cx.simulate_keystrokes("j");
        cx.update(|_, cx| {
            assert_eq!(
                panel.read(cx).selected,
                2,
                "After clicking a row, J should select the next row"
            )
        });

        let disable = cx
            .debug_bounds("inspector-disable")
            .expect("disable missing");
        cx.simulate_click(disable.center(), Modifiers::default());
        let focus = cx.update(|_, cx| panel.read(cx).focus.clone());
        cx.update(|window, cx| window.focus(&focus, cx));
        cx.simulate_keystrokes("j");
        cx.update(|_, cx| {
            assert_eq!(
                panel.read(cx).selected,
                2,
                "While disabled, J should leave the selection alone"
            )
        });

        assert_eq!(
            list.size.height, detail.size.height,
            "The list should keep the detail's height and scroll inside it"
        );
        cx.simulate_event(ScrollWheelEvent {
            position: list.center(),
            delta: ScrollDelta::Pixels(gpui::point(px(0.0), px(-500.0))),
            ..Default::default()
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let last = cx.debug_bounds("inspector-row-11").expect("row missing");
        assert!(
            list.contains(&last.center()),
            "Scrolling should bring the last row into view"
        );

        cx.simulate_resize(gpui::size(px(640.0), px(560.0)));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let list = cx.debug_bounds("inspector-list").expect("list missing");
        let detail = cx.debug_bounds("inspector-detail").expect("detail missing");
        assert!(
            list.origin.x == detail.origin.x && detail.origin.y > list.origin.y,
            "In a narrow window the detail should sit below the list"
        );
    }
}
