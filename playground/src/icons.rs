//! A few Lucide icons (ISC license), embedded so the app needs no asset source.
//! GPUI paints an SVG as a mask, so set the icon's color with text_color.

macro_rules! lucide {
    ($body:literal) => {
        concat!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">"#,
            $body,
            "</svg>"
        )
        .as_bytes()
    };
}

pub const CIRCLE_CHECK: &[u8] =
    lucide!(r#"<circle cx="12" cy="12" r="10"/><path d="m9 12 2 2 4-4"/>"#);
pub const CIRCLE_X: &[u8] =
    lucide!(r#"<circle cx="12" cy="12" r="10"/><path d="m15 9-6 6"/><path d="m9 9 6 6"/>"#);
pub const CIRCLE_ALERT: &[u8] = lucide!(
    r#"<circle cx="12" cy="12" r="10"/><line x1="12" x2="12" y1="8" y2="12"/><line x1="12" x2="12.01" y1="16" y2="16"/>"#
);
pub const CHEVRON_RIGHT: &[u8] = lucide!(r#"<path d="m9 18 6-6-6-6"/>"#);
pub const ROTATE_CCW: &[u8] =
    lucide!(r#"<path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/>"#);
