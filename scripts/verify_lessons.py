#!/usr/bin/env python3
"""Verify red starter checks and green reference solutions without editing learner files."""
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
FIXES = {'01_basics/basics1.rs': [('"Hello, Rust!"', '"Hello, GPUI!"')],
 '01_basics/basics2.rs': [('count.saturating_sub(1)', 'count.saturating_add(1)')],
 '01_basics/basics3.rs': [('if count > 3', 'if count >= 3')],
 '02_views/views1.rs': [('.flex_col()', '.flex_row()')],
 '02_views/views2.rs': [('this.active = false;', 'this.active = !this.active;')],
 '02_views/views3.rs': [('.gap(px(0.0))', '.gap(px(16.0))')],
 '03_contexts/contexts1.rs': [('// TODO: Tell GPUI this entity changed, so it renders again.',
                         '// TODO: Tell GPUI this entity changed, so it renders again.\n                        cx.notify();')],
 '03_contexts/contexts2.rs': [('.on_click(|_, _, _| {})', '.on_click(cx.listener(Self::record_click))')],
 '03_contexts/contexts3.rs': [('let mut detached = this.score.read(cx).clone();\n'
                         '                        detached.value += 1;',
                         'this.score.update(cx, |score, cx| {\n'
                         '                            score.value += 1;\n'
                         '                            cx.notify();\n'
                         '                        });')],
 '03_contexts/contexts4.rs': [('this.mirrored = 0;', 'this.mirrored = reading.read(cx).value;')],
 '03_contexts/contexts5.rs': [('_subscription: None,', '_subscription: Some(subscription),')],
 '04_interaction/interaction1.rs': [('.key_context("OtherPanel")', '.key_context("CommandPanel")')],
 '04_interaction/interaction2.rs': [('// and `cx`.\n',
                           '// and `cx`.\n'
                           '                        window.focus(&this.pad, cx);\n')],
 '05_lifetimes/lifetimes1.rs': [('let record = Some(&self.target);', 'let record = self.target.upgrade();')],
 '05_lifetimes/lifetimes2.rs': [('        // the panel owns it: Cancel drops it, and a new load replaces it.\n',
                         '        // the panel owns it: Cancel drops it, and a new load replaces it.\n'
                         '        self.task = Some(task);\n')],
 '06_responsive/responsive1.rs': [('    if narrow {\n        cards.gap(px(8.0)).flex_row()',
                          '    if narrow {\n        cards.gap(px(8.0)).flex_col()')],
 '06_responsive/responsive2.rs': [('.overflow_hidden()', '.overflow_y_scroll()'),
                         ('// attached to it yet. Make this list track the handle.\n',
                          '// attached to it yet. Make this list track the handle.\n'
                          '                    .track_scroll(&self.scroll)\n')],
 '06_responsive/responsive3.rs': [('        self.selected = !self.selected;\n        cx.notify();',
                      '        if !self.disabled {\n            self.selected = !self.selected;\n        }\n        cx.notify();')],
 '06_responsive/responsive4.rs': [('cx.listener(|this, _, _, cx| {})',
                    'cx.listener(|this, _, _, cx| {\n                        this.drag_start = None;\n                        cx.notify();\n                    })')],
 'quizzes/quiz1.rs': [('size.height < px(760.0)', 'size.width < px(760.0)'),
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
 '07_deeper/deeper1.rs': [('                                    // `window.focus(...)` with `this.trigger` and `cx`.\n',
                          '                                    // `window.focus(...)` with `this.trigger` and `cx`.\n'
                          '                                    window.focus(&this.trigger, cx);\n')],
 '07_deeper/deeper2.rs': [('            cx.notify();\n        }\n    }\n\n    fn route_in_parent',
                                 '            cx.notify();\n        } else {\n            cx.propagate();\n        }\n    }\n\n    fn route_in_parent')],
 '07_deeper/deeper3.rs': [('        // Notify after the change.\n',
                           '        // Notify after the change.\n        let weak = cx.weak_entity();\n        cx.defer(move |cx| {\n            let _ = weak.update(cx, |this, cx| {\n                this.history.push("Settled");\n                cx.notify();\n            });\n        });\n')],
 'quizzes/quiz2.rs': [('                                    "down" => this.selected = (this.selected + 1).min(1),\n'
                          '                                    "up" => this.selected = this.selected.saturating_sub(1),',
                          '                                    "down" => {\n                                        this.selected = (this.selected + 1).min(1);\n'
                          '                                        cx.notify();\n                                    }\n'
                          '                                    "up" => {\n                                        this.selected = this.selected.saturating_sub(1);\n'
                          '                                        cx.notify();\n                                    }'),
                         ('            .id("command-demo")\n', '            .id("command-demo")\n            .key_context("CommandMenuDemo")\n'),
                         ('                        .key_context("CommandMenuDemo")\n', ''),
                         ('        self.open = false;\n        window.focus(&self.menu, cx);',
                          '        self.open = false;\n        window.focus(&self.launcher, cx);')],
 '08_async/async1.rs': [('                this.total = 0;\n',
                              '                this.total = result;\n')],
 '08_async/async2.rs': [('        self.request(cx);\n    }\n}',
                         '        self.state = LoadState::Loading;\n        cx.notify();\n        self.request(cx);\n    }\n}')],
 '08_async/async3.rs': [('                this.visible = Some(label);',
                         '                if this.selected == Some(label) { this.visible = Some(label); }')],
 'quizzes/quiz3.rs': [('            query: cx.new(|cx| InputState::new(window, cx).placeholder("Search items")),\n',
                          '            query,\n'),
                         ('                if generation > this.generation {', '                if generation != this.generation {'),
                         ('        if query == self.query_text {\n            return;\n        }\n', '')],
 '09_application/application1.rs': [('    None\n}',
                    '    gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| cx.new(|_| WorkspaceRoot))\n        .ok()\n        .map(|(_, root)| root)\n}')],
 '09_application/application2.rs': [('_subscription: None,',
                    '_subscription: Some(subscription),')],
 '09_application/application3.rs': [('    false\n}',
                        '    fs::read_to_string(path).is_ok_and(|value| value.trim() == "compact")\n}')],
 '09_application/application4.rs': [('        // `self._close_subscription`, so the panel hears about the close.\n',
                     '        // `self._close_subscription`, so the panel hears about the close.\n'
                     '        self._close_subscription = Some(subscription);\n')],
 '09_application/application5.rs': [('let foreground = if dark {\n            palette.background\n        } else {\n            palette.foreground\n        };',
                        'let foreground = palette.foreground;')],
 '10_quality/quality1.rs': [('            // "Enable alerts" with Switch\'s `accessibility_label` method.\n',
                           '            // "Enable alerts" with Switch\'s `accessibility_label` method.\n            .accessibility_label("Enable alerts")\n')],
 '10_quality/quality2.rs': [('        // example, give it a point inside `load` and the modifier keys held.\n',
                           '        // example, give it a point inside `load` and the modifier keys held.\n        window.simulate_click(load.center(), Modifiers::default());\n'),
                          ('        // `run_until_parked`.\n',
                           '        // `run_until_parked`.\n        window.executor().advance_clock(Duration::from_secs(1));\n        window.run_until_parked();\n')],
 '10_quality/quality3.rs': [('        let all = (0..ROWS).map(|ix| self.row(ix, cx)).collect::<Vec<_>>();\n        all.into_iter()\n            .skip(range.start)\n            .take(range.len())\n            .collect()',
                        '        range.map(|ix| self.row(ix, cx)).collect()')],
 '10_quality/quality4.rs': [('        .checked(false)', '        .checked(checked)')],
 'quizzes/quiz4.rs': [('        let _task = cx.spawn(async move |this, cx| {',
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
        env={**os.environ, "CARGO_TARGET_DIR": str(ROOT / "target")},
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
    check(ROOT / "Cargo.toml", expect_red=True)
    with tempfile.TemporaryDirectory(prefix="gpui-lings-reference-") as temporary:
        workspace = Path(temporary)
        ignore = shutil.ignore_patterns("target")
        for name in ("playground", "shared", "exercises", "guide"):
            shutil.copytree(ROOT / name, workspace / name, ignore=ignore)
        for name in ("Cargo.toml", "Cargo.lock"):
            shutil.copy2(ROOT / name, workspace / name)
        for file, replacements in FIXES.items():
            source = workspace / "exercises" / file
            text = source.read_text()
            for before, after in replacements:
                if text.count(before) != 1:
                    raise SystemExit(f"Reference patch no longer applies exactly once: {file}")
                text = text.replace(before, after)
            source.write_text(text)
        # Reuse dependency builds, but never reuse the starter's test binary:
        # exercises now live outside the package root and shared Cargo caches
        # can otherwise consider a copied crate fresh against the original files.
        manifest = workspace / "Cargo.toml"
        text = manifest.read_text()
        if text.count("[lib]\n") != 1:
            raise SystemExit("Cargo.toml no longer has exactly one [lib] table")
        manifest.write_text(text.replace("[lib]\n", '[lib]\nname = "gpui_lings_reference"\n'))
        check(manifest, expect_red=False)
    print("Learner source files were not modified.")
