//! 33 — Follow appearance changes
//!
//! A view can read Window::appearance and observe later changes. Keep colors
//! in semantic tokens so the same content remains legible in light and dark
//! modes. The preview buttons override appearance for deterministic practice;
//! Follow system removes the override. The window observer still handles real
//! system changes while the app is running.
//!
//! Goal: Preview dark should show light foreground text on the dark surface.
//! Read the semantic foreground token instead of using the surface color.
//!
//! Example — Checking window appearance:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! let dark = matches!(window.appearance(),
//!     WindowAppearance::Dark | WindowAppearance::VibrantDark);
//! let label = if dark { "Dark appearance" } else { "Light appearance" };
//! div().child(label)
//! ```

// DARK and LIGHT are the playground's two palettes.
use crate::theme::{DARK, LIGHT, button, focus_ring};
use gpui_kit::{
    Context, IntoElement, Render, Rgba, Subscription, Window, WindowAppearance, div, prelude::*, px,
};

pub struct AppearancePanel {
    // An appearance chosen with Switch preview, or None to follow the system.
    preview: Option<WindowAppearance>,
    _appearance: Subscription,
}

impl AppearancePanel {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            preview: None,
            // Runs when the system switches between light and dark while this
            // window is open, with this view, the window and its context.
            // Notifying renders the panel again in the new appearance.
            _appearance: cx.observe_window_appearance(window, |_, _, cx| cx.notify()),
        }
    }

    // Picks the panel's colors: (background, text color, mode name). `system`
    // is the window's real appearance; a preview overrides it.
    fn tones(&self, system: WindowAppearance) -> (Rgba, Rgba, &'static str) {
        // GPUI has four appearances. Both Dark and VibrantDark count as dark.
        let dark = matches!(
            self.preview.unwrap_or(system),
            WindowAppearance::Dark | WindowAppearance::VibrantDark
        );
        // A palette holds semantic tokens, named for their role rather than
        // their color: `background` for surfaces, `foreground` for text on
        // them. Each palette pairs the two so they contrast.
        let palette = if dark { &DARK } else { &LIGHT };
        // TODO: In dark mode the text gets the background token, so it
        // vanishes into the surface. Use the palette's foreground token in both
        // appearances.
        let foreground = if dark {
            palette.background
        } else {
            palette.foreground
        };
        (
            palette.background,
            foreground,
            if dark { "dark" } else { "light" },
        )
    }
}

impl Render for AppearancePanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // `window.appearance()` is the appearance the system gives this window.
        let (background, foreground, mode) = self.tones(window.appearance());
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(16.0))
            .child(
                button("appearance-switch", "Switch preview", true)
                    .debug_selector(|| "appearance-switch".into())
                    // Preview the opposite of what the panel shows now.
                    .on_click(cx.listener(|this, _, window, cx| {
                        let current_dark = this.tones(window.appearance()).2 == "dark";
                        this.preview = Some(if current_dark {
                            WindowAppearance::Light
                        } else {
                            WindowAppearance::Dark
                        });
                        cx.notify();
                    })),
            )
            .child(
                button("appearance-system", "Follow system", false)
                    .debug_selector(|| "appearance-system".into())
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.preview = None;
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .debug_selector(move || format!("appearance-{mode}"))
                    .w(px(240.0))
                    .h(px(96.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_lg()
                    .focus_visible(focus_ring)
                    .bg(background)
                    .text_color(foreground)
                    .child(format!("{mode} appearance")),
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
    fn exercise_33(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(AppearancePanel::new);
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let switch = cx.debug_bounds("appearance-switch").unwrap();
        let system = cx.debug_bounds("appearance-system").unwrap();
        cx.update(|window, cx| {
            let (background, foreground, mode) = panel.read(cx).tones(window.appearance());
            assert_eq!(mode, "light");
            assert_eq!(background, LIGHT.background);
            assert_eq!(foreground, LIGHT.foreground);
        });
        cx.simulate_click(switch.center(), Modifiers::default());
        cx.update(|window, cx| {
            let (background, foreground, mode) = panel.read(cx).tones(window.appearance());
            assert_eq!(mode, "dark");
            assert_eq!(background, DARK.background);
            assert_eq!(foreground, DARK.foreground, "dark text must remain legible");
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("appearance-dark").is_some());
        cx.simulate_click(switch.center(), Modifiers::default());
        cx.update(|window, cx| {
            assert_eq!(panel.read(cx).tones(window.appearance()).2, "light");
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("appearance-light").is_some());
        cx.simulate_click(system.center(), Modifiers::default());
        cx.update(|_, cx| assert!(panel.read(cx).preview.is_none()));
        cx.update(|_, cx| {
            assert_eq!(
                panel.read(cx).tones(WindowAppearance::VibrantDark).2,
                "dark"
            );
        });
    }
}
