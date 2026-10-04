// A strong `Entity<T>` keeps its state alive. A `WeakEntity<T>` only remembers
// which entity it points at, without keeping it alive. That's what you want
// for a back-reference, or for a callback that shouldn't keep a closed view
// around.
//
// Before you use a weak handle, call `upgrade()`. It returns `Some(entity)`
// while an owner still holds the entity, and `None` once the last strong
// handle is gone. Try it in the preview: inspect, release the target, then
// inspect again.

use crate::theme::button;
use gpui_kit::{Context, Entity, IntoElement, Render, WeakEntity, Window, div, prelude::*};

pub struct Record;
pub struct WeakPanel {
    // The strong handle that keeps the record alive. Release target drops it.
    owner: Option<Entity<Record>>,
    // A weak handle to the same record: it does not keep the record alive.
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
        // TODO: A weak handle exists even after its record is released, so
        // wrapping it in `Some` proves nothing. Upgrade it instead.
        let record = Some(&self.target);
        self.status = if record.is_some() {
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

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_17(cx: &mut TestAppContext) {
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
            assert_eq!(
                panel.read(cx).status,
                "Released",
                "after Release target, inspecting must report Released"
            );
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("weak-Released").is_some());
    }
}
