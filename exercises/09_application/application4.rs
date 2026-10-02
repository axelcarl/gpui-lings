//! 32 — Open a second window
//!
//! A detail window can share an Entity with the main view. Both windows
//! observe that model, so a change from either redraws the other. GPUI Kit's
//! open_window creates the second window. Observe its closure and clear the
//! stored handle so Open can work again.
//!
//! Goal: open detail, update the count from either window, close detail, and
//! open it again. Retain the on_window_closed Subscription until it closes.
//!
//! Example — Sharing one entity between views:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! let detail_model = self.model.clone();
//! let (_, detail) = gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| {
//!     cx.new(|cx| DetailView::new(detail_model, cx))
//! })?;
//! // Both views read and update the same model; detail owns its own view state.
//! ```

use crate::theme::button;
use gpui_kit::{
    AnyWindowHandle, Context, Entity, IntoElement, Render, Subscription, Window, WindowOptions,
    div, prelude::*,
};

#[derive(Default)]
pub struct CountModel {
    value: u32,
}

pub struct DetailPanel {
    model: Entity<CountModel>,
    _observer: Subscription,
}
impl DetailPanel {
    fn new(model: Entity<CountModel>, cx: &mut Context<Self>) -> Self {
        let observer = cx.observe(&model, |_, _, cx| cx.notify());
        Self {
            model,
            _observer: observer,
        }
    }
}
impl Render for DetailPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.model.read(cx).value;
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                button("detail-increment", "Increment in detail", true)
                    .debug_selector(|| "detail-increment".into())
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.model.update(cx, |model, cx| {
                            model.value += 1;
                            cx.notify();
                        });
                    })),
            )
            .child(
                button("detail-close", "Close detail", false)
                    .debug_selector(|| "detail-close".into())
                    .on_click(|_, window, _| window.remove_window()),
            )
            .child(
                div()
                    .debug_selector(move || format!("detail-count-{count}"))
                    .child(format!("Count: {count}")),
            )
    }
}

pub struct WindowsPanel {
    model: Entity<CountModel>,
    detail: Option<AnyWindowHandle>,
    _model_observer: Subscription,
    _close_subscription: Option<Subscription>,
}
impl WindowsPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let model = cx.new(|_| CountModel::default());
        let observer = cx.observe(&model, |_, _, cx| cx.notify());
        Self {
            model,
            detail: None,
            _model_observer: observer,
            _close_subscription: None,
        }
    }

    fn open(&mut self, cx: &mut Context<Self>) {
        if self.detail.is_some() {
            return;
        }
        let model = self.model.clone();
        let (handle, _) = gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| {
            cx.new(|cx| DetailPanel::new(model, cx))
        })
        .expect("could not open detail window");
        self.detail = Some(handle);
        let weak = cx.weak_entity();
        let subscription = cx.on_window_closed(move |cx, closed| {
            if closed == handle.window_id() {
                let _ = weak.update(cx, |this, cx| {
                    this.detail = None;
                    this._close_subscription.take();
                    cx.notify();
                });
            }
        });
        let _ = subscription;
        cx.notify();
    }
}
impl Render for WindowsPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.model.read(cx).value;
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                button("windows-open", "Open detail", true)
                    .debug_selector(|| "windows-open".into())
                    .on_click(cx.listener(|this, _, _, cx| this.open(cx))),
            )
            .child(
                button("windows-increment", "Increment in main", false)
                    .debug_selector(|| "windows-increment".into())
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.model.update(cx, |model, cx| {
                            model.value += 1;
                            cx.notify();
                        });
                    })),
            )
            .child(
                div()
                    .debug_selector(move || format!("windows-count-{count}"))
                    .child(format!("Count: {count}")),
            )
            .child(
                div()
                    .debug_selector(|| {
                        if self.detail.is_some() {
                            "windows-detail-open".into()
                        } else {
                            "windows-detail-closed".into()
                        }
                    })
                    .child(if self.detail.is_some() {
                        "Detail open"
                    } else {
                        "Detail closed"
                    }),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext, VisualTestContext};

    #[gpui::test]
    fn exercise_32(cx: &mut TestAppContext) {
        cx.update(gpui_kit::init);
        let (panel, cx) = cx.add_window_view(|_, cx| WindowsPanel::new(cx));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let starting_windows = cx.update(|_, cx| cx.windows().len());
        let open = cx.debug_bounds("windows-open").unwrap();
        let main_increment = cx.debug_bounds("windows-increment").unwrap();
        cx.simulate_click(open.center(), Modifiers::default());
        let handle = cx.update(|_, cx| {
            assert_eq!(cx.windows().len(), starting_windows + 1);
            panel.read(cx).detail.unwrap()
        });

        let mut detail = VisualTestContext::from_window(handle, &cx.cx);
        detail.update(|window, cx| window.draw(cx).clear(cx));
        let increment = detail.debug_bounds("detail-increment").unwrap();
        detail.simulate_click(increment.center(), Modifiers::default());
        cx.update(|window, cx| {
            assert_eq!(panel.read(cx).model.read(cx).value, 1);
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("windows-count-1").is_some());

        cx.simulate_click(main_increment.center(), Modifiers::default());
        detail.update(|window, cx| window.draw(cx).clear(cx));
        assert!(detail.debug_bounds("detail-count-2").is_some());

        let close = detail.debug_bounds("detail-close").unwrap();
        detail.simulate_click(close.center(), Modifiers::default());
        cx.update(|window, cx| {
            assert_eq!(cx.windows().len(), starting_windows);
            assert!(panel.read(cx).detail.is_none(), "clear the closed handle");
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("windows-detail-closed").is_some());
        cx.simulate_click(open.center(), Modifiers::default());
        cx.update(|_, cx| assert_eq!(cx.windows().len(), starting_windows + 1));
    }
}
