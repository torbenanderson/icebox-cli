#!/usr/bin/env bash
set -euo pipefail

STATUS_PATH="docs/plan/task-status.json"
CURRENT_STATE_PATH="docs/plan/CURRENT_STATE.md"

err() {
  echo "error: $*" >&2
}

today() {
  date +%F
}

ensure_registry() {
  mkdir -p "$(dirname "$STATUS_PATH")"
  if [[ ! -f "$STATUS_PATH" ]]; then
    cat > "$STATUS_PATH" <<EOF
{
  "version": 1,
  "updated_at": "$(today)",
  "items": []
}
EOF
  fi
}

normalize_packet_id() {
  local backlog="$1"
  echo "PKT-${backlog}"
}

backlog_title() {
  local backlog="$1"
  awk -F'|' -v b="$backlog" '
    $0 ~ "^\\|[[:space:]]*" b "[[:space:]]*\\|" {
      val=$3
      gsub(/^[ \t]+|[ \t]+$/, "", val)
      print val
      exit
    }
  ' docs/plan/BACKLOG.md
}

upsert_entry() {
  local backlog="" status="" issue_number="null" issue_url="" pr_url="" spec_path=""
  local archive_path="" validated_at="" title="" validation_json="[]"
  local architecture_docs_json="[]" internal_docs_json="[]" external_docs_json="[]"

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --backlog) backlog="$2"; shift 2 ;;
      --status) status="$2"; shift 2 ;;
      --issue-number) issue_number="$2"; shift 2 ;;
      --issue-url) issue_url="$2"; shift 2 ;;
      --pr-url) pr_url="$2"; shift 2 ;;
      --spec-path) spec_path="$2"; shift 2 ;;
      --archive-path) archive_path="$2"; shift 2 ;;
      --validated-at) validated_at="$2"; shift 2 ;;
      --title) title="$2"; shift 2 ;;
      --validation-json) validation_json="$2"; shift 2 ;;
      --architecture-docs-json) architecture_docs_json="$2"; shift 2 ;;
      --internal-docs-json) internal_docs_json="$2"; shift 2 ;;
      --external-docs-json) external_docs_json="$2"; shift 2 ;;
      *) err "unknown arg: $1"; return 1 ;;
    esac
  done

  [[ -z "$backlog" || -z "$status" ]] && {
    err "upsert requires --backlog and --status"
    return 1
  }

  ensure_registry

  local packet_id
  packet_id="$(normalize_packet_id "$backlog")"
  if [[ -z "$title" ]]; then
    title="$(backlog_title "$backlog")"
  fi

  local tmp
  tmp="$(mktemp)"
  jq \
    --arg updated_at "$(today)" \
    --arg packet_id "$packet_id" \
    --arg backlog_id "$backlog" \
    --arg title "$title" \
    --arg status "$status" \
    --arg issue_url "$issue_url" \
    --arg pr_url "$pr_url" \
    --arg spec_path "$spec_path" \
    --arg archive_path "$archive_path" \
    --arg validated_at "$validated_at" \
    --argjson issue_number "$issue_number" \
    --argjson validation "$validation_json" \
    --argjson architecture_docs "$architecture_docs_json" \
    --argjson internal_docs "$internal_docs_json" \
    --argjson external_docs "$external_docs_json" \
    '
    .updated_at = $updated_at
    | .items = ((.items // []) | map(
        if .packet_id == $packet_id then
          . + {
            packet_id: $packet_id,
            backlog_id: $backlog_id,
            title: (if $title == "" then .title else $title end),
            status: $status,
            issue: {
              number: (if $issue_number == null then (.issue.number // null) else $issue_number end),
              url: (if $issue_url == "" then (.issue.url // null) else $issue_url end)
            },
            pr_url: (if $pr_url == "" then (.pr_url // null) else $pr_url end),
            spec_path: (if $spec_path == "" then (.spec_path // null) else $spec_path end),
            archive_path: (if $archive_path == "" then (.archive_path // null) else $archive_path end),
            validated_at: (if $validated_at == "" then (.validated_at // null) else $validated_at end),
            validation: (if ($validation | length) == 0 then (.validation // []) else $validation end),
            docs: {
              architecture: (if ($architecture_docs | length) == 0 then (.docs.architecture // []) else $architecture_docs end),
              internal: (if ($internal_docs | length) == 0 then (.docs.internal // []) else $internal_docs end),
              external: (if ($external_docs | length) == 0 then (.docs.external // []) else $external_docs end)
            }
          }
        else .
        end
      ))
    | if any(.items[]?; .packet_id == $packet_id) then .
      else .items += [{
        packet_id: $packet_id,
        backlog_id: $backlog_id,
        title: $title,
        status: $status,
        issue: {
          number: $issue_number,
          url: (if $issue_url == "" then null else $issue_url end)
        },
        pr_url: (if $pr_url == "" then null else $pr_url end),
        spec_path: (if $spec_path == "" then null else $spec_path end),
        archive_path: (if $archive_path == "" then null else $archive_path end),
        validated_at: (if $validated_at == "" then null else $validated_at end),
        validation: $validation,
        docs: {
          architecture: $architecture_docs,
          internal: $internal_docs,
          external: $external_docs
        }
      }]
      end
    | .items |= sort_by(.backlog_id)
    ' "$STATUS_PATH" > "$tmp"
  mv "$tmp" "$STATUS_PATH"
}

entry_status_for_backlog() {
  local backlog="$1"
  jq -r --arg backlog "$backlog" '
    (.items // []) | map(select(.backlog_id == $backlog)) | first | .status // ""
  ' "$STATUS_PATH"
}

sync_from_specs() {
  ensure_registry

  local path backlog archive_path existing_status desired_status

  while IFS= read -r path; do
    [[ -z "$path" ]] && continue
    backlog="$(basename "$path" | sed -E 's/^PKT-([A-Z0-9.-]+)-work-item\.md$/\1/')"
    existing_status="$(entry_status_for_backlog "$backlog")"
    desired_status="${existing_status:-planned}"
    [[ "$desired_status" == "done" ]] && desired_status="planned"
    upsert_entry --backlog "$backlog" --status "$desired_status" --spec-path "$path"
  done < <(find docs/plan/spec -maxdepth 1 -type f -name 'PKT-*.md' | sort)

  while IFS= read -r path; do
    [[ -z "$path" ]] && continue
    backlog="$(basename "$path" | sed -E 's/^PKT-([A-Z0-9.-]+)-work-item\.md$/\1/')"
    archive_path="$path"
    upsert_entry \
      --backlog "$backlog" \
      --status "done" \
      --spec-path "$path" \
      --archive-path "$archive_path"
  done < <(find docs/plan/spec/archive -type f -name 'PKT-*.md' | sort)
}

render_slice_row() {
  local name="$1"
  local definition="$2"
  shift 2
  local -a backlog_ids=("$@")
  local done_count=0
  local total_count="${#backlog_ids[@]}"
  local -a pending=()
  local backlog status

  for backlog in "${backlog_ids[@]}"; do
    status="$(entry_status_for_backlog "$backlog")"
    if [[ "$status" == "done" ]]; then
      done_count=$((done_count + 1))
    else
      pending+=("$backlog")
    fi
  done

  local state="incomplete"
  local notes
  if [[ "$done_count" -eq "$total_count" ]]; then
    state="complete"
    notes="all packet-backed items in this slice are done"
  else
    notes="pending: $(printf "%s, " "${pending[@]}" | sed 's/, $//')"
  fi

  printf '| %s | %s | %s (%s/%s) | %s |\n' "$name" "$definition" "$state" "$done_count" "$total_count" "$notes"
}

render_current_state() {
  ensure_registry
  sync_from_specs

  local done_count active_count planned_count in_progress_count implemented_count
  done_count="$(jq '[.items[] | select(.status == "done")] | length' "$STATUS_PATH")"
  active_count="$(jq '[.items[] | select(.status != "done")] | length' "$STATUS_PATH")"
  planned_count="$(jq '[.items[] | select(.status == "planned")] | length' "$STATUS_PATH")"
  in_progress_count="$(jq '[.items[] | select(.status == "in_progress")] | length' "$STATUS_PATH")"
  implemented_count="$(jq '[.items[] | select(.status == "implemented")] | length' "$STATUS_PATH")"

  local tmp
  tmp="$(mktemp)"
  {
    cat <<EOF
# Current State

This page is the human-readable project status snapshot.

## Source Of Truth

- Packet lifecycle source of truth: [task-status.json](task-status.json)
- Scope and acceptance criteria source of truth: [BACKLOG.md](BACKLOG.md) plus packet specs under \`docs/plan/spec/\`
- Closeout authority for \`done\`: \`skills/icebox-load/scripts/issue_packet.sh done\`
- Architecture impact must be explicitly accounted for during closeout, even when the result is \`none (not impacted)\`

## Snapshot

- Registry updated: $(jq -r '.updated_at' "$STATUS_PATH")
- Done packets: ${done_count}
- Active packets: ${active_count}
- Planned packets: ${planned_count}
- In-progress packets: ${in_progress_count}
- Implemented-not-closed packets: ${implemented_count}

## Slice Status

| Slice | Definition | Status | Notes |
|---|---|---|---|
EOF
    render_slice_row \
      "Vault Core Slice" \
      "E3 packet-backed vault foundation slice" \
      "E3-21" "E3-01" "E3-02" "E3-05" "E3-10" "E3-11" "E3-12"
    render_slice_row \
      "MVP Runnable Slice" \
      "\`register-agent -> add -> run -> remove\` path tracked in planning" \
      "E2-01" "E2-02" "E2-03" "E2-04" "E2-11" "E2-09" "E2-18" "E3-21" "E3-01" "E3-02" "E3-05" "E3-10" "E3-11" "E3-12" "E4-01" "E4-02" "E4-04" "E4-06" "E5-01" "E5-02" "E5-03" "E5-05" "E5-06" "E5-07" "E5-10"
    cat <<EOF

## Packet Summary By Epic

| Epic | Done | Planned | In Progress | Implemented |
|---|---|---|---|---|
EOF
    local epic epic_done epic_planned epic_in_progress epic_implemented
    for epic in E1 E2 E3 E4 E5 E6 E7 E7.5 E8 E9 E10; do
      epic_done="$(jq -r --arg epic "$epic" '[.items[] | select((.backlog_id | startswith($epic + "-")) and .status == "done")] | length' "$STATUS_PATH")"
      epic_planned="$(jq -r --arg epic "$epic" '[.items[] | select((.backlog_id | startswith($epic + "-")) and .status == "planned")] | length' "$STATUS_PATH")"
      epic_in_progress="$(jq -r --arg epic "$epic" '[.items[] | select((.backlog_id | startswith($epic + "-")) and .status == "in_progress")] | length' "$STATUS_PATH")"
      epic_implemented="$(jq -r --arg epic "$epic" '[.items[] | select((.backlog_id | startswith($epic + "-")) and .status == "implemented")] | length' "$STATUS_PATH")"
      if [[ "$epic_done" -gt 0 || "$epic_planned" -gt 0 || "$epic_in_progress" -gt 0 || "$epic_implemented" -gt 0 ]]; then
        printf '| %s | %s | %s | %s | %s |\n' "$epic" "$epic_done" "$epic_planned" "$epic_in_progress" "$epic_implemented"
      fi
    done
    cat <<EOF

## Done Packets

EOF
    jq -r '
      [.items[] | select(.status == "done")] | sort_by(.backlog_id)[] |
      "- \(.backlog_id) -- \(.title) (\(.archive_path // .spec_path // "no spec path"))"
    ' "$STATUS_PATH"
    cat <<EOF

## Active Packets

EOF
    local active_lines
    active_lines="$(jq -r '
      [.items[] | select(.status != "done")] | sort_by(.backlog_id)[] |
      "- \(.backlog_id) -- \(.title) [\(.status)] (\(.spec_path // "no spec path"))"
    ' "$STATUS_PATH")"
    if [[ -n "$active_lines" ]]; then
      printf "%s\n" "$active_lines"
    else
      echo "- none"
    fi
    cat <<EOF

---
*Last updated: $(today)*
EOF
  } > "$tmp"
  mv "$tmp" "$CURRENT_STATE_PATH"
}

usage() {
  cat <<'EOF'
usage: skills/icebox-load/scripts/plan_status.sh <command> [args]

commands:
  ensure
  sync
  render
  sync-render
  upsert --backlog <id> --status <planned|in_progress|implemented|done> [--issue-number <n>] [--issue-url <url>] [--pr-url <url>] [--spec-path <path>] [--archive-path <path>] [--validated-at <date>] [--title <text>] [--validation-json <json>] [--architecture-docs-json <json>] [--internal-docs-json <json>] [--external-docs-json <json>]
EOF
}

main() {
  local cmd="${1:-}"
  shift || true

  case "$cmd" in
    ensure)
      ensure_registry
      ;;
    sync)
      sync_from_specs
      ;;
    render)
      render_current_state
      ;;
    sync-render)
      sync_from_specs
      render_current_state
      ;;
    upsert)
      upsert_entry "$@"
      ;;
    *)
      usage
      exit 1
      ;;
  esac
}

main "$@"
