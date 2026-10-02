//! 03 — Render derived state
//!
//! The view stores a count and computes its message from that count on each
//! render. Derived text needs no second state variable to keep synchronized.
//! The threshold below is off by one.
//!
//! Goal: show You reached three! at three clicks and beyond. Compare 2, 3,
//! and 4, then reset. This preview has its own working increment so you can
//! explore the boundary without first solving exercise 02.
//!
//! Example — Deriving text during render:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! let caption = format!("{} items", self.items.len());
//! div().child(caption)
//! ```

// The playground stores only the count. Each render calls this function to
// turn it into a message, so the text can never disagree with the number.
pub fn milestone_text(count: u32) -> &'static str {
    // TODO: This comparison is off by one. Celebrate as soon as the count
    // reaches three, not only after it passes three.
    if count > 3 {
        "You reached three!"
    } else {
        "Keep clicking to reach three."
    }
}

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise_03() {
        assert_eq!(milestone_text(0), "Keep clicking to reach three.");
        assert_eq!(milestone_text(2), "Keep clicking to reach three.");
        assert_eq!(milestone_text(3), "You reached three!");
        assert_eq!(milestone_text(4), "You reached three!");
    }
}
