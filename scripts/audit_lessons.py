#!/usr/bin/env python3
"""Report how much each reference fix asks of the learner, and whether a TODO marks it.

Regions are fix sites more than three lines apart, so one fault can span several.
The marker is "line" when a TODO comment (with the comment lines that continue it)
sits within one line of a fix site,
"function" when the file has TODOs elsewhere, and "none" without any. Chapters 01–05
use line markers, later lessons function markers, and checkpoints none; see
lesson-plan.md#checkpoints-and-retrieval.
"""
import difflib
import re
import sys
from pathlib import Path

sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parent))
from verify_lessons import FIXES, ROOT  # noqa: E402

CATALOG = (ROOT / "shared/lessons.rs").read_text()
LESSONS = re.findall(r'id: "(\d+)",.*?title: "([^"]+)",.*?file: "exercises/([^"]+)"', CATALOG, re.S)


def audit(file):
    source = (ROOT / "exercises" / file).read_text()
    lines = source.splitlines()
    todos = []
    for i, line in enumerate(lines):
        if re.match(r"\s*//[^!].*TODO", line):
            end = i
            while end + 1 < len(lines) and re.match(r"\s*//[^!]", lines[end + 1]) and "TODO" not in lines[end + 1]:
                end += 1
            todos.append((i, end))
    spans, changed = [], 0
    for before, after in FIXES[file]:
        start = source[: source.index(before)].count("\n")
        spans.append((start, start + before.count("\n")))
        diff = difflib.ndiff(before.splitlines(), after.splitlines())
        changed += sum(line[:2] in ("+ ", "- ") for line in diff)
    spans.sort()
    regions = 1 + sum(b[0] - a[1] > 3 for a, b in zip(spans, spans[1:]))
    if any(s - 1 <= end and start <= e + 1 for s, e in spans for start, end in todos):
        marker = "line"
    else:
        marker = "function" if todos else "none"
    return regions, changed, marker


if __name__ == "__main__":
    print(f"{'':4}{'lesson':44}{'regions':>8}{'lines':>7}  marker")
    rows = [(id, title, *audit(file)) for id, title, file in LESSONS]
    for id, title, regions, changed, marker in rows:
        print(f"{id:4}{title[:42]:44}{regions:>8}{changed:>7}  {marker}")
    for marker in ("line", "function", "none"):
        ids = [r[0] for r in rows if r[4] == marker]
        print(f"\n{marker}: {len(ids)} lessons" + (f" ({', '.join(ids)})" if ids else ""), end="")
    print()
