#!/usr/bin/env python3
"""GitHub issue packet automation for icebox load/execute workflow."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from typing import Any

STATE_ORDER = ["draft", "ready-for-review", "ready-to-execute", "in-progress", "done"]
STATE_COLORS = {
    "draft": "8B949E",
    "ready-for-review": "1D76DB",
    "ready-to-execute": "0E8A16",
    "in-progress": "FBCA04",
    "done": "5319E7",
}

REQUIRED_LOADED_ITEMS = [
    "Backlog mapped",
    "Spec linked",
    "Tests mapped",
    "ADR triaged",
    "Docs impact listed",
]

REQUIRED_CLOSEOUT_FIELDS = [
    "PR link:",
    "Tests run",
    "Docs updated",
]


def run(cmd: list[str]) -> str:
    try:
        out = subprocess.run(cmd, check=True, text=True, capture_output=True)
        return out.stdout
    except FileNotFoundError:
        print("error: `gh` CLI is required but not found", file=sys.stderr)
        sys.exit(2)
    except subprocess.CalledProcessError as exc:
        print(exc.stderr.strip() or exc.stdout.strip(), file=sys.stderr)
        sys.exit(exc.returncode)


def gh_json(cmd: list[str]) -> dict[str, Any]:
    raw = run(cmd)
    try:
        return json.loads(raw)
    except json.JSONDecodeError:
        print("error: failed to parse JSON from gh output", file=sys.stderr)
        sys.exit(2)


def issue_data(issue_ref: str) -> dict[str, Any]:
    return gh_json(
        [
            "gh",
            "issue",
            "view",
            issue_ref,
            "--json",
            "number,title,body,labels,comments,url",
        ]
    )


def state_from_labels(labels: list[dict[str, Any]]) -> str | None:
    matches = [lbl["name"] for lbl in labels if lbl.get("name") in STATE_ORDER]
    if len(matches) > 1:
        print(f"error: multiple state labels found: {', '.join(matches)}", file=sys.stderr)
        sys.exit(1)
    return matches[0] if matches else None


def checklist_checked(body: str, item: str) -> bool:
    pattern = rf"^\s*-\s*\[(?P<mark>[ xX])\]\s*{re.escape(item)}\s*$"
    for line in body.splitlines():
        m = re.match(pattern, line.strip())
        if m and m.group("mark").lower() == "x":
            return True
    return False


def has_execution_plan_comment(comments: list[dict[str, Any]]) -> bool:
    for c in comments:
        body = c.get("body", "")
        if re.search(r"\bExecution Plan\b", body, re.IGNORECASE):
            return True
    return False


def is_nonempty_field(content: str, field_prefix: str) -> bool:
    for line in content.splitlines():
        if line.strip().lower().startswith(field_prefix.lower()):
            rhs = line.split(":", 1)[1].strip() if ":" in line else ""
            return rhs not in {"", "n/a", "na", "tbd", "none", "-"}
    return False


def adr_required(content: str) -> bool:
    return bool(re.search(r"ADR required\?\s*[:\-]\s*yes\b", content, re.IGNORECASE))


def ensure_labels() -> int:
    for label in STATE_ORDER:
        run(
            [
                "gh",
                "label",
                "create",
                label,
                "--color",
                STATE_COLORS[label],
                "--description",
                "Execution packet state",
                "--force",
            ]
        )
    print("ok: ensured state labels")
    return 0


def transition(issue_ref: str, target: str, dry_run: bool) -> int:
    if target not in STATE_ORDER:
        print(f"error: invalid target state {target}", file=sys.stderr)
        return 1

    data = issue_data(issue_ref)
    current = state_from_labels(data.get("labels", []))
    if current is None:
        print("error: issue has no state label", file=sys.stderr)
        return 1

    src_idx = STATE_ORDER.index(current)
    dst_idx = STATE_ORDER.index(target)
    if dst_idx != src_idx + 1 and dst_idx != src_idx:
        print(f"error: invalid transition {current} -> {target}", file=sys.stderr)
        return 1

    if dst_idx == src_idx:
        print(f"ok: already in state {target}")
        return 0

    remove = [s for s in STATE_ORDER if s != target and s in [l["name"] for l in data["labels"]]]
    cmd = ["gh", "issue", "edit", issue_ref, "--add-label", target]
    if remove:
        cmd += ["--remove-label", ",".join(remove)]

    if dry_run:
        print(f"dry-run: {' '.join(cmd)}")
        return 0

    run(cmd)
    print(f"ok: transitioned {issue_ref} {current} -> {target}")
    return 0


def validate_execute(issue_ref: str) -> int:
    data = issue_data(issue_ref)
    errors: list[str] = []
    state = state_from_labels(data.get("labels", []))
    if state != "ready-to-execute":
        errors.append("missing required state label: ready-to-execute")

    body = data.get("body", "")
    for item in REQUIRED_LOADED_ITEMS:
        if not checklist_checked(body, item):
            errors.append(f"unchecked required checklist item: {item}")

    if not has_execution_plan_comment(data.get("comments", [])):
        errors.append("missing required issue comment: Execution Plan")

    if errors:
        print("execute gate failed:")
        for e in errors:
            print(f"- {e}")
        return 1

    print(f"ok: execute gate passed for {issue_ref}")
    return 0


def validate_closeout(issue_ref: str) -> int:
    data = issue_data(issue_ref)
    content = data.get("body", "") + "\n\n" + "\n\n".join(c.get("body", "") for c in data.get("comments", []))
    errors: list[str] = []

    for field in REQUIRED_CLOSEOUT_FIELDS:
        if not is_nonempty_field(content, field):
            errors.append(f"missing closeout evidence field: {field}")

    if adr_required(content) and not is_nonempty_field(content, "ADR link:"):
        errors.append("ADR required but ADR link is missing/empty")

    if errors:
        print("closeout gate failed:")
        for e in errors:
            print(f"- {e}")
        return 1

    print(f"ok: closeout gate passed for {issue_ref}")
    return 0


def parse_args() -> argparse.Namespace:
    p = argparse.ArgumentParser(description="Issue packet automation")
    sub = p.add_subparsers(dest="cmd", required=True)

    sub.add_parser("ensure-labels", help="Create/update required state labels")

    tr = sub.add_parser("transition", help="Move issue to next lifecycle state")
    tr.add_argument("--issue", required=True, help="Issue number or #number")
    tr.add_argument("--to", required=True, choices=STATE_ORDER)
    tr.add_argument("--dry-run", action="store_true")

    ve = sub.add_parser("validate-execute", help="Validate execute gates")
    ve.add_argument("--issue", required=True, help="Issue number or #number")

    vc = sub.add_parser("validate-closeout", help="Validate closeout evidence gates")
    vc.add_argument("--issue", required=True, help="Issue number or #number")

    return p.parse_args()


def normalize_issue(ref: str) -> str:
    ref = ref.strip()
    if ref.startswith("#"):
        return ref[1:]
    return ref


def main() -> int:
    args = parse_args()
    if args.cmd == "ensure-labels":
        return ensure_labels()
    if args.cmd == "transition":
        return transition(normalize_issue(args.issue), args.to, args.dry_run)
    if args.cmd == "validate-execute":
        return validate_execute(normalize_issue(args.issue))
    if args.cmd == "validate-closeout":
        return validate_closeout(normalize_issue(args.issue))
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
