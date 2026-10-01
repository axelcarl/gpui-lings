//! 34 — Expose an accessible control
//!
//! GPUI Base's unstyled Switch supplies the Switch role, toggled state, and
//! pointer/keyboard activation. Application code still needs to give it a
//! meaningful name. A child label is visible, but it is not a substitute for
//! the control's accessible label. Keep the checked value in the owning view.
//!
//! Goal: give the Alerts switch the name "Enable alerts". Its AccessKit node
//! should expose that label together with Switch and the current toggled
//! state. Click and Space must still change the value.

use crate::theme::colors;
use gpui_kit::base::Switch;
use gpui_kit::{App, ClickEvent, Context, IntoElement, Render, Window, div, prelude::*, px};

#[derive(Default)]
pub struct AccessibilityPanel {
    enabled: bool,
}

impl AccessibilityPanel {
    // TODO: Give the switch an accessible name.
    fn control(
        enabled: bool,
        on_change: impl Fn(bool, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Switch {
        Switch::new("alerts-switch")
            .checked(enabled)
            .on_change(on_change)
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
                        let _ = weak.update(cx, |this, cx| {
                            this.enabled = next;
                            cx.notify();
                        });
                    })),
            )
            .child(
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{
        KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, Role, TestAppContext, Toggled, accesskit,
        canvas,
    };
    use std::sync::{Arc, Mutex};

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
    fn exercise_34(cx: &mut TestAppContext) {
        let (panel, window) = cx.add_window_view(|_, _| AccessibilityPanel::default());
        window.update(|window, cx| window.draw(cx).clear(cx));
        let control = window.debug_bounds("alerts-control").unwrap();
        window.simulate_click(control.center(), Modifiers::default());
        window.update(|window, cx| {
            assert!(panel.read(cx).enabled);
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
