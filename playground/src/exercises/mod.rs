//! Every exercise file, compiled as a module named after the file, and the
//! preview each lesson shows. `shared/lessons.rs` holds the course order.

use gpui_kit::{
    AnyElement, AnyView, App, AppContext as _, Context, FontWeight, IntoElement, Render, Window,
    div, prelude::*, px,
};

#[path = "../../../exercises/09_application/application1.rs"]
pub mod application1;
#[path = "../../../exercises/09_application/application2.rs"]
pub mod application2;
#[path = "../../../exercises/09_application/application3.rs"]
pub mod application3;
#[path = "../../../exercises/09_application/application4.rs"]
pub mod application4;
#[path = "../../../exercises/09_application/application5.rs"]
pub mod application5;
#[path = "../../../exercises/08_async/async1.rs"]
pub mod async1;
#[path = "../../../exercises/08_async/async2.rs"]
pub mod async2;
#[path = "../../../exercises/08_async/async3.rs"]
pub mod async3;
#[path = "../../../exercises/01_basics/basics1.rs"]
pub mod basics1;
#[path = "../../../exercises/01_basics/basics2.rs"]
pub mod basics2;
#[path = "../../../exercises/01_basics/basics3.rs"]
pub mod basics3;
#[path = "../../../exercises/03_contexts/contexts1.rs"]
pub mod contexts1;
#[path = "../../../exercises/03_contexts/contexts2.rs"]
pub mod contexts2;
#[path = "../../../exercises/03_contexts/contexts3.rs"]
pub mod contexts3;
#[path = "../../../exercises/03_contexts/contexts4.rs"]
pub mod contexts4;
#[path = "../../../exercises/03_contexts/contexts5.rs"]
pub mod contexts5;
#[path = "../../../exercises/07_dispatch/dispatch1.rs"]
pub mod dispatch1;
#[path = "../../../exercises/07_dispatch/dispatch2.rs"]
pub mod dispatch2;
#[path = "../../../exercises/04_keyboard/keyboard1.rs"]
pub mod keyboard1;
#[path = "../../../exercises/04_keyboard/keyboard2.rs"]
pub mod keyboard2;
#[path = "../../../exercises/04_keyboard/keyboard3.rs"]
pub mod keyboard3;
#[path = "../../../exercises/04_keyboard/keyboard4.rs"]
pub mod keyboard4;
#[path = "../../../exercises/06_layout_states/layout_states1.rs"]
pub mod layout_states1;
#[path = "../../../exercises/06_layout_states/layout_states2.rs"]
pub mod layout_states2;
#[path = "../../../exercises/06_layout_states/layout_states3.rs"]
pub mod layout_states3;
#[path = "../../../exercises/06_layout_states/layout_states4.rs"]
pub mod layout_states4;
#[path = "../../../exercises/05_lifetimes/lifetimes1.rs"]
pub mod lifetimes1;
#[path = "../../../exercises/05_lifetimes/lifetimes2.rs"]
pub mod lifetimes2;
#[path = "../../../exercises/05_lifetimes/lifetimes3.rs"]
pub mod lifetimes3;
#[path = "../../../exercises/10_quality/quality1.rs"]
pub mod quality1;
#[path = "../../../exercises/10_quality/quality2.rs"]
pub mod quality2;
#[path = "../../../exercises/10_quality/quality3.rs"]
pub mod quality3;
#[path = "../../../exercises/10_quality/quality4.rs"]
pub mod quality4;
#[path = "../../../exercises/quizzes/quiz1.rs"]
pub mod quiz1;
#[path = "../../../exercises/quizzes/quiz2.rs"]
pub mod quiz2;
#[path = "../../../exercises/quizzes/quiz3.rs"]
pub mod quiz3;
#[path = "../../../exercises/quizzes/quiz4.rs"]
pub mod quiz4;
#[path = "../../../exercises/quizzes/quiz5.rs"]
pub mod quiz5;
#[path = "../../../exercises/quizzes/quiz6.rs"]
pub mod quiz6;
#[path = "../../../exercises/02_views/views1.rs"]
pub mod views1;
#[path = "../../../exercises/02_views/views2.rs"]
pub mod views2;
#[path = "../../../exercises/02_views/views3.rs"]
pub mod views3;

/// Shows an exercise that builds elements rather than a view of its own.
struct Elements(fn() -> AnyElement);

