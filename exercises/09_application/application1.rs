//! 29 — Start a standalone GPUI app
//!
//! The playground gives every earlier lesson an application and window. A
//! standalone program must create those itself. GPUI Kit's application()
//! supplies the platform Application; init() registers its shared layers;
//! open_window() installs a root view in the first window. The separate
//! `playground/examples/lesson29.rs` entrypoint calls run_standalone().
//!
//! Goal: Open workspace should create a new window containing WorkspaceRoot.
//! Replace the placeholder in open_workspace with gpui_kit::open_window,
//! build WorkspaceRoot in its closure, and return the root entity. The native
//! preview button uses the same function; the starter playground still runs.
//!
//! Example — Installing a root view in a new window:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! let (_, root) = gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| {
//!     cx.new(|_| WelcomeView)
//! })?;
//! // WelcomeView implements Render; root is its Entity handle.
//! ```

use crate::theme::button;
use gpui_kit::{App, Context, Entity, IntoElement, Render, Window, WindowOptions, div, prelude::*};

pub struct WorkspaceRoot;

impl Render for WorkspaceRoot {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .debug_selector(|| "workspace-root".into())
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .child("My GPUI workspace")
    }
}

// TODO: Open a window with WorkspaceRoot as its root view.
pub fn open_workspace(cx: &mut App) -> Option<Entity<WorkspaceRoot>> {
    let _ = (cx, WindowOptions::default());
    None
}

pub fn run_standalone() {
    gpui_kit::application().run(|cx| {
        gpui_kit::init(cx);
        let _ = open_workspace(cx);
    });
}

#[derive(Default)]
pub struct StartupPanel {
    opened: Option<Entity<WorkspaceRoot>>,
}

impl Render for StartupPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                button("startup-open", "Open workspace", true)
                    .debug_selector(|| "startup-open".into())
                    .on_click(cx.listener(|this, _, _, cx| {
                        if this.opened.is_none() {
                            this.opened = open_workspace(cx);
                            cx.notify();
                        }
                    })),
            )
            .child(
                div()
                    .debug_selector(|| {
                        if self.opened.is_some() {
                            "startup-opened".into()
                        } else {
                            "startup-waiting".into()
                        }
                    })
                    .child(if self.opened.is_some() {
                        "Window opened"
                    } else {
                        "No workspace window yet"
                    }),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_29(cx: &mut TestAppContext) {
        cx.update(gpui_kit::init);
        let (panel, cx) = cx.add_window_view(|_, _| StartupPanel::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let windows_before = cx.update(|_, cx| cx.windows().len());
        let open = cx.debug_bounds("startup-open").unwrap();
        cx.simulate_click(open.center(), Modifiers::default());
        cx.update(|window, cx| {
            assert_eq!(cx.windows().len(), windows_before + 1);
            assert!(panel.read(cx).opened.is_some(), "keep the new root entity");
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("startup-opened").is_some());
    }

    #[gpui::test]
    fn workspace_root_renders(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, _| WorkspaceRoot);
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(cx.debug_bounds("workspace-root").is_some());
    }
}
