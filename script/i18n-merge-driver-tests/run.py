#!/usr/bin/env python3
"""Run all i18n-merge-driver fixtures and report pass/fail."""
from __future__ import annotations

import subprocess
import sys
import shutil
from pathlib import Path

ROOT = Path(__file__).resolve().parent
DRIVER = ROOT.parent / 'i18n-merge-driver'


def run_case(case_dir: Path) -> tuple[bool, str]:
    base, ours, theirs = (case_dir / n for n in ('base', 'ours', 'theirs'))
    expected = (case_dir / 'expected').read_text()
    expect_exit = int((case_dir / 'expect_exit').read_text().strip())

    actual = case_dir / 'actual'
    shutil.copy(ours, actual)
    try:
        cp = subprocess.run(
            [str(DRIVER), str(base), str(actual), str(theirs)],
            capture_output=True, text=True,
        )
        got = actual.read_text()
        if cp.returncode != expect_exit:
            return False, f"exit code: expected {expect_exit}, got {cp.returncode}\nstderr:\n{cp.stderr}"
        if got != expected:
            return False, f"output mismatch:\n--- expected ---\n{expected!r}\n--- got ---\n{got!r}"
        return True, ""
    finally:
        actual.unlink(missing_ok=True)


def main() -> int:
    cases = sorted(p for p in ROOT.iterdir() if p.is_dir() and p.name.startswith('case-'))
    if not cases:
        print("no fixtures found", file=sys.stderr)
        return 1

    fails = 0
    for case in cases:
        ok, msg = run_case(case)
        mark = "✓" if ok else "✗"
        print(f"  {mark} {case.name}")
        if not ok:
            print(f"    {msg.replace(chr(10), chr(10) + '    ')}")
            fails += 1

    total = len(cases)
    print(f"\n{total - fails}/{total} passed")
    return 0 if fails == 0 else 1


if __name__ == '__main__':
    sys.exit(main())
