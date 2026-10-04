// A key handler, like lesson 14's, belongs to one element and one key. A
// command that a button and a shortcut should both trigger is better as an
// *action*: a named command. `actions!` declares actions, a `KeyBinding` maps a
// keystroke to one, and `on_action` says what an element does when the action
// reaches it. Actions travel from the focused element up through its parents,
// the same way keys do.
//
// Here, the Like button already calls `like`. Ctrl-L is bound to the `Like`
// action too, but no element handles that action yet. Click "Focus panel"
// before you press Ctrl-L: as in lesson 13, keys only arrive once something
// has focus.

use crate::theme::{button, colors, focus_ring};
use gpui_kit::{
    Context, FocusHandle, IntoElement, KeyBinding, Render, Window, actions, div, prelude::*, px,
};

// Declares the `Like` action: a command that a key binding can trigger.
actions!(gpui_lings_likes, [Like]);

pub struct LikesPanel {
    focus: FocusHandle,
    likes: u32,
}

impl LikesPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        // Ctrl-L dispatches `Like`. The binding names no key context, so it
        // matches wherever focus is; lesson 16 narrows that down.
        cx.bind_keys([KeyBinding::new("ctrl-l", Like, None)]);
        Self {
            focus: cx.focus_handle(),
            likes: 0,
        }
    }

    // The command itself. Its arguments are what `on_action` passes along:
    // the action, the window and this view's context.
    fn like(&mut self, _: &Like, _: &mut Window, cx: &mut Context<Self>) {
        self.likes += 1;
        cx.notify();
    }
}

impl Render for LikesPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focused = self.focus.is_focused(window);
        div()
            .id("likes-panel")
            .w(px(320.0))
            .track_focus(&self.focus)
            // TODO: Ctrl-L dispatches `Like`, but nothing here handles that
            // action, so it goes nowhere. Handle it with `like`.
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .p_6()
            .rounded_xl()
            .border_1()
            .border_color(colors().border)
            .bg(colors().card)
            .focus(focus_ring)
            .child(
                button("likes-focus", "Focus panel", false)
                    .debug_selector(|| "likes-focus".into())
                    .on_click(cx.listener(|this, _, window, cx| window.focus(&this.focus, cx))),
            )
            .child(
                button("likes-like", "Like", true)
                    .debug_selector(|| "likes-like".into())
                    // One command, two ways in: this button and Ctrl-L.
                    .on_click(cx.listener(|this, _, window, cx| this.like(&Like, window, cx))),
            )
            .child(
                div()
                    .debug_selector(|| format!("likes-{}", self.likes))
                    .child(format!("{} likes", self.likes)),
            )
            .child(if focused {
                "Focused · press Ctrl-L"
            } else {
                "Not focused · click Focus panel first"
            })
    }
}

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Modifiers, TestAppContext};

    #[gpui::test]
    fn exercise_15(cx: &mut TestAppContext) {
        let (panel, cx) = cx.add_window_view(|_, cx| LikesPanel::new(cx));
        cx.update(|window, cx| window.draw(cx).clear(cx));
        let like = cx.debug_bounds("likes-like").unwrap();
        cx.simulate_click(like.center(), Modifiers::default());
        cx.update(|_, cx| assert_eq!(panel.read(cx).likes, 1, "the Like button should like"));

        let focus = cx.debug_bounds("likes-focus").unwrap();
        cx.simulate_click(focus.center(), Modifiers::default());
        cx.simulate_keystrokes("ctrl-l ctrl-l");
        cx.update(|window, cx| {
            assert_eq!(
                panel.read(cx).likes,
                3,
                "Ctrl-L should run the same command as the button"
            );
            window.draw(cx).clear(cx);
        });
        assert!(cx.debug_bounds("likes-3").is_some());
    }
}
