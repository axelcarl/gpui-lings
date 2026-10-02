// Starter code leaves some callback arguments and placeholders unused until the
// learner fills them in. Keep those warnings out of every lesson's check output.
#[allow(unused)]
pub mod exercises;
mod icons;
pub use gpui_lings_shared::lessons;
mod preview;
mod theme;

use gpui_kit::prelude::*;
use gpui_kit::*;
use gpui_lings_shared::preview::{PreviewState, Refresh};
use lessons::LESSONS;
use std::{path::PathBuf, time::Duration};
use theme::{MONO, Variant, alpha, badge, button, button_base, code, colors, icon};

/// The terminal's latest check of the current lesson.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Status {
    Unchecked,
    Passed,
    Failed,
    Error,
}

impl Status {
    fn parse(value: &str) -> Self {
        match value {
            "passed" => Self::Passed,
            "failed" => Self::Failed,
            "error" => Self::Error,
            _ => Self::Unchecked,
        }
    }

    /// Green once the learner can move on, red while the check fails.
    fn tone(self) -> Option<Rgba> {
        match self {
            Self::Passed => Some(colors().success),
            Self::Failed | Self::Error => Some(colors().destructive),
            Self::Unchecked => None,
        }
    }
}

struct Playground {
    index: usize,
    completed: usize,
    count: u32,
    status: Status,
    refresh: Refresh,
    _preview_updates: Option<Task<()>>,
    _bounds: Option<Subscription>,
    /// GPUI_LINGS_APPEARANCE=light|dark overrides the system appearance.
    forced_appearance: Option<WindowAppearance>,
    toggle: Entity<exercises::views::entity::TogglePanel>,
    advanced: Option<AnyView>,
    _appearance: Subscription,
}

impl Playground {
    fn new(
        index: usize,
        completed: usize,
        status: Status,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let forced_appearance = match std::env::var("GPUI_LINGS_APPEARANCE").as_deref() {
            Ok("light") => Some(WindowAppearance::Light),
            Ok("dark") => Some(WindowAppearance::Dark),
            _ => None,
        };
        let updates = std::env::var_os("GPUI_LINGS_PREVIEW_STATE").map(|path| {
            let path = PathBuf::from(path);
            cx.spawn(async move |this, cx| {
                loop {
                    cx.background_executor()
                        .timer(Duration::from_millis(100))
                        .await;
                    let Some(state) = PreviewState::read(&path) else {
                        continue;
                    };
                    if this
                        .update(cx, |this, cx| {
                            if this.apply_preview_state(&state) {
                                cx.notify();
                            }
                        })
                        .is_err()
                    {
                        break;
                    }
                }
            })
        });
        Self {
            index,
            completed,
            count: 0,
            status,
            refresh: Refresh::Current,
            _preview_updates: updates,
            _bounds: None,
            forced_appearance,
            toggle: cx.new(|_| exercises::views::entity::TogglePanel::new()),
            advanced: exercises::advanced_preview(index, window, cx),
            _appearance: cx.observe_window_appearance(window, |_, _, cx| cx.notify()),
        }
    }

    fn apply_preview_state(&mut self, state: &PreviewState) -> bool {
        let previous = (self.status, self.refresh, self.completed);
        // Navigation can check a different lesson while this old preview stays open.
        if !matches!(state.refresh, Refresh::Checking | Refresh::Building) {
            self.completed = state.completed;
        }
        if state.index == self.index && state.refresh == Refresh::Current {
            self.status = Status::parse(&state.status);
        }
        self.refresh = state.refresh;
        previous != (self.status, self.refresh, self.completed)
    }

    fn status_tone(&self) -> Option<Rgba> {
        match self.refresh {
            Refresh::Checking | Refresh::Building => Some(colors().loading),
            Refresh::Failed => Some(colors().destructive),
            Refresh::Current => self.status.tone(),
        }
    }

    fn completed_count(&self) -> usize {
        self.completed.min(LESSONS.len())
    }

