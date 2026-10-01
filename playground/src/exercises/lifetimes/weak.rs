//! 14 — Check a weak handle before using it
//!
//! Cloning Entity<T> keeps its state alive. WeakEntity<T> remembers its identity
//! without owning it. upgrade() returns Some(Entity<T>) while an owner exists,
//! and None once it is gone. This is useful for back-references and callbacks
//! that should not keep a closed view alive. Never unwrap a weak upgrade blindly.
//!
//! Goal: Inspect target must show Available, then Released after Release target.
//! Use the weak handle to obtain an optional strong handle inside inspect.
//! The temporary strong handle should live only for the inspection. Repeated
//! inspections after release must be harmless. Reset preview recreates the owner.

use crate::theme::button;
use gpui_kit::{Context, Entity, IntoElement, Render, WeakEntity, Window, div, prelude::*};

pub struct Record;
pub struct WeakPanel {
    owner: Option<Entity<Record>>,
    pub target: WeakEntity<Record>,
    status: &'static str,
}
impl WeakPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let owner = cx.new(|_| Record);
        Self {
            target: owner.downgrade(),
            owner: Some(owner),
            status: "Not inspected",
        }
    }
    fn inspect(&mut self, cx: &mut Context<Self>) {
        // TODO: Upgrade self.target instead of assuming it is gone.
        let target: Option<Entity<Record>> = None;
        self.status = if target.is_some() {
            "Available"
        } else {
            "Released"
        };
        cx.notify();
    }
}
impl Render for WeakPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(
                button("weak-inspect", "Inspect target", true)
                    .debug_selector(|| "weak-inspect".into())
                    .on_click(cx.listener(|this, _, _, cx| this.inspect(cx))),
            )
            .child(
                button("weak-release", "Release target", false)
                    .debug_selector(|| "weak-release".into())
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.owner.take();
                        this.status = "Target released · inspect again";
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .debug_selector(|| format!("weak-{}", self.status))
                    .child(self.status),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_14(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, cx| WeakPanel::new(cx));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let inspect = cx.debug_bounds("weak-inspect").unwrap();
        cx.simulate_click(inspect.center(), Modifiers::default());
        cx.update(|_, cx| {
            assert_eq!(
                panel.read(cx).status,
                "Available",
                "upgrade succeeds while the owner exists"
            )
        });
        let release = cx.debug_bounds("weak-release").unwrap();
        cx.simulate_click(release.center(), Modifiers::default());
        cx.update(|_, cx| {
            assert!(
                panel.read(cx).target.upgrade().is_none(),
                "inspection must not retain a strong owner"
            )
        });
        for _ in 0..2 {
            cx.simulate_click(inspect.center(), Modifiers::default());
        }
        cx.update(|window, cx| {
            assert_eq!(panel.read(cx).status, "Released");
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("weak-Released").is_some());
    }
}
