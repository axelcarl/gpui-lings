// Screen readers don't read pixels. GPUI describes the window to them as an
// accessibility tree, built with the AccessKit library: one node per control,
// with a role (what it is), a name (what it's called) and a state (such as on
// or off).
//
// GPUI Base's unstyled `Switch` gives you the role, the state, and activation
// by mouse and keyboard. The name has to come from you. The text inside the
// switch doesn't count: it's a visible child, not the switch's label.

use crate::theme::colors;
// GPUI Base's Switch brings behavior and accessibility, but no look of its own.
use gpui_kit::base::Switch;
use gpui_kit::{App, ClickEvent, Context, IntoElement, Render, Window, div, prelude::*, px};

#[derive(Default)]
pub struct AccessibilityPanel {
    // The switch's value. The view owns it; the Switch only displays it.
    enabled: bool,
}

impl AccessibilityPanel {
    // Builds the alerts switch. It is a separate function so the check can also
    // build it on its own and inspect its accessibility node. `on_change` runs
    // on a click, or on Space while the switch has focus. It receives the
    // *next* value, the event, the window and the app.
    fn control(
        enabled: bool,
        on_change: impl Fn(bool, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Switch {
        Switch::new("alerts-switch")
            // The value to show, and to expose as the toggled state.
            .checked(enabled)
            // TODO: The switch has no name for a screen reader to announce.
            // Name it "Enable alerts" with Switch's `accessibility_label` method.
            .on_change(on_change)
            // Everything from here on is the app's own styling.
            .w(px(220.0))
            .h(px(48.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded_lg()
            .bg(colors().muted)
            .child(if enabled { "Alerts on" } else { "Alerts off" })
    }
}

impl Render for AccessibilityPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // `control` takes a plain callback that isn't tied to this view, so it
        // reaches the view through a weak handle (lesson 17).
        let weak = cx.weak_entity();
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                div()
                    .debug_selector(|| "alerts-control".into())
                    .child(Self::control(self.enabled, move |next, _, _, cx| {
                        // Store the value the switch asks for, then render again.
                        let _ = weak.update(cx, |this, cx| {
                            this.enabled = next;
                            cx.notify();
                        });
                    })),
            )
            .child(
                // The status the check looks for: "alerts-on" or "alerts-off".
                div()
                    .debug_selector(|| {
                        if self.enabled {
                            "alerts-on".into()
                        } else {
                            "alerts-off".into()
                        }
                    })
                    .child(if self.enabled { "On" } else { "Off" }),
            )
    }
}

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{
        KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, Role, TestAppContext, Toggled, accesskit,
        canvas,
    };
    use std::sync::{Arc, Mutex};

    // Builds the switch on its own and records the AccessKit node it writes:
    // what assistive technology would be told about the control.
    struct SemanticsProbe(Arc<Mutex<Option<accesskit::Node>>>);
    impl Render for SemanticsProbe {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let captured = self.0.clone();
            canvas(
                move |_, window, cx| {
                    let mut node = accesskit::Node::new(Role::Switch);
                    AccessibilityPanel::control(true, |_, _, _, _| {})
                        .render(window, cx)
                        .into_element()
                        .write_a11y_info(&mut node);
                    *captured.lock().unwrap() = Some(node);
                },
                |_, _, _, _| {},
            )
        }
    }

    #[gpui::test]
    fn exercise_38(cx: &mut TestAppContext) {
        let (panel, window) = cx.add_window_view(|_, _| AccessibilityPanel::default());
        window.update(|window, cx| window.draw(cx).clear(cx));
        let control = window.debug_bounds("alerts-control").unwrap();
        window.simulate_click(control.center(), Modifiers::default());
        window.update(|window, cx| {
            assert!(panel.read(cx).enabled, "a click should switch alerts on");
            window.draw(cx).clear(cx);
        });
        assert!(window.debug_bounds("alerts-on").is_some());
        let space = Keystroke::parse("space").unwrap();
        window.simulate_event(KeyDownEvent {
            keystroke: space.clone(),
            is_held: false,
            prefer_character_input: false,
        });
        window.simulate_event(KeyUpEvent { keystroke: space });
        window.update(|window, cx| {
            assert!(!panel.read(cx).enabled, "Space should activate the switch");
            window.draw(cx).clear(cx);
        });
        assert!(window.debug_bounds("alerts-off").is_some());

        let captured = Arc::new(Mutex::new(None));
        let (_, probe) = window.cx.add_window_view({
            let captured = captured.clone();
            move |_, _| SemanticsProbe(captured)
        });
        probe.update(|window, cx| window.draw(cx).clear(cx));
        let node = captured.lock().unwrap().take().unwrap();
        assert_eq!(node.role(), Role::Switch);
        assert_eq!(node.toggled(), Some(Toggled::True));
        assert_eq!(node.label(), Some("Enable alerts"), "name the control");
        assert!(node.supports_action(accesskit::Action::Click));
    }
}