    fn header(&self) -> impl IntoElement {
        let c = colors();
        let (section, page) = match LESSONS.get(self.index) {
            Some(lesson) => (lesson.chapter, format!("Lesson {}", lesson.id)),
            None => ("GPUI Lings", "Complete".into()),
        };
        let done = self.completed_count();
        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_between()
            .gap_4()
            .h(px(56.0))
            .px_6()
            .border_b_1()
            .border_color(c.border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1p5()
                    .min_w_0()
                    .child(div().text_color(c.muted_foreground).child(section))
                    .child(
                        icon(icons::CHEVRON_RIGHT)
                            .size_3p5()
                            .text_color(c.muted_foreground),
                    )
                    .child(page),
            )
            .child(
                div()
                    .debug_selector(|| "lesson-progress".into())
                    .flex()
                    .flex_shrink_0()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .text_xs()
                            .text_color(c.muted_foreground)
                            .child(format!("{done} of {}", LESSONS.len())),
                    )
                    .child(
                        div()
                            .w(px(128.0))
                            .h(px(6.0))
                            .rounded_full()
                            .bg(alpha(c.primary, 0.2))
                            .child(
                                div()
                                    .h_full()
                                    .w(relative(done as f32 / LESSONS.len() as f32))
                                    .rounded_full()
                                    .bg(c.primary),
                            ),
                    ),
            )
    }

    /// The check result above the preview: green once the learner can move on.
    fn status_bar(&self, lesson: &lessons::Lesson, cx: &mut Context<Self>) -> impl IntoElement {
        let c = colors();
        let tone = self.status_tone();
        let (glyph, label, detail) = match self.refresh {
            Refresh::Checking => (None, "Checking…", None),
            Refresh::Building => (None, "Rebuilding…", None),
            Refresh::Failed => (
                Some(icons::CIRCLE_ALERT),
                "Refresh failed",
                Some(div().child("See terminal diagnostics.")),
            ),
            Refresh::Current => match self.status {
                Status::Passed => (
                    Some(icons::CIRCLE_CHECK),
                    "Passed",
                    Some(
                        div()
                            .truncate()
                            .child("Press n in the terminal to continue."),
                    ),
                ),
                Status::Failed => (
                    Some(icons::CIRCLE_X),
                    "Not passing yet",
                    Some(
                        div()
                            .font_family(MONO)
                            .text_xs()
                            .overflow_hidden()
                            .whitespace_nowrap()
                            .text_ellipsis_start()
                            .child(lesson.file),
                    ),
                ),
                Status::Error => (
                    Some(icons::CIRCLE_ALERT),
                    "Needs attention",
                    Some(div().truncate().child("See terminal diagnostics.")),
                ),
                Status::Unchecked => (None, "Preview", None),
            },
        };
        div()
            .debug_selector(|| "lesson-status".into())
            .flex()
            .flex_shrink_0()
            .items_center()
            .gap_2()
            .pl_4()
            .pr_3()
            .py_3()
            .rounded_t_xl()
            .border_b_1()
            .border_color(tone.map_or(c.border, |tone| alpha(tone, 0.25)))
            .when_some(tone, |el, tone| el.bg(alpha(tone, 0.08)))
            .children(glyph.map(|glyph| icon(glyph).text_color(tone.unwrap_or(c.foreground))))
            .child(
                div()
                    .flex_shrink_0()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(tone.unwrap_or(c.foreground))
                    .child(label),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .text_color(c.muted_foreground)
                    .children(detail),
            )
            .child(
                button_base("reset-preview", Variant::Outline)
                    .debug_selector(|| "reset-preview".into())
                    .aria_label("Reset preview")
                    .h_8()
                    .px_3()
                    .gap_1p5()
                    .child(icon(icons::ROTATE_CCW).text_color(c.foreground))
                    .child("Reset")
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.advanced = exercises::advanced_preview(this.index, window, cx);
                        this.count = 0;
                        this.toggle = cx.new(|_| exercises::views::entity::TogglePanel::new());
                        cx.notify();
                    })),
            )
    }

    fn preview(&self, lesson: &lessons::Lesson, cx: &mut Context<Self>) -> impl IntoElement {
        let c = colors();
        let mut canvas = div()
            .debug_selector(|| "lesson-preview".into())
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .flex_1()
            .gap_5()
            .min_h(px(280.0))
            .p_8();
        match self.index {
            0 => {
                canvas = canvas.child(
                    div()
                        .debug_selector(|| "exercise-01".into())
                        .text_size(px(36.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(exercises::basics::greeting::welcome_text()),
                );
            }
            1 | 2 => {
                canvas = canvas.child(
                    div()
                        .debug_selector(|| "counter-panel".into())
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap_4()
                        .child(
                            div()
                                .text_size(px(48.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .line_height(relative(1.0))
                                .child(self.count.to_string()),
                        )
                        .child(
                            button("increment-button", "Increase count", true)
                                .debug_selector(|| "increment-button".into())
                                .on_click(cx.listener(|this, _, _, cx| {
                                    // Each lesson can be previewed independently of previous solutions.
                                    if this.index == 1 {
                                        exercises::basics::counter::increment(&mut this.count);
                                    } else {
                                        this.count = this.count.saturating_add(1);
                                    }
                                    cx.notify();
                                })),
                        ),
                );
                if self.index == 2 {
                    canvas = canvas.child(
                        badge(false)
                            .debug_selector(|| "milestone-panel".into())
                            .px_3()
                            .py_1()
                            .text_sm()
                            .child(exercises::basics::milestone::milestone_text(self.count)),
                    );
                }
            }
            3 => canvas = canvas.child(exercises::views::layout::progress_strip()),
            4 => canvas = canvas.child(self.toggle.clone()),
            5 => canvas = canvas.child(exercises::views::spacing::spaced_tiles()),
            _ => {
                if let Some(view) = &self.advanced {
                    canvas = canvas.child(view.clone());
                }
            }
        }
        let tone = self.status_tone();
        div()
            .flex_1()
            .flex()
            .flex_col()
            .rounded_xl()
            .border_1()
            .border_color(tone.map_or(c.border, |tone| alpha(tone, 0.4)))
            .bg(c.card)
            .shadow_sm()
            .child(self.status_bar(lesson, cx))
            .when(self.refresh != Refresh::Current, |el| {
                el.child(
                    div()
                        .debug_selector(|| "preview-outdated".into())
                        .px_4()
                        .py_2()
                        .text_xs()
                        .text_color(tone.unwrap_or(c.muted_foreground))
                        .child(self.refresh.label()),
                )
            })
            .child(canvas)
    }

    fn lesson(&self, lesson: &lessons::Lesson, cx: &mut Context<Self>) -> impl IntoElement {
        let c = colors();
        div()
            .w_full()
            .max_w(px(896.0))
            .flex_1()
            .flex()
            .flex_col()
            .gap_6()
            .px_8()
            .py_8()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1p5()
                    .child(
                        div()
                            .text_2xl()
                            .line_height(relative(1.25))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(lesson.title),
                    )
                    .child(
                        div()
                            .text_base()
                            .text_color(c.muted_foreground)
                            .child(lesson.objective),
                    ),
            )
            .child(self.preview(lesson, cx))
    }

    fn complete(&self) -> impl IntoElement {
        let c = colors();
        div()
            .flex_1()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_6()
            .p_8()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .size_12()
                    .rounded_lg()
                    .bg(c.muted)
                    .child(icon(icons::CIRCLE_CHECK).size_6().text_color(c.foreground)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::MEDIUM)
                            .child("Workshop complete"),
                    )
                    .child(
                        div()
                            .max_w(px(400.0))
                            .text_center()
                            .text_color(c.muted_foreground)
                            .child("You have practiced rendering, contexts, events, focus, and async work."),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .text_color(c.muted_foreground)
                    .child("Revisit any lesson with")
                    .child(code("./gpui-lings app 01").text_color(c.foreground)),
            )
    }
}

impl Render for Playground {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        theme::use_appearance(
            self.forced_appearance
                .unwrap_or_else(|| window.appearance()),
        );
        let c = colors();
        let content = div()
            .id("lesson-content")
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .flex()
            .flex_col()
            .items_center();
        let content = match LESSONS.get(self.index) {
            Some(lesson) => content.child(self.lesson(lesson, cx)),
            None => content.child(self.complete()),
        };
        div()
            .id("workshop")
            .tab_group()
            .flex()
            .flex_col()
            .size_full()
            .bg(c.background)
            .text_color(c.foreground)
            .font_family(".SystemUIFont")
            .text_sm()
            .line_height(relative(1.5))
            .child(self.header())
            .child(content)
    }
}

