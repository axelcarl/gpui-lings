// A setting the user chose should survive a restart, so the app writes it to a
// file and reads it back on startup. Reading can fail, though: the file may not
// exist yet, or it may hold something unexpected. Either way, the app should
// fall back to a sensible default instead of crashing.
//
// This panel saves to a temporary file, never your real settings. Try Save,
// Toggle spacing, then Load. "Write invalid data" lets you test the fallback.

use crate::theme::button;
use gpui_kit::{Context, IntoElement, Render, Window, div, prelude::*};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};

// Numbers each new panel's file, so one panel's cleanup never deletes another
// panel's setting.
static NEXT_STORE: AtomicUsize = AtomicUsize::new(0);

// Returns the stored setting: true for Compact, false for Comfortable. The
// panel calls it on startup (`at_path`) and when Load is clicked.
fn load_setting(path: &Path) -> bool {
    // TODO: This ignores the file and always returns Comfortable. Read the
    // file with `fs::read_to_string`, and return true only when its text,
    // trimmed of whitespace, is "compact". Anything else means Comfortable,
    // and so does a missing or unreadable file.
    false
}

pub struct PersistencePanel {
    // Where the setting is stored.
    path: PathBuf,
    compact: bool,
    // The result of the last button press, shown under the preview.
    status: &'static str,
}

// The playground's panel: a new file in the system's temporary directory,
// removed first so every run starts with nothing saved.
impl Default for PersistencePanel {
    fn default() -> Self {
        let id = NEXT_STORE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "gpui-lings-preference-{}-{id}.txt",
            std::process::id()
        ));
        let _ = fs::remove_file(&path);
        Self::at_path(path)
    }
}

impl PersistencePanel {
    // Restores whatever is stored at `path`, as an app does at startup.
    fn at_path(path: PathBuf) -> Self {
        let compact = load_setting(&path);
        Self {
            path,
            compact,
            status: "Ready",
        }
    }

    // Stores the setting as plain text: "compact" or "comfortable".
    fn save(&mut self, cx: &mut Context<Self>) {
        let value = if self.compact {
            "compact"
        } else {
            "comfortable"
        };
        self.status = if fs::write(&self.path, value).is_ok() {
            "Saved"
        } else {
            "Save failed"
        };
        cx.notify();
    }

    fn load(&mut self, cx: &mut Context<Self>) {
        self.compact = load_setting(&self.path);
        self.status = "Loaded";
        cx.notify();
    }
}

// Deletes the temporary file when the panel goes away.
impl Drop for PersistencePanel {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

impl Render for PersistencePanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                button("persist-toggle", "Toggle spacing", true)
                    .debug_selector(|| "persist-toggle".into())
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.compact = !this.compact;
                        cx.notify();
                    })),
            )
            .child(
                button("persist-save", "Save", false)
                    .debug_selector(|| "persist-save".into())
                    .on_click(cx.listener(|this, _, _, cx| this.save(cx))),
            )
            .child(
                button("persist-load", "Load", false)
                    .debug_selector(|| "persist-load".into())
                    .on_click(cx.listener(|this, _, _, cx| this.load(cx))),
            )
            .child(
                button("persist-corrupt", "Write invalid data", false)
                    .debug_selector(|| "persist-corrupt".into())
                    // Stores text that is neither value, to try the fallback.
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.status = if fs::write(&this.path, "not-a-setting").is_ok() {
                            "Invalid file"
                        } else {
                            "Write failed"
                        };
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .debug_selector(|| {
                        if self.compact {
                            "persist-compact".into()
                        } else {
                            "persist-comfortable".into()
                        }
                    })
                    .child(if self.compact {
                        "Compact"
                    } else {
                        "Comfortable"
                    }),
            )
            .child(div().child(self.status))
    }
}

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_35(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, _| PersistencePanel::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let toggle = cx.debug_bounds("persist-toggle").unwrap();
        let save = cx.debug_bounds("persist-save").unwrap();
        let load = cx.debug_bounds("persist-load").unwrap();
        let corrupt = cx.debug_bounds("persist-corrupt").unwrap();

        cx.simulate_click(load.center(), Modifiers::default());
        cx.update(|_, cx| assert!(!panel.read(cx).compact, "missing file uses default"));
        cx.simulate_click(toggle.center(), Modifiers::default());
        cx.simulate_click(save.center(), Modifiers::default());
        cx.update(|_, cx| {
            assert_eq!(fs::read_to_string(&panel.read(cx).path).unwrap(), "compact");
        });
        let path = cx.update(|_, cx| panel.read(cx).path.clone());
        let restored = cx.update(|_, cx| cx.new(|_| PersistencePanel::at_path(path)));
        cx.update(|_, cx| assert!(restored.read(cx).compact, "load on startup"));
        cx.simulate_click(toggle.center(), Modifiers::default());
        cx.simulate_click(load.center(), Modifiers::default());
        cx.update(|window, cx| {
            assert!(panel.read(cx).compact, "load the saved Compact setting");
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("persist-compact").is_some());

        cx.simulate_click(corrupt.center(), Modifiers::default());
        cx.simulate_click(load.center(), Modifiers::default());
        cx.update(|window, cx| {
            assert!(!panel.read(cx).compact, "invalid data uses default");
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("persist-comfortable").is_some());
    }
}
