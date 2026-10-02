//! 36 — Render a large collection efficiently
//!
//! GPUI Base's virtual list accepts the whole collection size, but asks its
//! callback to build only a visible range. The starter builds every row first
//! and then throws most of them away. That still allocates 1,000 elements on
//! every frame. Keep the stable row IDs and selection behavior while building
//! only the requested range.
//!
//! Goal: select a row, scroll to the last row, and keep the selection while
//! constructing fewer than 40 rows for each frame of this small viewport.
//!
//! Example — Constructing only the requested range:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! let visible = 20..30;
//! let labels: Vec<_> = visible.map(|index| format!("Row {index}")).collect();
//! // A virtual-list callback receives a range like this for the current viewport.
//! ```

use gpui_kit::base::{VirtualListScrollHandle, v_virtual_list};
use gpui_kit::{Context, IntoElement, Render, Window, div, prelude::*, px, size};
use std::{cell::Cell, ops::Range, rc::Rc};

const ROWS: usize = 1_000;

pub struct LargeListPanel {
    sizes: Rc<Vec<gpui_kit::Size<gpui_kit::Pixels>>>,
    scroll: VirtualListScrollHandle,
    selected: Option<usize>,
    built: Cell<usize>,
}

impl Default for LargeListPanel {
    fn default() -> Self {
        Self {
            sizes: Rc::new(vec![size(px(300.0), px(32.0)); ROWS]),
            scroll: VirtualListScrollHandle::new(),
            selected: None,
            built: Cell::new(0),
        }
    }
}

impl LargeListPanel {
    fn row(&self, ix: usize, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        self.built.set(self.built.get() + 1);
        let selected = self.selected == Some(ix);
        div()
            .id(ix)
            .debug_selector(move || format!("large-row-{ix}"))
            .h(px(32.0))
            .w_full()
            .px_2()
            .cursor_pointer()
            .when(selected, |row| row.bg(crate::theme::colors().muted))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.selected = Some(ix);
                cx.notify();
            }))
            .child(format!(
                "Record {ix}{}",
                if selected { " · selected" } else { "" }
            ))
    }

    // TODO: Build only the rows in the visible range, not all ROWS first.
    fn rows(
        &mut self,
        range: Range<usize>,
        cx: &mut Context<Self>,
    ) -> Vec<impl IntoElement + use<>> {
        let all = (0..ROWS).map(|ix| self.row(ix, cx)).collect::<Vec<_>>();
        all.into_iter()
            .skip(range.start)
            .take(range.len())
            .collect()
    }
}

impl Render for LargeListPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.built.set(0);
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(div().child(format!("{} records", ROWS)))
            .child(
                div().debug_selector(|| "large-viewport".into()).child(
                    v_virtual_list(
                        cx.entity(),
                        "large-records",
                        self.sizes.clone(),
                        |this, range, _, cx| this.rows(range, cx),
                    )
                    .track_scroll(&self.scroll)
                    .h(px(220.0))
                    .w(px(300.0)),
                ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, ScrollStrategy, TestAppContext};

    #[gpui::test]
    fn exercise_36(cx: &mut TestAppContext) {
        let (panel, window) = cx.add_window_view(|_, _| LargeListPanel::default());
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert!(window.debug_bounds("large-row-0").is_some());
        window.update(|_, cx| {
            assert!(panel.read(cx).built.get() < 40, "build only visible rows");
        });

        let first = window.debug_bounds("large-row-0").unwrap();
        window.simulate_click(first.center(), Modifiers::default());
        window.update(|window, cx| {
            assert_eq!(panel.read(cx).selected, Some(0));
            panel
                .read(cx)
                .scroll
                .scroll_to_item(ROWS - 1, ScrollStrategy::Top);
            window.draw(cx).clear(cx);
        });
        assert!(window.debug_bounds("large-row-999").is_some());
        window.update(|_, cx| {
            assert!(panel.read(cx).built.get() < 40);
            assert_eq!(panel.read(cx).selected, Some(0));
        });
    }
}
