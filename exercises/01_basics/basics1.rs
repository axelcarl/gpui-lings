// Welcome to GPUI Lings! Every exercise is a small piece of a GPUI app with
// something wrong in it. Fix it, save the file, and the terminal checks your
// change and rebuilds the preview window. Once the check passes, you're free
// to keep experimenting. Press `n` in the terminal when you're ready to move
// on, or `h` whenever you'd like a hint.
//
// GPUI draws a view as a tree of elements. `div()` creates a container, and
// `.child(...)` puts something inside it, such as a piece of text. The
// playground builds its headline that way, from what `welcome_text` returns.

// The playground calls this function every time it renders its headline:
//     div().child(welcome_text())
// A `&'static str` is text that is stored in the program itself.
pub fn welcome_text() -> &'static str {
    // TODO: Greet the framework you're learning instead: "Hello, GPUI!".
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
