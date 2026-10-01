#!/usr/bin/env python3
"""Verify red starter checks and green reference solutions without editing learner files."""
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
FIXES = {'basics/greeting.rs': [('"Hello, Rust!"', '"Hello, GPUI!"')],
 'basics/counter.rs': [('count.saturating_sub(1)', 'count.saturating_add(1)')],
 'basics/milestone.rs': [('if count > 3', 'if count >= 3')],
 'views/layout.rs': [('.flex_col()', '.flex_row()')],
 'views/entity.rs': [('this.active = false;', 'this.active = !this.active;')],
 'views/spacing.rs': [('.gap(px(0.0))', '.gap(px(16.0))')],
 'contexts/notify.rs': [('// TODO: Tell GPUI this entity changed.',
                         '// TODO: Tell GPUI this entity changed.\n                        _cx.notify();')],
 'contexts/listener.rs': [('.on_click(|_, _, _| {})', '.on_click(_cx.listener(Self::record_click))')],
 'contexts/update.rs': [('let mut detached = this.score.read(cx).clone();\n'
                         '                        detached.value += 1;\n'
                         '                        let _ = detached.value;',
                         'this.score.update(cx, |score, cx| {\n'
                         '                            score.value += 1;\n'
                         '                            cx.notify();\n'
                         '                        });')],
 'contexts/observe.rs': [('this.mirrored = 0;', 'this.mirrored = _reading.read(cx).value;')],
 'contexts/events.rs': [('        drop(subscription);\n', ''),
                        ('_subscription: None,', '_subscription: Some(subscription),')],
 'interaction/actions.rs': [('.key_context("OtherPanel")', '.key_context("CommandPanel")')],
 'interaction/focus.rs': [('// TODO: Move focus to this.pad using the window.',
                           '// TODO: Move focus to this.pad using the window.\n'
                           '                        _window.focus(&_this.pad, _cx);')],
 'lifetimes/weak.rs': [('let target: Option<Entity<Record>> = None;', 'let target = self.target.upgrade();')],
 'lifetimes/tasks.rs': [('        drop(task);', '        self.task = Some(task);')],
 'views/responsive.rs': [('    if narrow {\n        cards.gap(px(8.0)).flex_row()',
                          '    if narrow {\n        cards.gap(px(8.0)).flex_col()')],
 'views/scrolling.rs': [('.overflow_hidden()', '.overflow_y_scroll()')],
 'views/states.rs': [('        self.selected = !self.selected;\n        cx.notify();',
                      '        if !self.disabled {\n            self.selected = !self.selected;\n        }\n        cx.notify();')],
 'views/drag.rs': [('cx.listener(|_this, _, _, _cx| {})',
                    'cx.listener(|this, _, _, cx| {\n                        this.drag_start = None;\n                        cx.notify();\n                    })')],
 'views/inspector.rs': [('size.height < px(760.0)', 'size.width < px(760.0)'),
                        ('            .w(px(220.0))\n            .overflow_y_scroll()',
                         '            .w(px(220.0))\n            .h(px(168.0))\n            .overflow_y_scroll()'),
                        ('        self.selected = (self.selected + 1).min(11);\n        cx.notify();',
                         '        if !self.disabled {\n            self.selected = (self.selected + 1).min(11);\n            cx.notify();\n        }'),
                        ('            .rounded_lg()\n            .children(',
                         '            .rounded_lg()\n            .on_key_down(cx.listener(|this, event: &gpui_kit::KeyDownEvent, _, cx| {\n'
                         '                if event.keystroke.key == "j" {\n                    this.next(cx);\n                    cx.stop_propagation();\n'
                         '                }\n            }))\n            .children('),
                        ('            .border_color(c.border)\n            .on_key_down(cx.listener(|this, event: &gpui_kit::KeyDownEvent, _, cx| {\n'
                         '                if event.keystroke.key == "j" {\n                    this.next(cx);\n                    cx.stop_propagation();\n'
                         '                }\n            }))\n            .child(format!',
                         '            .border_color(c.border)\n            .child(format!')],
 'interaction/regions.rs': [('                                    this.open = false;\n                                    cx.notify();',
                             '                                    this.open = false;\n                                    _window.focus(&this.trigger, cx);\n                                    cx.notify();')],
 'interaction/propagation.rs': [('            cx.notify();\n        }\n    }\n\n    fn route_in_parent',
                                 '            cx.notify();\n        } else {\n            cx.propagate();\n        }\n    }\n\n    fn route_in_parent')],
 'contexts/deferred.rs': [('        self.history.push("Queued");\n        cx.notify();\n    }',
                           '        self.history.push("Queued");\n        cx.notify();\n        let weak = cx.weak_entity();\n        cx.defer(move |cx| {\n            let _ = weak.update(cx, |this, cx| {\n                this.history.push("Settled");\n                cx.notify();\n            });\n        });\n    }')],
 'interaction/menu.rs': [('                                    "down" => this.selected = (this.selected + 1).min(1),\n'
                          '                                    "up" => this.selected = this.selected.saturating_sub(1),',
                          '                                    "down" => {\n                                        this.selected = (this.selected + 1).min(1);\n'
                          '                                        cx.notify();\n                                    }\n'
                          '                                    "up" => {\n                                        this.selected = this.selected.saturating_sub(1);\n'
                          '                                        cx.notify();\n                                    }'),
                         ('            .id("command-demo")\n', '            .id("command-demo")\n            .key_context("CommandMenuDemo")\n'),
                         ('                        .key_context("CommandMenuDemo")\n', ''),
                         ('        self.open = false;\n        window.focus(&self.menu, cx);',
                          '        self.open = false;\n        window.focus(&self.launcher, cx);')],
 'lifetimes/background.rs': [('                this.total = 0;\n                let _ = result;',
                              '                this.total = result;')],
 'lifetimes/retry.rs': [('        self.request(cx);\n    }\n}',
                         '        self.state = LoadState::Loading;\n        cx.notify();\n        self.request(cx);\n    }\n}')],
 'lifetimes/stale.rs': [('                this.visible = Some(label);',
                         '                if this.selected == Some(label) { this.visible = Some(label); }')],
 'lifetimes/search.rs': [('            query: cx.new(|cx| InputState::new(window, cx).placeholder("Search items")),\n',
                          '            query,\n'),
                         ('                if generation > this.generation {', '                if generation != this.generation {'),
                         ('        if query == self.query_text {\n            return;\n        }\n', '')],
 'app/startup.rs': [('    let _ = (cx, WindowOptions::default());\n    None',
                    '    gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| cx.new(|_| WorkspaceRoot))\n        .ok()\n        .map(|(_, root)| root)')],
 'app/shared.rs': [('let _subscription = cx.observe_global',
                    'let subscription = cx.observe_global'),
                   ('_subscription: None,',
                    '_subscription: Some(subscription),')],
 'app/persistence.rs': [('    let _ = path;\n    false',
                        '    fs::read_to_string(path).is_ok_and(|value| value.trim() == "compact")')],
 'app/windows.rs': [('        let _ = subscription;',
                     '        self._close_subscription = Some(subscription);')],
 'app/appearance.rs': [('let foreground = if dark {\n            palette.background\n        } else {\n            palette.foreground\n        };',
                        'let foreground = palette.foreground;')],
 'app/accessibility.rs': [('            .checked(enabled)\n            .on_change(on_change)',
                           '            .checked(enabled)\n            .accessibility_label("Enable alerts")\n            .on_change(on_change)')],
 'app/behavior_test.rs': [('        let _ = (load, Modifiers::default());',
                           '        window.simulate_click(load.center(), Modifiers::default());'),
                          ('        assert!(window.debug_bounds("behavior-Loading").is_some());\n',
                           '        assert!(window.debug_bounds("behavior-Loading").is_some());\n        window.executor().advance_clock(Duration::from_secs(1));\n        window.run_until_parked();\n')],
 'app/large_list.rs': [('        let all = (0..ROWS).map(|ix| self.row(ix, cx)).collect::<Vec<_>>();\n        all.into_iter()\n            .skip(range.start)\n            .take(range.len())\n            .collect()',
                        '        range.map(|ix| self.row(ix, cx)).collect()')],
 'app/reusable.rs': [('        .checked(false)', '        .checked(checked)')],
 'app/capstone.rs': [('        let _task = cx.spawn(async move |this, cx| {',
                       '        self._task = Some(cx.spawn(async move |this, cx| {'),
                      ('                cx.notify();\n            });\n        });\n    }',
                       '                cx.notify();\n            });\n        }));\n    }'),
                      ('    fn retry(&mut self, cx: &mut Context<Self>) {\n        self.request(cx);',
                       '    fn retry(&mut self, cx: &mut Context<Self>) {\n        self.status = "Loading";\n        cx.notify();\n        self.request(cx);'),
                      ('size.width > px(620.0)', 'size.width < px(620.0)'),
                      ('        let selected = self.selected;', '        let selected = self.model.read(cx).selected;'),
                      ('    selected: usize,\n    _task', '    _task'),
                      ('            selected: 0,\n', ''),
                      ('            .id("notes-workspace")\n',
                       '            .id("notes-workspace")\n            .track_focus(&self.focus)\n')]}


