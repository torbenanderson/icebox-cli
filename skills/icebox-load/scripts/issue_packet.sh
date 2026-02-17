#!/usr/bin/env bash
set -euo pipefail

STATE_ORDER=("draft" "ready-for-review" "ready-to-execute" "in-progress" "done")

required_loaded_items=(
  "Backlog mapped"
  "Spec linked"
  "Tests mapped"
  "ADR triaged"
  "Docs impact listed"
)

required_closeout_fields=(
  "PR link:"
  "Tests run"
  "Docs updated"
)

err() {
  echo "error: $*" >&2
}

need_gh() {
  if ! command -v gh >/dev/null 2>&1; then
    err "\`gh\` CLI is required but not found"
    exit 2
  fi
}

normalize_issue() {
  local issue="${1#\#}"
  echo "$issue"
}

contains_state() {
  local candidate="$1"
  for s in "${STATE_ORDER[@]}"; do
    if [[ "$s" == "$candidate" ]]; then
      return 0
    fi
  done
  return 1
}

issue_labels() {
  local issue="$1"
  gh issue view "$issue" --json labels --jq '.labels[].name'
}

current_state() {
  local issue="$1"
  local matches
  matches="$(issue_labels "$issue" | grep -E '^(draft|ready-for-review|ready-to-execute|in-progress|done)$' || true)"
  local count
  count="$(echo "$matches" | sed '/^$/d' | wc -l | tr -d ' ')"
  if [[ "$count" -gt 1 ]]; then
    err "multiple state labels found: $(echo "$matches" | paste -sd ', ' -)"
    exit 1
  fi
  if [[ "$count" -eq 0 ]]; then
    echo ""
  else
    echo "$matches" | sed -n '1p'
  fi
}

issue_body() {
  local issue="$1"
  gh issue view "$issue" --json body --jq '.body'
}

issue_comments() {
  local issue="$1"
  gh issue view "$issue" --json comments --jq '.comments[].body'
}

checklist_checked() {
  local body="$1"
  local item="$2"
  echo "$body" | grep -Eiq "^[[:space:]]*-[[:space:]]*\[[xX]\][[:space:]]*${item//\?/\\?}[[:space:]]*$"
}

has_execution_plan_comment() {
  local issue="$1"
  issue_comments "$issue" | grep -Eiq '\bExecution Plan\b'
}

is_nonempty_field() {
  local content="$1"
  local prefix="$2"
  local line
  line="$(echo "$content" | grep -Ei "^[[:space:]]*${prefix//\?/\\?}[[:space:]]*" | head -n1 || true)"
  if [[ -z "$line" ]]; then
    return 1
  fi
  local rhs
  rhs="$(echo "$line" | sed -E 's/^[^:]*:[[:space:]]*//')"
  case "${rhs,,}" in
    ""|"n/a"|"na"|"tbd"|"none"|"-")
      return 1
      ;;
    *)
      return 0
      ;;
  esac
}

adr_required() {
  local content="$1"
  echo "$content" | grep -Eiq 'ADR required\?[[:space:]]*[:\-][[:space:]]*yes'
}

state_index() {
  local target="$1"
  local i=0
  for s in "${STATE_ORDER[@]}"; do
    if [[ "$s" == "$target" ]]; then
      echo "$i"
      return 0
    fi
    i=$((i + 1))
  done
  echo "-1"
}

ensure_labels() {
  gh label create "draft" --color "8B949E" --description "Execution packet state" --force >/dev/null
  gh label create "ready-for-review" --color "1D76DB" --description "Execution packet state" --force >/dev/null
  gh label create "ready-to-execute" --color "0E8A16" --description "Execution packet state" --force >/dev/null
  gh label create "in-progress" --color "FBCA04" --description "Execution packet state" --force >/dev/null
  gh label create "done" --color "5319E7" --description "Execution packet state" --force >/dev/null
  echo "ok: ensured state labels"
}

transition() {
  local issue="$1"
  local target="$2"
  local dry_run="${3:-false}"

  if ! contains_state "$target"; then
    err "invalid target state $target"
    return 1
  fi

  local current
  current="$(current_state "$issue")"
  if [[ -z "$current" ]]; then
    err "issue has no state label"
    return 1
  fi

  local src_idx dst_idx
  src_idx="$(state_index "$current")"
  dst_idx="$(state_index "$target")"

  if [[ "$dst_idx" -ne "$src_idx" && "$dst_idx" -ne $((src_idx + 1)) ]]; then
    err "invalid transition $current -> $target"
    return 1
  fi

  if [[ "$dst_idx" -eq "$src_idx" ]]; then
    echo "ok: already in state $target"
    return 0
  fi

  local cmd=("gh" "issue" "edit" "$issue" "--add-label" "$target")
  local s
  while IFS= read -r s; do
    [[ -z "$s" || "$s" == "$target" ]] && continue
    if contains_state "$s"; then
      cmd+=("--remove-label" "$s")
    fi
  done < <(issue_labels "$issue")

  if [[ "$dry_run" == "true" ]]; then
    echo "dry-run: ${cmd[*]}"
    return 0
  fi

  "${cmd[@]}" >/dev/null
  echo "ok: transitioned #$issue $current -> $target"
}

