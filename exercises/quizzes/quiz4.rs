//! 38 — Quiz 4 (capstone): small native workspace
//!
//! This is a quiz for the whole course. Its five bugs come from the chapters
//! on contexts, lifetimes & async, responsive views, async data and focus.
//!
//! This two-pane notes workspace brings earlier chapters together. Separate
//! entities render the list and detail from one observed model. The panes
//! stack in a narrow window. Loading fails once and can be retried using a
//! simulated clock. A focused Ctrl-S action saves the selected note to an
//! isolated temporary file, and the save control has an accessible name.
//!
//! Nothing marks the broken lines this time. Testers reported:
//!
//! - Load shows Loading, but neither the error nor the notes ever arrive.
//! - Retry leaves the old error on screen until the next reply arrives.
//! - The panes stack in a wide window and sit side by side in a narrow one.
//! - Save stores the first note's index, whichever note is selected.
//! - Ctrl-S does nothing, even after clicking into the workspace.
//!
//! Goal: find and fix all five. The check stops at the first symptom it
//! sees; reproduce it in the preview, then trace it back to the code. Each
//! fix reuses an idea from an earlier chapter.
//!
//! Example — Updating a shared workspace model:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! self.model.update(cx, |model, cx| {
//!     model.selected = next_index;
//!     cx.notify();
//! });
//! // Observers redraw list and detail from the same selected value.
//! ```

use crate::theme::{button, colors};
use gpui_kit::{
    Context, Entity, FocusHandle, IntoElement, KeyBinding, Render, Subscription, Task, Window,
    actions, div, prelude::*, px,
};
use std::{
    collections::VecDeque,
    fs,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

actions!(gpui_lings_capstone, [SaveNote]);
static NEXT_WORKSPACE: AtomicUsize = AtomicUsize::new(0);

#[derive(Default)]
struct NotesModel {
    notes: Vec<&'static str>,
    selected: usize,
}

struct NotesList {
    model: Entity<NotesModel>,
    _observe: Subscription,
}
impl NotesList {
    fn new(model: Entity<NotesModel>, cx: &mut Context<Self>) -> Self {
        let observe = cx.observe(&model, |_, _, cx| cx.notify());
        Self {
            model,
            _observe: observe,
        }
    }
}
impl Render for NotesList {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let model = self.model.read(cx);
        let c = colors();
        div()
            .debug_selector(|| "workspace-list".into())
            .w(px(180.0))
            .p_2()
            .border_1()
            .border_color(c.border)
            .rounded_lg()
            .flex()
            .flex_col()
            .gap_2()
            .children(model.notes.iter().enumerate().map(|(ix, title)| {
                let source = self.model.clone();
                div()
                    .id(ix)
                    .debug_selector(move || format!("workspace-note-{ix}"))
                    .p_2()
                    .rounded_md()
                    .cursor_pointer()
                    .when(model.selected == ix, |row| row.bg(c.muted))
                    .on_click(move |_, _, cx| {
                        source.update(cx, |model, cx| {
                            model.selected = ix;
                            cx.notify();
                        });
                    })
                    .child(*title)
            }))
    }
}

struct NoteDetail {
    model: Entity<NotesModel>,
    _observe: Subscription,
}
impl NoteDetail {
    fn new(model: Entity<NotesModel>, cx: &mut Context<Self>) -> Self {
        let observe = cx.observe(&model, |_, _, cx| cx.notify());
        Self {
            model,
            _observe: observe,
        }
    }
}
impl Render for NoteDetail {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let model = self.model.read(cx);
        let title = model
            .notes
            .get(model.selected)
            .copied()
            .unwrap_or("No note");
        let c = colors();
        div()
            .debug_selector(|| "workspace-detail".into())
            .w(px(250.0))
            .min_h(px(100.0))
            .p_3()
            .border_1()
            .border_color(c.border)
            .rounded_lg()
            .child(title)
    }
}

pub struct CapstonePanel {
    model: Entity<NotesModel>,
    list: Entity<NotesList>,
    detail: Entity<NoteDetail>,
    focus: FocusHandle,
    replies: VecDeque<Result<Vec<&'static str>, &'static str>>,
    status: &'static str,
    selected: usize,
    _task: Option<Task<()>>,
    path: PathBuf,
}

impl CapstonePanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        cx.bind_keys([KeyBinding::new("ctrl-s", SaveNote, Some("NotesWorkspace"))]);
        let model = cx.new(|_| NotesModel::default());
        let list = cx.new(|cx| NotesList::new(model.clone(), cx));
        let detail = cx.new(|cx| NoteDetail::new(model.clone(), cx));
        let id = NEXT_WORKSPACE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "gpui-lings-workspace-{}-{id}.txt",
            std::process::id()
        ));
        let _ = fs::remove_file(&path);
        Self {
            model,
            list,
            detail,
            focus: cx.focus_handle(),
            replies: [Err("Offline"), Ok(vec!["Outline", "Review", "Ship"])].into(),
            status: "Idle",
            selected: 0,
            _task: None,
            path,
        }
    }

    fn request(&mut self, cx: &mut Context<Self>) {
        let reply = self.replies.pop_front().unwrap_or(Err("No response"));
        let _task = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(Duration::from_secs(1)).await;
            let _ = this.update(cx, |this, cx| {
                match reply {
                    Ok(notes) => {
                        this.model.update(cx, |model, cx| {
                            model.notes = notes;
                            model.selected = 0;
                            cx.notify();
                        });
                        this.status = "Ready";
                    }
                    Err(message) => this.status = message,
                }
                cx.notify();
            });
        });
    }

    fn load(&mut self, cx: &mut Context<Self>) {
        self.status = "Loading";
        cx.notify();
        self.request(cx);
    }

    fn retry(&mut self, cx: &mut Context<Self>) {
        self.request(cx);
    }

    fn save(&mut self, cx: &mut Context<Self>) {
        let selected = self.selected;
        self.status = if fs::write(&self.path, selected.to_string()).is_ok() {
            "Saved"
        } else {
            "Save failed"
        };
        cx.notify();
    }
}

