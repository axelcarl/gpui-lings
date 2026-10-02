//! 02 — Update state on click
//!
//! The Playground view owns count. Its button calls increment(&mut this.count)
//! from a click listener, then cx.notify() requests a redraw. The wiring is
//! ready; this exercise changes the state transition inside increment().
//!
//! Goal: two clicks should move the count from 0 to 1 to 2. Fix the direction
//! of the update. Reset preview returns to zero without changing your source.
//! Later exercises explore the listener and context behind this interaction.
//!
//! Example — Calling a state helper from a listener:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! cx.listener(|this, _: &ClickEvent, _, cx| {
//!     increment(&mut this.count);
//!     cx.notify();
//! })
//! ```

pub fn increment(count: &mut u32) {
    // TODO: Move the count forward by one on every click.
    *count = count.saturating_sub(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise_02() {
        let mut count = 0;
        increment(&mut count);
        assert_eq!(count, 1, "one click should increase the visible count");
        increment(&mut count);
        assert_eq!(count, 2, "the next click should increase it again");
    }
}
