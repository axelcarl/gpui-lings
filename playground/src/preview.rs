use gpui_kit::{
    App, Bounds, Context, DisplayId, Pixels, PlatformDisplay, Size, Subscription, Window,
    WindowBounds, point, px, size,
};
use std::{fs, path::Path};

pub fn watch_guide(cx: &mut App) {
    if std::env::var_os("GPUI_LINGS_MANAGED").is_none() {
        return;
    }
    // The guide owns the write end of this pipe. EOF also covers Ctrl-C,
    // terminal closure and crashes, where its normal cleanup cannot run.
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = std::io::copy(&mut std::io::stdin().lock(), &mut std::io::sink());
        let _ = tx.send(());
    });
    cx.spawn(async move |cx| {
        loop {
            cx.background_executor()
                .timer(std::time::Duration::from_millis(100))
                .await;
            if rx.try_recv().is_ok() {
                let _ = cx.update(|cx| cx.quit());
                break;
            }
        }
    })
    .detach();
}

fn display_key(display: &dyn PlatformDisplay) -> String {
    display
        .uuid()
        .map(|uuid| uuid.to_string())
        .unwrap_or_else(|_| format!("id:{}", u64::from(display.id())))
}

fn save_bounds(path: &Path, window: &Window, decoration: Size<Pixels>, cx: &App) {
    let Some(display) = window.display(cx) else {
        return;
    };
    let display = display_key(display.as_ref());
    let bounds = window.window_bounds();
    let mode = match bounds {
        WindowBounds::Windowed(_) => "windowed",
        WindowBounds::Maximized(_) => "maximized",
        WindowBounds::Fullscreen(_) => "fullscreen",
    };
    let mut restored = bounds.get_bounds();
    restored.size = restored.size - decoration;
    let bounds = restored;
    // Each process uses its own temporary file during the brief window overlap.
    let temporary = path.with_extension(format!("{}.tmp", std::process::id()));
    let value = format!(
        "{mode} {} {} {} {}\n{display}\n",
        f32::from(bounds.origin.x),
        f32::from(bounds.origin.y),
        f32::from(bounds.size.width),
        f32::from(bounds.size.height)
    );
    if fs::write(&temporary, value).is_ok() {
        let _ = fs::rename(temporary, path);
    }
}

pub fn track_bounds<T: 'static>(
    requested: WindowBounds,
    window: &mut Window,
    cx: &mut Context<T>,
) -> Option<Subscription> {
    let path = std::path::PathBuf::from(std::env::var_os("GPUI_LINGS_WINDOW_BOUNDS")?);
    // Some platforms report the outer frame but open with a content rectangle.
    // Measure the decoration once, including fractional borders, to avoid drift.
    let decoration = window.window_bounds().get_bounds().size - requested.get_bounds().size;
    save_bounds(&path, window, decoration, cx);
    Some(cx.observe_window_bounds(window, move |_, window, cx| {
        save_bounds(&path, window, decoration, cx)
    }))
}

fn parse_bounds(value: &str) -> Option<WindowBounds> {
    let mut values = value.split_whitespace();
    let mode = values.next()?;
    let numbers = values
        .map(str::parse::<f32>)
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    let [x, y, width, height] = numbers.as_slice() else {
        return None;
    };
    if !numbers.iter().all(|n| n.is_finite()) || *width <= 0.0 || *height <= 0.0 {
        return None;
    }
    let bounds = Bounds::new(
        point(px(*x), px(*y)),
        size(px(width.max(640.0)), px(height.max(560.0))),
    );
    match mode {
        "windowed" => Some(WindowBounds::Windowed(bounds)),
        "maximized" => Some(WindowBounds::Maximized(bounds)),
        "fullscreen" => Some(WindowBounds::Fullscreen(bounds)),
        _ => None,
    }
}

fn restore_on_display(
    saved: &str,
    displays: impl IntoIterator<Item = (String, DisplayId, Bounds<Pixels>)>,
) -> Option<(WindowBounds, DisplayId)> {
    let mut lines = saved.lines();
    let bounds = parse_bounds(lines.next()?)?;
    let saved_display = lines.next()?;
    // On macOS both monitors can have origin (0, 0): these coordinates only
    // identify a position when paired with the display they were measured on.
    let (_, id, display_bounds) = displays
        .into_iter()
        .find(|(key, _, _)| key == saved_display)?;
    display_bounds
        .intersects(&bounds.get_bounds())
        .then_some((bounds, id))
}

pub fn restored_placement(path: &Path, cx: &App) -> Option<(WindowBounds, DisplayId)> {
    let saved = fs::read_to_string(path).ok()?;
    // A disconnected monitor falls back to the centered default placement.
    restore_on_display(
        &saved,
        cx.displays().iter().map(|display| {
            (
                display_key(display.as_ref()),
                display.id(),
                display.bounds(),
            )
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restore_uses_the_saved_monitor_not_overlapping_local_coordinates() {
        let screen = Bounds::new(point(px(0.0), px(0.0)), size(px(1920.0), px(1080.0)));
        let saved = "windowed 140 170 880 650\nexternal-screen\n";
        let monitors = [
            ("laptop-screen".into(), DisplayId::new(1), screen),
            ("external-screen".into(), DisplayId::new(9), screen),
        ];
        let (bounds, display) = restore_on_display(saved, monitors.clone()).unwrap();
        assert_eq!(display, DisplayId::new(9));
        assert_eq!(bounds.get_bounds().origin, point(px(140.0), px(170.0)));
        assert_eq!(bounds.get_bounds().size, size(px(880.0), px(650.0)));
        // Reconnecting can change the runtime ID without changing the monitor UUID.
        let reconnected = [("external-screen".into(), DisplayId::new(12), screen)];
        assert_eq!(
            restore_on_display(saved, reconnected).unwrap().1,
            DisplayId::new(12)
        );
        assert!(restore_on_display(saved, [monitors[0].clone()]).is_none());
        assert!(
            restore_on_display("windowed 5000 170 880 650\nexternal-screen\n", monitors).is_none()
        );
    }

    #[test]
    fn bounds_preserve_position_size_and_mode() {
        for mode in ["windowed", "maximized", "fullscreen"] {
            let value = parse_bounds(&format!("{mode} -1200 80 900 700")).unwrap();
            assert_eq!(
                value.get_bounds(),
                Bounds::new(point(px(-1200.0), px(80.0)), size(px(900.0), px(700.0)))
            );
            assert_eq!(
                matches!(value, WindowBounds::Maximized(_)),
                mode == "maximized"
            );
            assert_eq!(
                matches!(value, WindowBounds::Fullscreen(_)),
                mode == "fullscreen"
            );
        }
        for value in [
            "",
            "windowed 0 0",
            "windowed NaN 0 960 760",
            "windowed 0 0 -1 760",
            "windowed 0 0 inf 760",
            "other 0 0 960 760",
        ] {
            assert!(parse_bounds(value).is_none(), "{value}");
        }
    }
}
