// This is a quiz for the following lessons:
// - 09 Entities & updates
// - 17–19 Lifetimes & async
// - 29–31 Async data & failure paths
//
// A search box with results. The text field is GPUI Base's unstyled
// `InputState`, which handles typing and editing for each platform, and this
// view subscribes to its Change event. Every query starts some background
// work, and an older request can finish after a newer one. Type "bad" to see
// a recoverable error.
//
// Nothing marks the broken lines. Testers reported:
//
// - Typing in the field doesn't start a search.
// - Typing "ap" quickly shows its two matches, then Banana appears a second
//   later.
// - After an error, Retry does nothing.
//
// Find and fix all three. The check stops at the first symptom it finds.
// Reproduce it in the preview, noting which request finishes last, then trace
// it back to the code and fix it before moving on.

use crate::theme::{button, colors, focus_ring};
use gpui_kit::base::input::{Input, InputBase, InputEvent, InputState};
use gpui_kit::{
    Context, Entity, FocusHandle, Focusable, IntoElement, Render, Subscription, Task, Window, div,
    prelude::*, px,
};
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq)]
enum SearchState {
    Idle,
    Loading,
    Error(&'static str),
    Results(Vec<&'static str>),
}

pub struct SearchPanel {
    query: Entity<InputState>,
    results_focus: FocusHandle,
    query_text: String,
    generation: u64,
    bad_attempts: usize,
    state: SearchState,
    selected: usize,
    chosen: Option<&'static str>,
    tasks: Vec<Task<()>>,
    _query_subscription: Subscription,
}

impl SearchPanel {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let query = cx.new(|cx| InputState::new(window, cx).placeholder("Search items"));
        let subscription = cx.subscribe(&query, |this, input, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Change) {
                this.search(input.read(cx).value().to_string(), cx);
            }
        });
        Self {
            query: cx.new(|cx| InputState::new(window, cx).placeholder("Search items")),
            results_focus: cx.focus_handle(),
            query_text: String::new(),
            generation: 0,
            bad_attempts: 0,
            state: SearchState::Idle,
            selected: 0,
            chosen: None,
            tasks: Vec::new(),
            _query_subscription: subscription,
        }
    }

    fn search(&mut self, query: String, cx: &mut Context<Self>) {
        if query == self.query_text {
            return;
        }
        self.query_text = query.clone();
        self.generation += 1;
        let generation = self.generation;
        self.selected = 0;
        self.state = SearchState::Loading;
        cx.notify();

        let is_retry = if query == "bad" {
            self.bad_attempts += 1;
            self.bad_attempts > 1
        } else {
            false
        };
        let delay = if query == "a" { 2 } else { 1 };
        let timer = cx.background_executor().timer(Duration::from_secs(delay));
        let work = cx.background_executor().spawn(async move {
            timer.await;
            if query == "bad" {
                if is_retry {
                    Ok(vec!["Recovered report"])
                } else {
                    Err("Offline")
                }
            } else {
                let items = ["Apple", "Apricot", "Banana", "Berry"];
                Ok(items
                    .into_iter()
                    .filter(|item| item.to_ascii_lowercase().contains(&query))
                    .collect())
            }
        });
        self.tasks.push(cx.spawn(async move |this, cx| {
            let result = work.await;
            let _ = this.update(cx, |this, cx| {
                if generation > this.generation {
                    return;
                }
                this.state = match result {
                    Ok(items) => SearchState::Results(items),
                    Err(message) => SearchState::Error(message),
                };
                cx.notify();
            });
        }));
    }

    fn retry(&mut self, cx: &mut Context<Self>) {
        self.search(self.query_text.clone(), cx);
    }

    fn choose(&mut self, cx: &mut Context<Self>) {
        if let SearchState::Results(items) = &self.state {
            self.chosen = items.get(self.selected).copied();
            cx.notify();
        }
    }
}

