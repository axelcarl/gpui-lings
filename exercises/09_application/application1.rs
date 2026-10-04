// Until now, the playground has created the application and the window for
// every lesson. A standalone GPUI program does that itself:
// `gpui_kit::application()` creates the platform application, `init` sets up
// GPUI Kit, and `open_window` opens a window with a root view in it.
//
// `run_standalone` below is a complete program, except that `open_workspace`
// doesn't open anything yet. The preview's Open workspace button calls the same
// function. You can also run the program by itself with
// `cargo run --example application1`.

use crate::theme::button;
use gpui_kit::{App, Context, Entity, IntoElement, Render, Window, WindowOptions, div, prelude::*};

// The view the new window shows. It has no state, so it is a unit struct.
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

// Opens the workspace window and returns its root view, or None when no window
// could be opened. Both the Open workspace button and run_standalone call it.
// `App` is the application context: it owns every window and entity.
pub fn open_workspace(cx: &mut App) -> Option<Entity<WorkspaceRoot>> {
    // `gpui_kit::open_window(options, cx, build)` opens a window, then calls
    // `build` with that window and `cx` to create its root view entity. It
    // returns a Result holding the window's handle and that root entity.
    // TODO: No window opens yet. Open one whose root view is a
    // `WorkspaceRoot`, and return that root entity.
    None
}

// The standalone program. `application()` creates the platform application
// and `run` starts its event loop, calling the closure once the app is ready.
// GPUI Kit's `init` must run before anything else uses the Kit: it installs
// the Kit's theme and the key bindings of its controls.
pub fn run_standalone() {
    gpui_kit::application().run(|cx| {
        gpui_kit::init(cx);
        let _ = open_workspace(cx);
    });
}

#[derive(Default)]
pub struct StartupPanel {
    // The workspace window's root view, once it is open. Holding it also lets
    // the button open the window only once.
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
                    // `cx` here is a `Context<StartupPanel>`, which derefs to
                    // `App`, so it can be passed to open_workspace directly.
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

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_33(cx: &mut TestAppContext) {
        cx.update(gpui_kit::init);
        let (panel, cx) = cx.add_window_view(|_, _| StartupPanel::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let windows_before = cx.update(|_, cx| cx.windows().len());
        let open = cx.debug_bounds("startup-open").unwrap();
        cx.simulate_click(open.center(), Modifiers::default());
        cx.update(|window, cx| {
            assert_eq!(
                cx.windows().len(),
                windows_before + 1,
                "Open workspace should open one new window"
            );
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
