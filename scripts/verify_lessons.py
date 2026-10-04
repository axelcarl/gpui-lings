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
 '01_basics/basics2.rs': [('this.count.saturating_sub(1)', 'this.count.saturating_add(1)')],
 '01_basics/basics3.rs': [('let reached = self.count > 3;', 'let reached = self.count >= 3;')],
 '02_views/views1.rs': [('.flex_col()', '.flex_row()')],
 '02_views/views2.rs': [('this.active = false;', 'this.active = !this.active;')],
 '02_views/views3.rs': [('.gap(px(0.0))', '.gap(px(16.0))')],
 '03_contexts/contexts1.rs': [('// TODO: Tell GPUI this entity changed, so it renders again.',
                               '// TODO: Tell GPUI this entity changed, so it renders again.\n'
                               '                        cx.notify();')],
 '03_contexts/contexts2.rs': [('.on_click(|_, _, _| {})', '.on_click(cx.listener(Self::record_click))')],
 '03_contexts/contexts3.rs': [('let mut detached = this.score.read(cx).clone();\n'
                               '                        detached.value += 1;',
                               'this.score.update(cx, |score, cx| {\n'
                               '                            score.value += 1;\n'
                               '                            cx.notify();\n'
                               '                        });')],
 '03_contexts/contexts4.rs': [('this.mirrored = 0;', 'this.mirrored = reading.read(cx).value;')],
 '03_contexts/contexts5.rs': [('        // Subscribe to `Cleared` events too, and set `received` back to 0.\n',
                               '        // Subscribe to `Cleared` events too, and set `received` back to 0.\n'
                               '        let cleared = cx.subscribe(&sender, |this, _, _: &Cleared, cx| {\n'
                               '            this.received = 0;\n'
                               '            cx.notify();\n'
                               '        });\n'),
                              ('_subscriptions: Vec::new(),', '_subscriptions: vec![signals, cleared],')],
 'quizzes/quiz1.rs': [('        // TODO: Build the view described at the top of this file. The text\n'
                       '        // below only marks the spot in the preview; replace it.\n'
                       '        div().child("Your tally view goes here")\n',
                       '        div()\n'
                       '            .flex()\n'
                       '            .flex_col()\n'
                       '            .items_center()\n'
                       '            .gap_4()\n'
                       '            .child(\n'
                       '                div()\n'
                       '                    .debug_selector(|| format!("tally-{}", self.count))\n'
                       '                    .child(format!("{} taps", self.count)),\n'
                       '            )\n'
                       '            .child(\n'
                       '                div()\n'
                       '                    .flex()\n'
                       '                    .gap_2()\n'
                       '                    .child(\n'
                       '                        button("tally-tap", "Tap", true)\n'
                       '                            .debug_selector(|| "tally-tap".into())\n'
                       '                            .on_click(cx.listener(|this, _, _, cx| {\n'
                       '                                this.count += 1;\n'
                       '                                cx.notify();\n'
                       '                            })),\n'
                       '                    )\n'
                       '                    .child(\n'
                       '                        button("tally-clear", "Clear", false)\n'
                       '                            .debug_selector(|| "tally-clear".into())\n'
                       '                            .on_click(cx.listener(|this, _, _, cx| {\n'
                       '                                this.count = 0;\n'
                       '                                cx.notify();\n'
                       '                            })),\n'
                       '                    ),\n'
                       '            )\n')],
 '04_keyboard/keyboard1.rs': [('// This listener gets the `window` that can do that.\n',
                               '// This listener gets the `window` that can do that.\n'
                               '                        window.focus(&this.pad, cx);\n')],
 '04_keyboard/keyboard2.rs': [('                // a change, and leave every other key alone.\n',
                               '                // a change, and leave every other key alone.\n'
                               '                match event.keystroke.key.as_str() {\n'
                               '                    "left" => this.value = this.value.saturating_sub(1),\n'
                               '                    "right" => this.value += 1,\n'
                               '                    _ => return,\n'
                               '                }\n'
                               '                cx.notify();\n')],
 '04_keyboard/keyboard3.rs': [('            // action, so it goes nowhere. Handle it with `like`.\n',
                               '            // action, so it goes nowhere. Handle it with `like`.\n'
                               '            .on_action(cx.listener(Self::like))\n')],
 '04_keyboard/keyboard4.rs': [('.key_context("OtherPanel")', '.key_context("CommandPanel")')],
 '05_lifetimes/lifetimes1.rs': [('let record = Some(&self.target);', 'let record = self.target.upgrade();')],
 '05_lifetimes/lifetimes2.rs': [('        // second step, once this update has ended, and notify after that.\n',
                                 '        // second step, once this update has ended, and notify after that.\n'
                                 '        let weak = cx.weak_entity();\n'
                                 '        cx.defer(move |cx| {\n'
                                 '            let _ = weak.update(cx, |this, cx| {\n'
                                 '                this.history.push("Settled");\n'
                                 '                cx.notify();\n'
                                 '            });\n'
                                 '        });\n')],
 '05_lifetimes/lifetimes3.rs': [('        // before the timer fires. Keep it in the panel instead.\n',
                                 '        // before the timer fires. Keep it in the panel instead.\n'
                                 '        self.task = Some(task);\n')],
 'quizzes/quiz2.rs': [('    badge: WeakEntity<Badge>,', '    badge: Entity<Badge>,'),
                      ('            badge: badge.downgrade(),', '            badge,'),
                      ('            .children(self.badge.upgrade())', '            .child(self.badge.clone())'),
                      ('        let observer = cx.observe(inbox, move |this, _, cx| {\n'
                       '            this.unread = unread;',
                       '        let observer = cx.observe(inbox, |this, inbox, cx| {\n'
                       '            this.unread = inbox.read(cx).unread;'),
                      ('        let _ = cx.subscribe(inbox, |this, _, arrived: &Arrived, cx| {',
                       '        let subscription = cx.subscribe(inbox, |this, _, arrived: &Arrived, cx| {'),
                      ('            _subscriptions: Vec::new(),', '            _subscriptions: vec![subscription],'),
                      ('        cx.spawn(async move |this, cx| {\n'
                       '            cx.background_executor().timer(Duration::from_secs(1)).await;',
                       '        self.fetch = Some(cx.spawn(async move |this, cx| {\n'
                       '            cx.background_executor().timer(Duration::from_secs(1)).await;'),
                      ('        })\n        .detach();\n', '        }));\n')],
 '06_layout_states/layout_states1.rs': [('    if narrow {\n        cards.gap(px(8.0)).flex_row()',
                                         '    if narrow {\n        cards.gap(px(8.0)).flex_col()')],
 '06_layout_states/layout_states2.rs': [('.overflow_hidden()', '.overflow_y_scroll()'),
                                        ('// attached to it yet. Make this list track the handle.\n',
                                         '// attached to it yet. Make this list track the handle.\n'
                                         '                    .track_scroll(&self.scroll)\n')],
 '06_layout_states/layout_states3.rs': [('        self.selected = !self.selected;\n        cx.notify();',
                                         '        if !self.disabled {\n'
                                         '            self.selected = !self.selected;\n'
                                         '        }\n'
                                         '        cx.notify();')],
 '06_layout_states/layout_states4.rs': [('                    // The filled part of the track shows the value.\n',
                                         '                    // The filled part of the track shows the value.\n'
                                         '                    .on_mouse_move(cx.listener(|this, event: '
                                         '&MouseMoveEvent, _, cx| {\n'
                                         '                        this.move_pointer(event.position.x, cx);\n'
                                         '                    }))\n'
                                         '                    .on_mouse_up(\n'
                                         '                        MouseButton::Left,\n'
                                         '                        cx.listener(|this, _, _, cx| {\n'
                                         '                            this.drag_start = None;\n'
                                         '                            cx.notify();\n'
                                         '                        }),\n'
                                         '                    )\n'
                                         '                    .on_mouse_up_out(\n'
                                         '                        MouseButton::Left,\n'
                                         '                        cx.listener(|this, _, _, cx| {\n'
                                         '                            this.drag_start = None;\n'
                                         '                            cx.notify();\n'
                                         '                        }),\n'
                                         '                    )\n')],
 'quizzes/quiz3.rs': [('size.height < px(760.0)', 'size.width < px(760.0)'),
                      ('            .w(px(220.0))\n            .overflow_y_scroll()',
                       '            .w(px(220.0))\n            .h(px(168.0))\n            .overflow_y_scroll()'),
                      ('        self.selected = (self.selected + 1).min(11);\n        cx.notify();',
                       '        if !self.disabled {\n'
                       '            self.selected = (self.selected + 1).min(11);\n'
                       '            cx.notify();\n'
                       '        }'),
                      ('            .rounded_lg()\n            .children(',
                       '            .rounded_lg()\n'
                       '            .on_key_down(cx.listener(|this, event: &gpui_kit::KeyDownEvent, _, cx| {\n'
                       '                if event.keystroke.key == "j" {\n'
                       '                    this.next(cx);\n'
                       '                    cx.stop_propagation();\n'
                       '                }\n'
                       '            }))\n'
                       '            .children('),
                      ('            .border_color(c.border)\n'
                       '            .on_key_down(cx.listener(|this, event: &gpui_kit::KeyDownEvent, _, cx| {\n'
                       '                if event.keystroke.key == "j" {\n'
                       '                    this.next(cx);\n'
                       '                    cx.stop_propagation();\n'
                       '                }\n'
                       '            }))\n'
                       '            .child(format!',
                       '            .border_color(c.border)\n            .child(format!')],
 '07_dispatch/dispatch1.rs': [('                                    // drawn. Return it to the Open button.\n',
                               '                                    // drawn. Return it to the Open button.\n'
                               '                                    window.focus(&this.trigger, cx);\n'),
                              ("                        // Keys the pads don't handle travel up to this element.\n",
                               "                        // Keys the pads don't handle travel up to this element.\n"
                               '                        .on_key_down(cx.listener(\n'
                               '                            |this, event: &gpui_kit::KeyDownEvent, window, cx| {\n'
                               '                                if event.keystroke.key == "escape" {\n'
                               '                                    this.open = false;\n'
                               '                                    window.focus(&this.trigger, cx);\n'
                               '                                    cx.notify();\n'
                               '                                }\n'
                               '                            },\n'
                               '                        ))\n')],
 '07_dispatch/dispatch2.rs': [('            cx.notify();\n        }\n    }\n\n    fn route_in_parent',
                               '            cx.notify();\n'
                               '        } else {\n'
                               '            cx.propagate();\n'
                               '        }\n'
                               '    }\n'
                               '\n'
                               '    fn route_in_parent')],
 'quizzes/quiz4.rs': [('                                    "down" => this.selected = (this.selected + 1).min(1),\n'
                       '                                    "up" => this.selected = this.selected.saturating_sub(1),',
                       '                                    "down" => {\n'
                       '                                        this.selected = (this.selected + 1).min(1);\n'
                       '                                        cx.notify();\n'
                       '                                    }\n'
                       '                                    "up" => {\n'
                       '                                        this.selected = this.selected.saturating_sub(1);\n'
                       '                                        cx.notify();\n'
                       '                                    }'),
                      ('            .id("command-demo")\n',
                       '            .id("command-demo")\n            .key_context("CommandMenuDemo")\n'),
                      ('                        .key_context("CommandMenuDemo")\n', ''),
                      ('        self.open = false;\n        window.focus(&self.menu, cx);',
                       '        self.open = false;\n        window.focus(&self.launcher, cx);')],
 '08_async/async1.rs': [('        // Keep the foreground task in `self.task`.\n',
                         '        // Keep the foreground task in `self.task`.\n'
                         '        let timer = cx.background_executor().timer(Duration::from_secs(1));\n'
                         '        let work = cx.background_executor().spawn(async move {\n'
                         '            timer.await;\n'
                         '            (1..=1_000).sum::<u32>()\n'
                         '        });\n'
                         '        self.task = Some(cx.spawn(async move |this, cx| {\n'
                         '            let result = work.await;\n'
                         '            let _ = this.update(cx, |this, cx| {\n'
                         '                this.status = "Ready";\n'
                         '                this.total = result;\n'
                         '                cx.notify();\n'
                         '            });\n'
                         '        }));\n')],
 '08_async/async2.rs': [('        self.request(cx);\n    }\n}',
                         '        self.state = LoadState::Loading;\n'
                         '        cx.notify();\n'
                         '        self.request(cx);\n'
                         '    }\n'
                         '}')],
 '08_async/async3.rs': [('                this.visible = Some(label);',
                         '                if this.selected == Some(label) { this.visible = Some(label); }')],
 'quizzes/quiz5.rs': [('            query: cx.new(|cx| InputState::new(window, cx).placeholder("Search items")),\n',
                       '            query,\n'),
                      ('                if generation > this.generation {',
                       '                if generation != this.generation {'),
                      ('        if query == self.query_text {\n            return;\n        }\n', '')],
 '09_application/application1.rs': [('    None\n}',
                                     '    gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| cx.new(|_| '
                                     'WorkspaceRoot))\n'
                                     '        .ok()\n'
                                     '        .map(|(_, root)| root)\n'
                                     '}')],
 '09_application/application2.rs': [('_subscription: None,', '_subscription: Some(subscription),')],
 '09_application/application3.rs': [('    false\n}',
                                     '    fs::read_to_string(path).is_ok_and(|value| value.trim() == "compact")\n}')],
 '09_application/application4.rs': [('        // the callback before the detail window can close. Keep it in the '
                                     'panel.\n',
                                     '        // the callback before the detail window can close. Keep it in the '
                                     'panel.\n'
                                     '        self._close_subscription = Some(subscription);\n')],
 '09_application/application5.rs': [('let foreground = if dark {\n'
                                     '            palette.background\n'
                                     '        } else {\n'
                                     '            palette.foreground\n'
                                     '        };',
                                     'let foreground = palette.foreground;')],
 '10_quality/quality1.rs': [('            // Name it "Enable alerts" with Switch\'s `accessibility_label` method.\n',
                             '            // Name it "Enable alerts" with Switch\'s `accessibility_label` method.\n'
                             '            .accessibility_label("Enable alerts")\n')],
 '10_quality/quality2.rs': [('        // such as the center of `load`, and the modifier keys held down.\n',
                             '        // such as the center of `load`, and the modifier keys held down.\n'
                             '        window.simulate_click(load.center(), Modifiers::default());\n'),
                            ('        // `run_until_parked`.\n',
                             '        // `run_until_parked`.\n'
                             '        window.executor().advance_clock(Duration::from_secs(1));\n'
                             '        window.run_until_parked();\n')],
 '10_quality/quality3.rs': [('        let all = (0..ROWS).map(|ix| self.row(ix, cx)).collect::<Vec<_>>();\n'
                             '        all.into_iter()\n'
                             '            .skip(range.start)\n'
                             '            .take(range.len())\n'
                             '            .collect()',
                             '        range.map(|ix| self.row(ix, cx)).collect()')],
 '10_quality/quality4.rs': [('        .checked(false)', '        .checked(checked)')],
 'quizzes/quiz6.rs': [('        let _task = cx.spawn(async move |this, cx| {',
                       '        self._task = Some(cx.spawn(async move |this, cx| {'),
                      ('                cx.notify();\n            });\n        });\n    }',
                       '                cx.notify();\n            });\n        }));\n    }'),
                      ('    fn retry(&mut self, cx: &mut Context<Self>) {\n        self.request(cx);',
                       '    fn retry(&mut self, cx: &mut Context<Self>) {\n'
                       '        self.status = "Loading";\n'
                       '        cx.notify();\n'
                       '        self.request(cx);'),
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
