// A list with a thousand rows doesn't need a thousand elements, only the few
// that fit on screen. GPUI Base's virtual list knows every row's size up front,
// so it can work out the scroll height and which rows are visible without
// building any of them. On each frame, it asks its callback for just that
// range of rows.
//
// This callback builds all 1,000 rows on every frame, then throws most of them
// away. The list looks fine, but it does far more work than it needs to.

use gpui_kit::base::{VirtualListScrollHandle, v_virtual_list};
use gpui_kit::{Context, IntoElement, Render, Window, div, prelude::*, px, size};
use std::{cell::Cell, ops::Range, rc::Rc};

const ROWS: usize = 1_000;

pub struct LargeListPanel {
    // One size per row; a vertical list uses only the heights. `Rc` lets the
    // list share this vector every frame without copying it.
    sizes: Rc<Vec<gpui_kit::Size<gpui_kit::Pixels>>>,
    // Lets code scroll the list. The check uses it to jump to the last row.
    scroll: VirtualListScrollHandle,
    // The selected row's index. It lives in the view, because row elements
    // are rebuilt every frame and rows scrolled out of view aren't built at all.
    selected: Option<usize>,
    // How many rows were built in the current frame, for the check to read.
    // A `Cell` lets `row` count through `&self`.
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
    // Builds the element for row `ix`, and counts it. `use<>` promises the
    // element borrows nothing from `self` or `cx`, so it can outlive the call.
    fn row(&self, ix: usize, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        self.built.set(self.built.get() + 1);
        let selected = self.selected == Some(ix);
        div()
            // A stable id from the row's index: the same row gets the same id
            // in every frame, wherever it has scrolled to.
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

    // The virtual list calls this with the indices of the rows on screen,
    // such as `0..7` at the top of the list, and shows the elements returned.
    fn rows(
        &mut self,
        range: Range<usize>,
        cx: &mut Context<Self>,
    ) -> Vec<impl IntoElement + use<>> {
        // TODO: This builds all 1,000 rows, then keeps only those in `range`.
        // Build rows for the indices in `range` alone: a Range is an iterator.
        let all = (0..ROWS).map(|ix| self.row(ix, cx)).collect::<Vec<_>>();
        all.into_iter()
            .skip(range.start)
            .take(range.len())
            .collect()
    }
}

impl Render for LargeListPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Start counting this frame's rows. The list calls `rows` later in the
        // same frame, once it knows which rows are visible.
        self.built.set(0);
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(div().child(format!("{} records", ROWS)))
            .child(
                div().debug_selector(|| "large-viewport".into()).child(
                    // A vertical virtual list: the view whose `rows` builds the
                    // elements, an id, every row's size, and the callback. The
                    // callback receives this view, the visible range, the
                    // window and the view's context.
                    v_virtual_list(
                        cx.entity(),
                        "large-records",
                        self.sizes.clone(),
                        |this, range, _, cx| this.rows(range, cx),
                    )
                    .track_scroll(&self.scroll)
                    // About seven 32px rows fit in this height.
                    .h(px(220.0))
                    .w(px(300.0)),
                ),
            )
    }
}

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, ScrollStrategy, TestAppContext};

    #[gpui::test]
    fn exercise_40(cx: &mut TestAppContext) {
        let (panel, window) = cx.add_window_view(|_, _| LargeListPanel::default());
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert!(window.debug_bounds("large-row-0").is_some());
        window.update(|_, cx| {
            assert!(panel.read(cx).built.get() < 40, "build only visible rows");
        });

        let first = window.debug_bounds("large-row-0").unwrap();
        window.simulate_click(first.center(), Modifiers::default());
        window.update(|window, cx| {
            assert_eq!(
                panel.read(cx).selected,
                Some(0),
                "clicking row 0 should select it"
            );
            panel
                .read(cx)
                .scroll
                .scroll_to_item(ROWS - 1, ScrollStrategy::Top);
            window.draw(cx).clear(cx);
        });
        assert!(
            window.debug_bounds("large-row-999").is_some(),
            "scrolling to the end should show the last row"
        );
        window.update(|_, cx| {
            assert!(
                panel.read(cx).built.get() < 40,
                "build only visible rows, also after scrolling"
            );
            assert_eq!(
                panel.read(cx).selected,
                Some(0),
                "the selection should survive scrolling"
            );
        });
    }
}