validate_execute() {
  local issue="$1"
  local errors=()

  local state
  state="$(current_state "$issue")"
  if [[ "$state" != "ready-to-execute" ]]; then
    errors+=("missing required state label: ready-to-execute")
  fi

  local body
  body="$(issue_body "$issue")"
  local item
  for item in "${required_loaded_items[@]}"; do
    if ! checklist_checked "$body" "$item"; then
      errors+=("unchecked required checklist item: $item")
    fi
  done

  if ! has_execution_plan_comment "$issue"; then
    errors+=("missing required issue comment: Execution Plan")
  fi

  if [[ "${#errors[@]}" -gt 0 ]]; then
    echo "execute gate failed:"
    printf -- "- %s\n" "${errors[@]}"
    return 1
  fi

  echo "ok: execute gate passed for #$issue"
}

validate_closeout() {
  local issue="$1"
  local errors=()

  local content
  content="$(issue_body "$issue")"$'\n\n'"$(issue_comments "$issue" || true)"

  local f
  for f in "${required_closeout_fields[@]}"; do
    if ! is_nonempty_field "$content" "$f"; then
      errors+=("missing closeout evidence field: $f")
    fi
  done

  if adr_required "$content" && ! is_nonempty_field "$content" "ADR link:"; then
    errors+=("ADR required but ADR link is missing/empty")
  fi

  if [[ "${#errors[@]}" -gt 0 ]]; then
    echo "closeout gate failed:"
    printf -- "- %s\n" "${errors[@]}"
    return 1
  fi

  echo "ok: closeout gate passed for #$issue"
}

usage() {
  cat <<'EOF'
usage: issue_packet.sh <command> [args]

commands:
  create --backlog <id> [--title <text>] [--packet-id <id>] [--spec-path <path>]
  ensure-labels
  transition --issue <id> --to <state> [--dry-run]
  validate-execute --issue <id>
  validate-closeout --issue <id>
EOF
}

create_issue() {
  local backlog="" title="" packet_id="" spec_path=""
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --backlog) backlog="$2"; shift 2 ;;
      --title) title="$2"; shift 2 ;;
      --packet-id) packet_id="$2"; shift 2 ;;
      --spec-path) spec_path="$2"; shift 2 ;;
      *) err "unknown arg: $1"; usage; return 1 ;;
    esac
  done

  if [[ -z "$backlog" ]]; then
    err "create requires --backlog"
    usage
    return 1
  fi

  if ! echo "$backlog" | grep -Eq '^E[0-9]+(\.[0-9]+)?(-[0-9]+[a-z]?)?$'; then
    err "backlog ID must use E* format (examples: E1, E4, E7.5; optional suffixes like E1-02)"
    return 1
  fi

  if [[ -z "$packet_id" ]]; then
    packet_id="PKT-${backlog}-work-item"
  fi
  if [[ -z "$spec_path" ]]; then
    spec_path="docs/plan/spec/${packet_id}.md"
  fi
  if [[ -z "$title" ]]; then
    title="packet: ${backlog}"
  fi

  ensure_labels >/dev/null

  local body
  body="$(cat <<EOF
Backlog ID: ${backlog}
Packet ID: ${packet_id}
Spec path: ${spec_path}
Current state: draft

## Objective
- Define objective for ${backlog}.

## Scope
In scope:
- TBD
Out of scope:
- TBD

## Acceptance Criteria
- AC1 TBD
- AC2 TBD
- AC3 TBD

## Tests Mapped
Test IDs:
- TBD
Planned checks:
- TBD

ADR required?: no
ADR link:

## Definition Of Loaded (Required For ready-to-execute)
- [ ] Backlog mapped
- [ ] Spec linked
- [ ] Tests mapped
- [ ] ADR triaged
- [ ] Docs impact listed

## Execution Plan (Required Before Coding)
Commit split plan:
- TBD
Planned validation commands:
- TBD
Risk notes:
- TBD

## Closeout Evidence (Required For done)
PR link:
Tests run (commands + result):
Docs updated (paths):
ADR link:
EOF
)"

  local out
  out="$(gh issue create --title "$title" --body "$body" --label "draft")"
  echo "ok: created issue $out"
}

main() {
  need_gh
  local cmd="${1:-}"
  shift || true

  case "$cmd" in
    create)
      create_issue "$@"
      ;;
    ensure-labels)
      ensure_labels
      ;;
    transition)
      local issue="" to="" dry_run="false"
      while [[ $# -gt 0 ]]; do
        case "$1" in
          --issue) issue="$(normalize_issue "$2")"; shift 2 ;;
          --to) to="$2"; shift 2 ;;
          --dry-run) dry_run="true"; shift 1 ;;
          *) err "unknown arg: $1"; usage; return 1 ;;
        esac
      done
      [[ -z "$issue" || -z "$to" ]] && { err "transition requires --issue and --to"; usage; return 1; }
      transition "$issue" "$to" "$dry_run"
      ;;
    validate-execute)
      local issue=""
      while [[ $# -gt 0 ]]; do
        case "$1" in
          --issue) issue="$(normalize_issue "$2")"; shift 2 ;;
          *) err "unknown arg: $1"; usage; return 1 ;;
        esac
      done
      [[ -z "$issue" ]] && { err "validate-execute requires --issue"; usage; return 1; }
      validate_execute "$issue"
      ;;
    validate-closeout)
      local issue=""
      while [[ $# -gt 0 ]]; do
        case "$1" in
          --issue) issue="$(normalize_issue "$2")"; shift 2 ;;
          *) err "unknown arg: $1"; usage; return 1 ;;
        esac
      done
      [[ -z "$issue" ]] && { err "validate-closeout requires --issue"; usage; return 1; }
      validate_closeout "$issue"
      ;;
    ""|-h|--help|help)
      usage
      ;;
    *)
      err "unknown command: $cmd"
      usage
      return 1
      ;;
  esac
}

main "$@"
