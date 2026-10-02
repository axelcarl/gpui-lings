//! shadcn/ui's neutral palette and the few primitives the playground needs.
//! The palette follows the window's light or dark appearance.

use gpui_kit::{prelude::*, *};
use std::cell::Cell;

pub const MONO: &str = "Menlo";

pub struct Palette {
    pub background: Rgba,
    pub foreground: Rgba,
    pub card: Rgba,
    pub primary: Rgba,
    pub primary_foreground: Rgba,
    pub muted: Rgba,
    pub muted_foreground: Rgba,
    pub accent: Rgba,
    pub destructive: Rgba,
    /// Tailwind green, paired with shadcn's destructive red for check status.
    pub success: Rgba,
    pub loading: Rgba,
    pub border: Rgba,
    pub input: Rgba,
    pub ring: Rgba,
}

const fn hex(value: u32) -> Rgba {
    Rgba {
        r: ((value >> 16) & 0xff) as f32 / 255.0,
        g: ((value >> 8) & 0xff) as f32 / 255.0,
        b: (value & 0xff) as f32 / 255.0,
        a: 1.0,
    }
}

pub const fn alpha(color: Rgba, a: f32) -> Rgba {
    Rgba { a, ..color }
}

const TRANSPARENT: Rgba = alpha(hex(0x000000), 0.0);

pub const LIGHT: Palette = Palette {
    background: hex(0xffffff),
    foreground: hex(0x0a0a0a),
    card: hex(0xffffff),
    primary: hex(0x171717),
    primary_foreground: hex(0xfafafa),
    muted: hex(0xf5f5f5),
    muted_foreground: hex(0x737373),
    accent: hex(0xf5f5f5),
    destructive: hex(0xe7000b),
    success: hex(0x00a63e),
    loading: hex(0x2563eb),
    border: hex(0xe5e5e5),
    input: hex(0xe5e5e5),
    ring: hex(0xa1a1a1),
};

pub const DARK: Palette = Palette {
    background: hex(0x0a0a0a),
    foreground: hex(0xfafafa),
    card: hex(0x171717),
    primary: hex(0xe5e5e5),
    primary_foreground: hex(0x171717),
    muted: hex(0x262626),
    muted_foreground: hex(0xa1a1a1),
    accent: hex(0x262626),
    destructive: hex(0xff6467),
    success: hex(0x05df72),
    loading: hex(0x60a5fa),
    border: alpha(hex(0xffffff), 0.1),
    input: alpha(hex(0xffffff), 0.15),
    ring: hex(0x737373),
};

thread_local! {
    static IS_DARK: Cell<bool> = const { Cell::new(false) };
}

/// Choose the palette for everything rendered after this call.
pub fn use_appearance(appearance: WindowAppearance) {
    IS_DARK.set(matches!(
        appearance,
        WindowAppearance::Dark | WindowAppearance::VibrantDark
    ));
}

pub fn colors() -> &'static Palette {
    if IS_DARK.get() { &DARK } else { &LIGHT }
}

/// The 3px focus ring shadcn draws around keyboard-focused controls.
pub fn focus_ring(style: StyleRefinement) -> StyleRefinement {
    let c = colors();
    style.border_color(c.ring).shadow(vec![BoxShadow {
        color: alpha(c.ring, 0.5).into(),
        offset: point(px(0.0), px(0.0)),
        blur_radius: px(0.0),
        spread_radius: px(3.0),
        inset: false,
    }])
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Primary,
    Outline,
}

/// A button without content, for callers that add an icon or dynamic label.
pub fn button_base(id: impl Into<ElementId>, variant: Variant) -> Stateful<Div> {
    let c = colors();
    let base = div()
        .id(id)
        .tab_index(0)
        .role(Role::Button)
        .flex()
        .flex_shrink_0()
        .items_center()
        .justify_center()
        .gap_2()
        .h_9()
        .px_4()
        .rounded_md()
        .border_1()
        .text_sm()
        .font_weight(FontWeight::MEDIUM)
        .whitespace_nowrap()
        .cursor_pointer()
        .focus_visible(focus_ring);
    match variant {
        Variant::Primary => base
            .border_color(TRANSPARENT)
            .bg(c.primary)
            .text_color(c.primary_foreground)
            .shadow_xs()
            .hover(|style| style.bg(alpha(c.primary, 0.9))),
        Variant::Outline => base
            .border_color(c.input)
            .bg(c.background)
            .text_color(c.foreground)
            .shadow_xs()
            .hover(|style| style.bg(c.accent)),
    }
}

/// A primary or outline button with a text label.
pub fn button(id: &'static str, label: &'static str, primary: bool) -> Stateful<Div> {
    let variant = if primary {
        Variant::Primary
    } else {
        Variant::Outline
    };
    button_base(id, variant).aria_label(label).child(label)
}

pub fn icon(data: &'static [u8]) -> Svg {
    svg().data(data).size_4().flex_shrink_0()
}

/// A small status label; primary badges are filled, others are outlined.
pub fn badge(primary: bool) -> Div {
    let c = colors();
    div()
        .flex()
        .flex_shrink_0()
        .items_center()
        .gap_1()
        .px_2()
        .py_0p5()
        .rounded_md()
        .border_1()
        .text_xs()
        .font_weight(FontWeight::MEDIUM)
        .whitespace_nowrap()
        .map(|badge| {
            if primary {
                badge
                    .border_color(TRANSPARENT)
                    .bg(c.primary)
                    .text_color(c.primary_foreground)
            } else {
                badge.border_color(c.border).text_color(c.foreground)
            }
        })
}

/// Inline code, as in shadcn's typography examples.
pub fn code(text: impl Into<SharedString>) -> Div {
    let c = colors();
    div()
        .px(px(6.0))
        .py(px(2.0))
        .rounded(px(4.0))
        .bg(c.muted)
        .font_family(MONO)
        .text_xs()
        .child(text.into())
}