def check(manifest, expect_red):
    result = subprocess.run(
        ["cargo", "test", "--offline", "--manifest-path", str(manifest), "--lib", "--", "--color=never"],
        # Reuse dependencies already built by the native playground.
        env={**os.environ, "CARGO_TARGET_DIR": str(ROOT / "playground/target")},
        text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
    )
    failed = re.findall(r"^test ([\w:]+) \.\.\. FAILED$", result.stdout, re.MULTILINE)
    expected = {f"exercise_{i:02}" for i in range(1, len(FIXES) + 1)}
    if expect_red:
        valid = result.returncode != 0 and len(failed) == len(FIXES) and {name.split("::")[-1] for name in failed} == expected
    else:
        passed = re.findall(r"^test ([\w:]+) \.\.\. ok$", result.stdout, re.MULTILINE)
        valid = result.returncode == 0 and expected <= {name.split("::")[-1] for name in passed}
    if not valid:
        raise SystemExit(result.stdout)
    print(f"✓ All {len(FIXES)} starter checks fail as intended; app checks pass." if expect_red else f"✓ All {len(FIXES)} reference solutions and app checks pass.", flush=True)


if __name__ == "__main__":
    check(ROOT / "playground/Cargo.toml", expect_red=True)
    with tempfile.TemporaryDirectory(prefix="gpui-lings-reference-") as temporary:
        workspace = Path(temporary)
        playground = workspace / "playground"
        shutil.copytree(ROOT / "playground/src", playground / "src")
        shutil.copytree(ROOT / "shared", workspace / "shared")
        for name in ("Cargo.toml", "Cargo.lock"):
            shutil.copy2(ROOT / "playground" / name, playground / name)
        for file, replacements in FIXES.items():
            source = playground / "src/exercises" / file
            text = source.read_text()
            for before, after in replacements:
                if text.count(before) != 1:
                    raise SystemExit(f"Reference patch no longer applies exactly once: {file}")
                text = text.replace(before, after)
            source.write_text(text)
        check(playground / "Cargo.toml", expect_red=False)
    print("Learner source files were not modified.")