pub fn run() {
    let id = std::env::var("GPUI_LINGS_LESSON").unwrap_or_else(|_| "01".into());
    let index = if id == "complete" {
        LESSONS.len()
    } else {
        lessons::progress_index(&id)
    };
    let status = Status::parse(&std::env::var("GPUI_LINGS_STATUS").unwrap_or_default());
    let completed = std::env::var("GPUI_LINGS_COMPLETED")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0)
        .min(LESSONS.len());
    gpui_kit::application().run(move |cx| {
        gpui_kit::init(cx);
        preview::watch_guide(cx);
        cx.on_window_closed(|cx, _| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
        let placement = std::env::var_os("GPUI_LINGS_WINDOW_BOUNDS")
            .and_then(|path| preview::restored_placement(&PathBuf::from(path), cx));
        let options = WindowOptions {
            display_id: placement.map(|(_, display)| display),
            window_bounds: Some(
                placement
                    .map(|(bounds, _)| bounds)
                    .unwrap_or_else(|| WindowBounds::centered(size(px(960.0), px(760.0)), cx)),
            ),
            window_min_size: Some(size(px(640.0), px(560.0))),
            titlebar: Some(TitlebarOptions {
                title: Some("GPUI Lings".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let requested_bounds = options.window_bounds.unwrap();
        gpui_kit::open_window(options, cx, |window, cx| {
            cx.new(|cx| {
                let mut playground = Playground::new(index, completed, status, window, cx);
                playground._bounds = preview::track_bounds(requested_bounds, window, cx);
                playground
            })
        })
        .expect("could not open the GPUI playground window");
        cx.activate(true);
        // GPUI's open_window synchronously draws the root before returning.
        // Frame callbacks can be suspended when the window is occluded.
        if let Some(path) = std::env::var_os("GPUI_LINGS_READY") {
            let _ = std::fs::write(path, "ready");
        }
    });
}

#[cfg(test)]
mod tests {
    use super::{Playground, Status};
    use crate::theme::colors;
    use gpui::{Context, Modifiers, TestAppContext, Window};

    fn view(index: usize, window: &mut Window, cx: &mut Context<Playground>) -> Playground {
        Playground::new(index, 0, Status::Unchecked, window, cx)
    }

    #[gpui::test]
    fn progress_can_drop_below_the_open_lesson_after_a_reset(cx: &mut TestAppContext) {
        use super::{PreviewState, Refresh};
        let (view, cx) =
            cx.add_window_view(|window, cx| Playground::new(6, 7, Status::Passed, window, cx));
        view.update(cx, |view, _| {
            assert_eq!(view.completed_count(), 7);
            assert!(view.apply_preview_state(&PreviewState {
                index: 0,
                status: "failed".into(),
                completed: 0,
                refresh: Refresh::Building,
            }));
            assert_eq!(
                view.completed_count(),
                7,
                "pending builds hide the new result"
            );
            assert_eq!(view.status_tone(), Some(crate::theme::colors().loading));
            assert_eq!(
                view.index, 6,
                "old preview stays open until the replacement is ready"
            );
            view.apply_preview_state(&PreviewState {
                index: 6,
                status: "failed".into(),
                completed: 0,
                refresh: Refresh::Current,
            });
            assert_eq!(view.completed_count(), 0);
            assert_eq!(view.status, Status::Failed);
        });
    }

    #[gpui::test]
    fn live_check_updates_progress_without_navigation(cx: &mut TestAppContext) {
        use super::{PreviewState, Refresh};
        let (view, cx) =
            cx.add_window_view(|window, cx| Playground::new(3, 3, Status::Failed, window, cx));
        view.update(cx, |view, cx| {
            assert_eq!(view.completed_count(), 3);
            assert!(view.apply_preview_state(&PreviewState {
                index: 3,
                status: "passed".into(),
                completed: 4,
                refresh: Refresh::Building,
            }));
            assert_eq!(
                view.completed_count(),
                3,
                "progress is revealed with the replacement"
            );
            assert_eq!(view.status, Status::Failed);
            assert_eq!(view.status_tone(), Some(crate::theme::colors().loading));
            assert_eq!(view.index, 3, "passing must not navigate");
            cx.notify();
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(cx.debug_bounds("preview-outdated").is_some());
        view.update(cx, |view, _| {
            view.apply_preview_state(&PreviewState {
                index: 4,
                status: "failed".into(),
                completed: 4,
                refresh: Refresh::Checking,
            });
            assert_eq!(
                view.status,
                Status::Failed,
                "pending checks must not reveal the new result"
            );
            view.apply_preview_state(&PreviewState {
                index: 3,
                status: "failed".into(),
                completed: 3,
                refresh: Refresh::Failed,
            });
            assert_eq!(
                view.completed_count(),
                3,
                "a regressed solution is no longer green"
            );
        });
        let (next, cx) =
            cx.add_window_view(|window, cx| Playground::new(4, 4, Status::Failed, window, cx));
        cx.update(|_, cx| {
            assert_eq!(
                next.read(cx).completed_count(),
                4,
                "next must not double count"
            )
        });
    }

    #[gpui::test]
    fn greeting_stage_hides_future_exercises(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|window, cx| view(0, window, cx));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(cx.debug_bounds("exercise-01").is_some());
        for selector in [
            "counter-panel",
            "milestone-panel",
            "step-one",
            "toggle-button",
            "spacing-tile-0",
        ] {
            assert!(cx.debug_bounds(selector).is_none(), "unexpected {selector}");
        }
    }

    #[gpui::test]
    fn layout_stage_shows_only_layout_work(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|window, cx| view(3, window, cx));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(cx.debug_bounds("step-one").is_some());
        assert!(cx.debug_bounds("exercise-01").is_none());
        assert!(cx.debug_bounds("counter-panel").is_none());
        assert!(cx.debug_bounds("toggle-button").is_none());
    }

    #[gpui::test]
    fn milestone_preview_does_not_depend_on_counter_solution(cx: &mut TestAppContext) {
        let (view, cx) = cx.add_window_view(|window, cx| view(2, window, cx));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let button = cx.debug_bounds("increment-button").unwrap();
        for _ in 0..3 {
            cx.simulate_click(button.center(), Modifiers::default());
        }
        cx.update(|_, cx| assert_eq!(view.read(cx).count, 3));
        let reset = cx.debug_bounds("reset-preview").unwrap();
        cx.simulate_click(reset.center(), Modifiers::default());
        cx.update(|_, cx| assert_eq!(view.read(cx).count, 0));
    }

    #[gpui::test]
    fn minimum_window_fits_progress_status_and_preview(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|window, cx| view(5, window, cx));
        cx.simulate_resize(gpui::size(gpui::px(640.0), gpui::px(560.0)));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let progress = cx.debug_bounds("lesson-progress").unwrap();
        let status = cx.debug_bounds("lesson-status").unwrap();
        let preview = cx.debug_bounds("lesson-preview").unwrap();
        assert!(
            progress.right() <= gpui::px(640.0),
            "progress must fit the header"
        );
        assert!(
            status.right() <= gpui::px(640.0),
            "status must fit the window"
        );
        assert!(
            preview.right() <= gpui::px(640.0),
            "preview must fit the window"
        );
        assert!(
            preview.bottom() <= gpui::px(560.0),
            "preview must fit without scrolling"
        );
    }

    #[gpui::test]
    fn advanced_previews_open_visible_and_reset_independently(cx: &mut TestAppContext) {
        let selectors = [
            "notify-enable",
            "listener-record",
            "update-child",
            "observe-increment",
            "event-send",
            "action-focus",
            "focus-pad-button",
            "weak-inspect",
            "task-load",
            "responsive-card-0",
            "scroll-viewport",
            "state-toggle",
            "drag-track",
            "inspector-list",
            "regions-open",
            "route-focus-child",
            "deferred-queue",
            "menu-launcher",
            "background-start",
            "retry-load",
            "stale-slow",
            "search-input",
            "startup-open",
            "shared-toggle",
            "persist-toggle",
            "windows-open",
            "appearance-switch",
            "alerts-control",
            "behavior-load",
            "large-viewport",
            "setting-first",
            "workspace-load",
        ];
        for (offset, selector) in selectors.iter().enumerate() {
            let index = offset + 6;
            let (view, cx) = cx.add_window_view(|window, cx| view(index, window, cx));
            cx.simulate_resize(gpui::size(gpui::px(640.0), gpui::px(560.0)));
            cx.update(|window, cx| window.draw(cx).clear(cx));
            assert!(
                cx.debug_bounds(selector).is_some(),
                "missing preview for {selector}"
            );
            for other in selectors.iter().filter(|other| *other != selector) {
                assert!(
                    cx.debug_bounds(other).is_none(),
                    "unexpected preview for {other}"
                );
            }
            let previous = cx.update(|_, cx| view.read(cx).advanced.as_ref().unwrap().entity_id());
            let reset = cx.debug_bounds("reset-preview").unwrap();
            assert!(reset.bottom() <= gpui::px(560.0), "reset must be reachable");
            cx.simulate_click(reset.center(), Modifiers::default());
            cx.update(|window, cx| {
                assert_ne!(
                    view.read(cx).advanced.as_ref().unwrap().entity_id(),
                    previous,
                    "reset must create fresh state for lesson {}",
                    index + 1
                );
                window.draw(cx).clear(cx);
            });
            assert!(cx.debug_bounds(selector).is_some());
        }
    }

    #[test]
    fn status_is_green_to_move_on_and_red_while_failing() {
        assert_eq!(Status::parse("passed"), Status::Passed);
        assert_eq!(Status::parse("failed"), Status::Failed);
        assert_eq!(Status::parse("error"), Status::Error);
        assert_eq!(Status::parse(""), Status::Unchecked);
        assert_eq!(Status::Passed.tone(), Some(colors().success));
        assert_eq!(Status::Failed.tone(), Some(colors().destructive));
        assert_eq!(Status::Error.tone(), Some(colors().destructive));
        assert_eq!(Status::Unchecked.tone(), None);
    }

    #[gpui::test]
    fn every_status_renders_beside_the_preview(cx: &mut TestAppContext) {
        for status in [
            Status::Unchecked,
            Status::Passed,
            Status::Failed,
            Status::Error,
        ] {
            let (view, cx) =
                cx.add_window_view(|window, cx| Playground::new(3, 0, status, window, cx));
            cx.update(|window, cx| window.draw(cx).clear(cx));
            let bar = cx.debug_bounds("lesson-status").unwrap();
            let preview = cx.debug_bounds("lesson-preview").unwrap();
            assert!(
                bar.bottom() <= preview.top(),
                "{status:?} must sit above the preview"
            );
            for refresh in [super::Refresh::Checking, super::Refresh::Building] {
                view.update(cx, |view, cx| {
                    view.refresh = refresh;
                    assert_eq!(view.status_tone(), Some(colors().loading));
                    cx.notify();
                });
                cx.update(|window, cx| window.draw(cx).clear(cx));
                assert!(cx.debug_bounds("preview-outdated").is_some());
            }
        }
    }
}