impl Render for SearchPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = colors();
        let state_label = match &self.state {
            SearchState::Idle => "Idle",
            SearchState::Loading => "Loading",
            SearchState::Error(_) => "Error",
            SearchState::Results(_) => "Results",
        };
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_3()
            .child(
                button("search-focus", "Type query", true)
                    .debug_selector(|| "search-focus".into())
                    .on_click(cx.listener(|this, _, window, cx| {
                        let focus = this.query.read(cx).focus_handle(cx);
                        window.focus(&focus, cx);
                    })),
            )
            .child(
                InputBase::new("search-input")
                    .debug_selector(|| "search-input".into())
                    .w(px(240.0))
                    .h(px(36.0))
                    .border_1()
                    .border_color(c.border)
                    .rounded_lg()
                    .px_2()
                    .child(Input::new(&self.query)),
            )
            .child(
                button("search-retry", "Retry", false)
                    .debug_selector(|| "search-retry".into())
                    .on_click(cx.listener(|this, _, _, cx| this.retry(cx))),
            )
            .child(
                button("search-focus-results", "Focus results", false)
                    .debug_selector(|| "search-focus-results".into())
                    .on_click(cx.listener(|this, _, window, cx| {
                        window.focus(&this.results_focus, cx);
                    })),
            )
            .child(
                div()
                    .id("search-results")
                    .debug_selector(|| "search-results".into())
                    .track_focus(&self.results_focus)
                    .tab_index(0)
                    .bg(colors().card)
                    .focus_visible(focus_ring)
                    .on_key_down(cx.listener(|this, event: &gpui_kit::KeyDownEvent, _, cx| {
                        match event.keystroke.key.as_str() {
                            "down" => {
                                if let SearchState::Results(items) = &this.state {
                                    this.selected =
                                        (this.selected + 1).min(items.len().saturating_sub(1));
                                    cx.notify();
                                }
                            }
                            "up" => {
                                this.selected = this.selected.saturating_sub(1);
                                cx.notify();
                            }
                            "enter" => this.choose(cx),
                            _ => return,
                        }
                        cx.stop_propagation();
                    }))
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .debug_selector(move || format!("search-state-{state_label}"))
                            .child(state_label),
                    )
                    .when(matches!(&self.state, SearchState::Error(_)), |el| {
                        if let SearchState::Error(message) = &self.state {
                            el.child(*message)
                        } else {
                            el
                        }
                    })
                    .when(matches!(&self.state, SearchState::Results(_)), |el| {
                        if let SearchState::Results(items) = &self.state {
                            el.children(items.iter().copied().enumerate().map(|(index, item)| {
                                div()
                                    .debug_selector(move || format!("search-item-{item}"))
                                    .when(self.selected == index, |el| {
                                        el.bg(c.primary).text_color(c.primary_foreground)
                                    })
                                    .child(item)
                            }))
                        } else {
                            el
                        }
                    }),
            )
            .child(
                div()
                    .debug_selector(|| format!("search-chosen-{}", self.chosen.unwrap_or("None")))
                    .child(format!("Chosen: {}", self.chosen.unwrap_or("None"))),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_32(cx: &mut TestAppContext) {
        cx.update(gpui_kit::init);
        let (panel, cx) = cx.add_window_view(SearchPanel::new);
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let focus = cx.debug_bounds("search-focus").unwrap();
        cx.simulate_click(focus.center(), Modifiers::default());
        cx.simulate_keystrokes("a p");
        cx.update(|_, cx| {
            assert_eq!(
                panel.read(cx).query_text,
                "ap",
                "Typing in the field should start a search"
            );
            assert_eq!(panel.read(cx).state, SearchState::Loading);
        });
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        cx.update(|_, cx| {
            assert_eq!(
                panel.read(cx).state,
                SearchState::Results(vec!["Apple", "Apricot"])
            );
        });
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        cx.update(|window, cx| {
            assert_eq!(
                panel.read(cx).state,
                SearchState::Results(vec!["Apple", "Apricot"]),
                "\"ap\" should keep its two matches after the slower \"a\" search ends"
            );
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("search-item-Apricot").is_some());

        let results = cx.debug_bounds("search-focus-results").unwrap();
        cx.simulate_click(results.center(), Modifiers::default());
        cx.simulate_keystrokes("down enter");
        cx.update(|_, cx| assert_eq!(panel.read(cx).chosen, Some("Apricot")));

        panel.update(cx, |panel, cx| panel.search("bad".into(), cx));
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        cx.update(|window, cx| {
            assert_eq!(panel.read(cx).state, SearchState::Error("Offline"));
            window.draw(cx).clear(cx);
        });
        let retry = cx.debug_bounds("search-retry").unwrap();
        cx.simulate_click(retry.center(), Modifiers::default());
        cx.update(|_, cx| {
            assert_eq!(
                panel.read(cx).state,
                SearchState::Loading,
                "After an error, Retry should search again"
            )
        });
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.run_until_parked();
        cx.update(|_, cx| {
            assert_eq!(
                panel.read(cx).state,
                SearchState::Results(vec!["Recovered report"])
            )
        });
    }
}
