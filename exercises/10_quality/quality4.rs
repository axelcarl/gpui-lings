// A reusable control takes everything it shows as input: its value, whether
// it's disabled, its label, and a callback for changes. The view that uses it
// owns the value. Here, `setting_switch` wraps GPUI Base's `Switch` in the
// course theme, and three tiles use it.
//
// `Switch` is a *controlled* component: it keeps no value of its own. Each
// render tells it the current value with `checked(...)`, and when the user
// activates it, `on_change` receives the opposite of that value for the owner
// to store. Try both live switches, and the disabled one.

use crate::theme::colors;
use gpui_kit::base::Switch;
use gpui_kit::{
    App, ClickEvent, Context, Entity, IntoElement, Render, Window, div, prelude::*, px,
};

// A themed switch that any view can reuse. It holds no state: the caller passes
// every input on each render, and gets the next value back through
// `on_change`, with the click event, the window and the app.
fn setting_switch(
    id: &'static str,
    label: &'static str,
    checked: bool,
    disabled: bool,
    on_change: impl Fn(bool, &ClickEvent, &mut Window, &mut App) + 'static,
) -> Switch {
    let c = colors();
    Switch::new(id)
        // TODO: The Switch is always told it is off, so it announces Off and
        // every click reports `true` ("on") again. Give it the value this
        // wrapper receives in `checked` instead.
        .checked(false)
        // While disabled, the Switch ignores clicks and keys and can't be focused.
        .disabled(disabled)
        // The name a screen reader announces (lesson 38).
        .accessibility_label(label)
        .on_change(on_change)
        // The course theme. The colors and text already follow `checked`.
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

// One setting in its own view. Each tile owns its value, so the tiles change
// independently even though they share `setting_switch`.
pub struct SettingTile {
    id: &'static str,
    label: &'static str,
    checked: bool,
    disabled: bool,
}

impl Render for SettingTile {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // The switch's callback isn't tied to this view, so it reaches the tile
        // through a weak handle.
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
                    // Store the requested value. The next render passes it back
                    // into `setting_switch`.
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

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_41(cx: &mut TestAppContext) {
        let (panel, window) = cx.add_window_view(|_, cx| ReusablePanel::new(cx));
        window.update(|window, cx| window.draw(cx).clear(cx));
        let first = window.debug_bounds("setting-first").unwrap();
        let second = window.debug_bounds("setting-second").unwrap();
        let locked = window.debug_bounds("setting-locked").unwrap();
        window.simulate_click(first.center(), Modifiers::default());
        window.update(|window, cx| {
            assert!(
                panel.read(cx).first.read(cx).checked,
                "clicking Sounds should turn it on"
            );
            assert!(
                !panel.read(cx).second.read(cx).checked,
                "Alerts should stay off: each tile owns its own value"
            );
            window.draw(cx).clear(cx);
        });
        // GPUI Base's controlled switch must reflect the value on the next click.
        window.simulate_click(first.center(), Modifiers::default());
        window.update(|window, cx| {
            assert!(
                !panel.read(cx).first.read(cx).checked,
                "a second click should turn Sounds off again"
            );
            window.draw(cx).clear(cx);
        });
        window.simulate_click(second.center(), Modifiers::default());
        window.simulate_click(locked.center(), Modifiers::default());
        window.update(|window, cx| {
            assert!(
                !panel.read(cx).first.read(cx).checked,
                "Sounds should stay off"
            );
            assert!(
                panel.read(cx).second.read(cx).checked,
                "clicking Alerts should turn it on"
            );
            assert!(
                !panel.read(cx).locked.read(cx).checked,
                "the disabled Managed switch must ignore clicks"
            );
            window.draw(cx).clear(cx);
        });
    }
}