impl Drop for CapstonePanel {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

impl Render for CapstonePanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let narrow = window.bounds().size.width > px(620.0);
        let c = colors();
        let panes = div()
            .flex()
            .gap_3()
            .when(narrow, |el| el.flex_col())
            .when(!narrow, |el| el.flex_row())
            .child(self.list.clone())
            .child(self.detail.clone());
        div()
            .id("notes-workspace")
            .key_context("NotesWorkspace")
            .on_action(cx.listener(|this, _: &SaveNote, _, cx| this.save(cx)))
            .flex()
            .flex_col()
            .gap_3()
            .text_color(c.foreground)
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        button("workspace-load", "Load", true)
                            .debug_selector(|| "workspace-load".into())
                            .on_click(cx.listener(|this, _, _, cx| this.load(cx))),
                    )
                    .child(
                        button("workspace-retry", "Retry", false)
                            .debug_selector(|| "workspace-retry".into())
                            .on_click(cx.listener(|this, _, _, cx| this.retry(cx))),
                    )
                    .child(
                        button("workspace-save", "Save", false)
                            .aria_label("Save selected note")
                            .debug_selector(|| "workspace-save".into())
                            .on_click(cx.listener(|this, _, window, cx| {
                                window.focus(&this.focus, cx);
                                this.save(cx);
                            })),
                    ),
            )
            .child(
                div()
                    .debug_selector(|| format!("workspace-status-{}", self.status))
                    .child(self.status),
            )
            .child(panes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext, VisualTestContext};

    #[gpui::test]
    fn exercise_38(cx: &mut TestAppContext) {
        let (panel, window) = cx.add_window_view(|_, cx| CapstonePanel::new(cx));
        window.simulate_resize(gpui::size(px(800.0), px(560.0)));
        window.update(|window, cx| window.draw(cx).clear(cx));
        let load = window.debug_bounds("workspace-load").unwrap();
        let retry = window.debug_bounds("workspace-retry").unwrap();
        window.simulate_click(load.center(), Modifiers::default());
        window.executor().advance_clock(Duration::from_secs(1));
        window.run_until_parked();
        window.update(|window, cx| {
            assert_eq!(
                panel.read(cx).status,
                "Offline",
                "Load should report its first reply after one second"
            );
            window.draw(cx).clear(cx);
        });
        window.simulate_click(retry.center(), Modifiers::default());
        window.update(|window, cx| {
            assert_eq!(
                panel.read(cx).status,
                "Loading",
                "Retry should replace the error with Loading immediately"
            );
            window.draw(cx).clear(cx);
        });
        assert!(window.debug_bounds("workspace-status-Loading").is_some());
        window.executor().advance_clock(Duration::from_secs(1));
        window.run_until_parked();
        window.update(|window, cx| {
            assert_eq!(panel.read(cx).status, "Ready");
            window.draw(cx).clear(cx);
        });
        let list = window.debug_bounds("workspace-list").unwrap();
        let detail = window.debug_bounds("workspace-detail").unwrap();
        assert!(
            detail.origin.x > list.origin.x,
            "In a wide window the panes should sit side by side"
        );

        select(window, "workspace-note-1");
        let save = window.debug_bounds("workspace-save").unwrap();
        window.simulate_click(save.center(), Modifiers::default());
        assert_eq!(
            saved(window, &panel).as_deref(),
            Some("1"),
            "Save should store the selected note's index"
        );
        select(window, "workspace-note-2");
        window.simulate_keystrokes("ctrl-s");
        assert_eq!(
            saved(window, &panel).as_deref(),
            Some("2"),
            "Ctrl-S should save the selected note once the workspace has focus"
        );

        window.simulate_resize(gpui::size(px(540.0), px(560.0)));
        window.update(|window, cx| window.draw(cx).clear(cx));
        let list = window.debug_bounds("workspace-list").unwrap();
        let detail = window.debug_bounds("workspace-detail").unwrap();
        assert!(
            detail.origin.y > list.origin.y,
            "In a narrow window the panes should stack"
        );
    }

    fn select(window: &mut VisualTestContext, row: &'static str) {
        let row = window.debug_bounds(row).unwrap();
        window.simulate_click(row.center(), Modifiers::default());
        window.update(|window, cx| window.draw(cx).clear(cx));
    }

    fn saved(window: &mut VisualTestContext, panel: &Entity<CapstonePanel>) -> Option<String> {
        window.update(|_, cx| fs::read_to_string(&panel.read(cx).path).ok())
    }
}
