// This is a quiz for the following lessons:
// - 10–11 Observe & subscribe
// - 17–19 Lifetimes & async
//
// A small inbox. The `Inbox` model counts unread messages and emits an
// `Arrived` event for each new one. A badge observes the model to show the
// unread count, and a log subscribes to its events to list each subject.
// Receive adds a message right away. Fetch adds one after a second, unless you
// press Cancel first.
//
// Nothing marks the broken lines. Testers reported:
//
// - The unread badge never appears.
// - Once it does, the badge stays at 0 however many messages arrive.
// - The log never lists a message.
// - Cancel doesn't stop a fetch: its message still arrives a second later.
//
// Find and fix all four. The check stops at the first symptom it finds.
// Reproduce it in the preview, trace it back to the code, and fix it before
// moving on.

use crate::theme::{badge, button, colors};
use gpui_kit::{
    Context, Entity, EventEmitter, IntoElement, Render, Subscription, Task, WeakEntity, Window,
    div, prelude::*, px,
};
use std::time::Duration;

// The model. It isn't a view: it has no `render`.
#[derive(Default)]
pub struct Inbox {
    unread: usize,
}

// Sent once for every new message, with its subject.
pub struct Arrived(pub &'static str);
impl EventEmitter<Arrived> for Inbox {}

impl Inbox {
    fn receive(&mut self, subject: &'static str, cx: &mut Context<Self>) {
        self.unread += 1;
        cx.emit(Arrived(subject));
        cx.notify();
    }
}

// Shows how many messages are unread.
pub struct Badge {
    unread: usize,
    _observer: Subscription,
}

impl Badge {
    fn new(inbox: &Entity<Inbox>, cx: &mut Context<Self>) -> Self {
        let unread = inbox.read(cx).unread;
        let observer = cx.observe(inbox, move |this, _, cx| {
            this.unread = unread;
            cx.notify();
        });
        Self {
            unread,
            _observer: observer,
        }
    }
}

impl Render for Badge {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let unread = self.unread;
        badge(true)
            .debug_selector(move || format!("inbox-unread-{unread}"))
            .px_3()
            .py_1()
            .child(format!("{unread} unread"))
    }
}

// Lists the subject of every message that arrives.
pub struct Log {
    subjects: Vec<&'static str>,
    _subscriptions: Vec<Subscription>,
}

impl Log {
    fn new(inbox: &Entity<Inbox>, cx: &mut Context<Self>) -> Self {
        let _ = cx.subscribe(inbox, |this, _, arrived: &Arrived, cx| {
            this.subjects.push(arrived.0);
            cx.notify();
        });
        Self {
            subjects: Vec::new(),
            _subscriptions: Vec::new(),
        }
    }
}

impl Render for Log {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let count = self.subjects.len();
        div()
            .debug_selector(move || format!("inbox-log-{count}"))
            .w(px(240.0))
            .min_h(px(64.0))
            .p_3()
            .rounded_lg()
            .border_1()
            .border_color(colors().border)
            .text_sm()
            .when(count == 0, |log| log.child("No messages yet"))
            .children(self.subjects.iter().map(|subject| div().child(*subject)))
    }
}

pub struct InboxPanel {
    inbox: Entity<Inbox>,
    badge: WeakEntity<Badge>,
    log: Entity<Log>,
    // The fetch in flight, if any.
    fetch: Option<Task<()>>,
    status: &'static str,
}

impl InboxPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let inbox = cx.new(|_| Inbox::default());
        let badge = cx.new(|cx| Badge::new(&inbox, cx));
        let log = cx.new(|cx| Log::new(&inbox, cx));
        Self {
            inbox,
            badge: badge.downgrade(),
            log,
            fetch: None,
            status: "Idle",
        }
    }

    fn fetch(&mut self, cx: &mut Context<Self>) {
        self.status = "Fetching…";
        cx.notify();
        cx.spawn(async move |this, cx| {
            cx.background_executor().timer(Duration::from_secs(1)).await;
            let _ = this.update(cx, |this, cx| {
                this.inbox
                    .update(cx, |inbox, cx| inbox.receive("Fetched report", cx));
                this.status = "Idle";
                cx.notify();
            });
        })
        .detach();
    }

    fn cancel(&mut self, cx: &mut Context<Self>) {
        self.fetch = None;
        self.status = "Cancelled";
        cx.notify();
    }
}

impl Render for InboxPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        button("inbox-receive", "Receive", true)
                            .debug_selector(|| "inbox-receive".into())
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.inbox
                                    .update(cx, |inbox, cx| inbox.receive("Hello", cx));
                            })),
                    )
                    .child(
                        button("inbox-fetch", "Fetch", false)
                            .debug_selector(|| "inbox-fetch".into())
                            .on_click(cx.listener(|this, _, _, cx| this.fetch(cx))),
                    )
                    .child(
                        button("inbox-cancel", "Cancel", false)
                            .debug_selector(|| "inbox-cancel".into())
                            .on_click(cx.listener(|this, _, _, cx| this.cancel(cx))),
                    ),
            )
            .children(self.badge.upgrade())
            .child(self.log.clone())
            .child(
                div()
                    .debug_selector(|| format!("inbox-status-{}", self.status))
                    .text_color(colors().muted_foreground)
                    .child(self.status),
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
    fn exercise_20(cx: &mut TestAppContext) {
        let (_, cx) = cx.add_window_view(|_, cx| InboxPanel::new(cx));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(
            cx.debug_bounds("inbox-unread-0").is_some(),
            "the unread badge never appears"
        );

        let receive = cx.debug_bounds("inbox-receive").unwrap();
        cx.simulate_click(receive.center(), Modifiers::default());
        cx.simulate_click(receive.center(), Modifiers::default());
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(
            cx.debug_bounds("inbox-unread-2").is_some(),
            "the badge stays at 0 however many messages arrive"
        );
        assert!(
            cx.debug_bounds("inbox-log-2").is_some(),
            "the log never lists a message"
        );

        // A fetch that runs to the end delivers its message after a second.
        let fetch = cx.debug_bounds("inbox-fetch").unwrap();
        cx.simulate_click(fetch.center(), Modifiers::default());
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(
            cx.debug_bounds("inbox-unread-3").is_some(),
            "Fetch should add a message after a second"
        );

        // A cancelled one must not.
        let cancel = cx.debug_bounds("inbox-cancel").unwrap();
        cx.simulate_click(fetch.center(), Modifiers::default());
        cx.run_until_parked();
        cx.simulate_click(cancel.center(), Modifiers::default());
        cx.executor().advance_clock(Duration::from_secs(2));
        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert!(
            cx.debug_bounds("inbox-unread-3").is_some()
                && cx.debug_bounds("inbox-status-Cancelled").is_some(),
            "Cancel doesn't stop a fetch: its message still arrives a second later"
        );
    }
}
