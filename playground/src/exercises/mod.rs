//! Small, ordered changes to the same native app.

pub mod app;
pub mod basics;
pub mod contexts;
pub mod interaction;
pub mod lifetimes;
pub mod views;

use gpui_kit::{AnyView, App, AppContext as _};

pub(crate) fn advanced_preview(
    index: usize,
    window: &mut gpui_kit::Window,
    cx: &mut App,
) -> Option<AnyView> {
    Some(match index {
        6 => cx.new(|_| contexts::notify::NotifyPanel::default()).into(),
        7 => cx
            .new(|_| contexts::listener::ListenerPanel::default())
            .into(),
        8 => cx.new(contexts::update::UpdatePanel::new).into(),
        9 => cx.new(contexts::observe::ObservePanel::new).into(),
        10 => cx.new(contexts::events::EventsPanel::new).into(),
        11 => cx.new(interaction::actions::ActionsPanel::new).into(),
        12 => cx.new(interaction::focus::FocusPanel::new).into(),
        13 => cx.new(lifetimes::weak::WeakPanel::new).into(),
        14 => cx.new(|_| lifetimes::tasks::TasksPanel::default()).into(),
        15 => cx.new(|_| views::responsive::ResponsivePanel).into(),
        16 => cx
            .new(|_| views::scrolling::ScrollingPanel::default())
            .into(),
        17 => cx.new(views::states::StatesPanel::new).into(),
        18 => cx.new(|_| views::drag::DragPanel::default()).into(),
        19 => cx.new(views::inspector::InspectorPanel::new).into(),
        20 => cx.new(interaction::regions::RegionsPanel::new).into(),
        21 => cx
            .new(interaction::propagation::PropagationPanel::new)
            .into(),
        22 => cx
            .new(|_| contexts::deferred::DeferredPanel::default())
            .into(),
        23 => cx.new(interaction::menu::MenuPanel::new).into(),
        24 => cx
            .new(|_| lifetimes::background::BackgroundPanel::default())
            .into(),
        25 => cx.new(|_| lifetimes::retry::RetryPanel::default()).into(),
        26 => cx.new(|_| lifetimes::stale::StalePanel::default()).into(),
        27 => cx
            .new(|cx| lifetimes::search::SearchPanel::new(window, cx))
            .into(),
        28 => cx.new(|_| app::startup::StartupPanel::default()).into(),
        29 => cx.new(app::shared::SharedPanel::new).into(),
        30 => cx
            .new(|_| app::persistence::PersistencePanel::default())
            .into(),
        31 => cx.new(app::windows::WindowsPanel::new).into(),
        32 => cx
            .new(|cx| app::appearance::AppearancePanel::new(window, cx))
            .into(),
        33 => cx
            .new(|_| app::accessibility::AccessibilityPanel::default())
            .into(),
        34 => cx
            .new(|_| app::behavior_test::BehaviorTestPanel::default())
            .into(),
        35 => cx
            .new(|_| app::large_list::LargeListPanel::default())
            .into(),
        36 => cx.new(app::reusable::ReusablePanel::new).into(),
        37 => cx.new(app::capstone::CapstonePanel::new).into(),
        _ => return None,
    })
}