impl Render for Elements {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        (self.0)()
    }
}

/// Lesson 01's headline, from the text its exercise returns.
fn greeting() -> AnyElement {
    div()
        .debug_selector(|| "exercise-01".into())
        .text_size(px(36.0))
        .font_weight(FontWeight::SEMIBOLD)
        .child(basics1::welcome_text())
        .into_any_element()
}

/// A fresh preview for the lesson with this exercise name. Each lesson builds
/// its own state, so a preview never depends on solving an earlier lesson.
pub(crate) fn preview(name: &str, window: &mut Window, cx: &mut App) -> Option<AnyView> {
    Some(match name {
        "basics1" => cx.new(|_| Elements(greeting)).into(),
        "basics2" => cx.new(|_| basics2::Counter::default()).into(),
        "basics3" => cx.new(|_| basics3::Milestone::default()).into(),
        "views1" => cx
            .new(|_| Elements(|| views1::progress_strip().into_any_element()))
            .into(),
        "views2" => cx.new(|_| views2::TogglePanel::new()).into(),
        "views3" => cx
            .new(|_| Elements(|| views3::spaced_tiles().into_any_element()))
            .into(),
        "contexts1" => cx.new(|_| contexts1::NotifyPanel::default()).into(),
        "contexts2" => cx.new(|_| contexts2::ListenerPanel::default()).into(),
        "contexts3" => cx.new(contexts3::UpdatePanel::new).into(),
        "contexts4" => cx.new(contexts4::ObservePanel::new).into(),
        "contexts5" => cx.new(contexts5::EventsPanel::new).into(),
        "quiz1" => cx.new(|_| quiz1::Tally::default()).into(),
        "keyboard1" => cx.new(keyboard1::FocusPanel::new).into(),
        "keyboard2" => cx.new(keyboard2::StepperPanel::new).into(),
        "keyboard3" => cx.new(keyboard3::LikesPanel::new).into(),
        "keyboard4" => cx.new(keyboard4::ActionsPanel::new).into(),
        "lifetimes1" => cx.new(lifetimes1::WeakPanel::new).into(),
        "lifetimes2" => cx.new(|_| lifetimes2::DeferredPanel::default()).into(),
        "lifetimes3" => cx.new(|_| lifetimes3::TasksPanel::default()).into(),
        "quiz2" => cx.new(quiz2::InboxPanel::new).into(),
        "layout_states1" => cx.new(|_| layout_states1::ResponsivePanel).into(),
        "layout_states2" => cx.new(|_| layout_states2::ScrollingPanel::default()).into(),
        "layout_states3" => cx.new(layout_states3::StatesPanel::new).into(),
        "layout_states4" => cx.new(|_| layout_states4::DragPanel::default()).into(),
        "quiz3" => cx.new(quiz3::InspectorPanel::new).into(),
        "dispatch1" => cx.new(dispatch1::RegionsPanel::new).into(),
        "dispatch2" => cx.new(dispatch2::PropagationPanel::new).into(),
        "quiz4" => cx.new(quiz4::MenuPanel::new).into(),
        "async1" => cx.new(|_| async1::BackgroundPanel::default()).into(),
        "async2" => cx.new(|_| async2::RetryPanel::default()).into(),
        "async3" => cx.new(|_| async3::StalePanel::default()).into(),
        "quiz5" => cx.new(|cx| quiz5::SearchPanel::new(window, cx)).into(),
        "application1" => cx.new(|_| application1::StartupPanel::default()).into(),
        "application2" => cx.new(application2::SharedPanel::new).into(),
        "application3" => cx.new(|_| application3::PersistencePanel::default()).into(),
        "application4" => cx.new(application4::WindowsPanel::new).into(),
        "application5" => cx
            .new(|cx| application5::AppearancePanel::new(window, cx))
            .into(),
        "quality1" => cx.new(|_| quality1::AccessibilityPanel::default()).into(),
        "quality2" => cx.new(|_| quality2::BehaviorTestPanel::default()).into(),
        "quality3" => cx.new(|_| quality3::LargeListPanel::default()).into(),
        "quality4" => cx.new(quality4::ReusablePanel::new).into(),
        "quiz6" => cx.new(quiz6::CapstonePanel::new).into(),
        _ => return None,
    })
}
