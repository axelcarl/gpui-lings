//! 01 — Render a greeting
//!
//! A view's Render method builds an element tree. div() creates a container;
//! child(...) adds an element or text. The playground passes welcome_text()
//! into its headline each time it renders.
//!
//! Goal: make the headline say Hello, GPUI! exactly. Change welcome_text(),
//! save, and inspect the app. Try different punctuation after passing to see
//! that the preview updates even when the check expects a specific string.
//!
//! Example — Building text elements:
//! (Illustrative names and fields; adapt them to the view below.)
//! ```ignore
//! let heading = div().child("Welcome to the workshop");
//! let card = div().child(heading);
//! ```

// The playground calls this function every time it renders its headline:
//     div().child(welcome_text())
// A `&'static str` is text that is stored in the program itself.
pub fn welcome_text() -> &'static str {
    // TODO: Greet the framework you are learning: return "Hello, GPUI!".
    "Hello, Rust!"
}

// The check that ./gpui-lings runs. Read it to see what passing means, but
// don't change it.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise_01() {
        assert_eq!(welcome_text(), "Hello, GPUI!");
    }
}
