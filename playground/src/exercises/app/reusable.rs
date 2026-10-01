//! 37 — Compose a reusable component
//!
//! A reusable control should take its value, disabled state, label, and change
//! handler as inputs. The parent view owns each value. GPUI Base's Switch
//! handles accessible interaction, while this small wrapper applies the course
//! theme and passes changes back to the owner. Two child views use it, plus a
//! disabled example.
//!
//! Goal: both live switches update independently. The starter wrapper ignores
//! its checked input, so a redraw loses the visible checked state. Pass that
//! input through while keeping the disabled example inert.

use crate::theme::colors;
use gpui_kit::base::Switch;
use gpui_kit::{
    App, ClickEvent, Context, Entity, IntoElement, Render, Window, div, prelude::*, px,
};

// TODO: The switch must show the checked value it receives.
fn setting_switch(
    id: &'static str,
    label: &'static str,
    checked: bool,
    disabled: bool,
    on_change: impl Fn(bool, &ClickEvent, &mut Window, &mut App) + 'static,
) -> Switch {
    let c = colors();
    Switch::new(id)
        .checked(false)
        .disabled(disabled)
        .accessibility_label(label)
        .on_change(on_change)
        .w(px(220.0))
        .h(px(44.0))
        .flex()
        .items_center()
        .px_3()
        .rounded_lg()
        .bg(if checked { c.primary } else { c.muted })
        .text_color(if checked {
            c.primary_foreground
        } else {
            c.foreground
        })
        .child(format!("{label}: {}", if checked { "On" } else { "Off" }))
}

pub struct SettingTile {
    id: &'static str,
    label: &'static str,
    checked: bool,
    disabled: bool,
}

impl Render for SettingTile {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let weak = cx.weak_entity();
        let id = self.id;
        div()
            .w(px(220.0))
            .debug_selector(move || id.into())
            .child(setting_switch(
                self.id,
                self.label,
                self.checked,
                self.disabled,
                move |next, _, _, cx| {
                    let _ = weak.update(cx, |this, cx| {
                        this.checked = next;
                        cx.notify();
                    });
                },
            ))
    }
}

pub struct ReusablePanel {
    first: Entity<SettingTile>,
    second: Entity<SettingTile>,
    locked: Entity<SettingTile>,
}

impl ReusablePanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let tile = |cx: &mut Context<Self>, id, label, disabled| {
            cx.new(|_| SettingTile {
                id,
                label,
                checked: false,
                disabled,
            })
        };
        Self {
            first: tile(cx, "setting-first", "Sounds", false),
            second: tile(cx, "setting-second", "Alerts", false),
            locked: tile(cx, "setting-locked", "Managed", true),
        }
    }
}

impl Render for ReusablePanel {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(self.first.clone())
            .child(self.second.clone())
            .child(self.locked.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_37(cx: &mut TestAppContext) {
        let (panel, window) = cx.add_window_view(|_, cx| ReusablePanel::new(cx));
        window.update(|window, cx| window.draw(cx).clear(cx));
        let first = window.debug_bounds("setting-first").unwrap();
        let second = window.debug_bounds("setting-second").unwrap();
        let locked = window.debug_bounds("setting-locked").unwrap();
        window.simulate_click(first.center(), Modifiers::default());
        window.update(|window, cx| {
            assert!(panel.read(cx).first.read(cx).checked);
            assert!(!panel.read(cx).second.read(cx).checked);
            window.draw(cx).clear(cx);
        });
        // GPUI Base's controlled switch must reflect the value on the next click.
        window.simulate_click(first.center(), Modifiers::default());
        window.update(|window, cx| {
            assert!(!panel.read(cx).first.read(cx).checked);
            window.draw(cx).clear(cx);
        });
        window.simulate_click(second.center(), Modifiers::default());
        window.simulate_click(locked.center(), Modifiers::default());
        window.update(|window, cx| {
            assert!(!panel.read(cx).first.read(cx).checked);
            assert!(panel.read(cx).second.read(cx).checked);
            assert!(!panel.read(cx).locked.read(cx).checked);
            window.draw(cx).clear(cx);
        });
    }
}
