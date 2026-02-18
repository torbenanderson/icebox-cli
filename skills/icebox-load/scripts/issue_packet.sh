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
  "Files added/changed (paths):"
)

err() {
  echo "error: $*" >&2
}

gh_try() {
  local attempt=1
  local max_attempts="${GH_RETRY_MAX_ATTEMPTS:-4}"
  local delay_seconds="${GH_RETRY_BASE_DELAY_SECONDS:-1}"
  local rc=0

  while true; do
    if command gh "$@"; then
      return 0
    fi
    rc=$?
    if (( attempt >= max_attempts )); then
      return "$rc"
    fi
    sleep "$delay_seconds"
    delay_seconds=$((delay_seconds * 2))
    attempt=$((attempt + 1))
  done
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

regex_escape() {
  echo "$1" | sed -e 's/[][(){}.^$*+?|\\/]/\\&/g'
}

issue_labels() {
  local issue="$1"
  gh_try issue view "$issue" --json labels --jq '.labels[].name'
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
  gh_try issue view "$issue" --json body --jq '.body'
}

issue_comments() {
  local issue="$1"
  gh_try issue view "$issue" --json comments --jq '.comments[].body'
}

epic_code_from_backlog() {
  local backlog="$1"
  if [[ "$backlog" == *-* ]]; then
    echo "${backlog%%-*}"
  else
    echo "$backlog"
  fi
}

roadmap_epic_name() {
  local epic="$1"
  awk -F'|' -v e="$epic" '
    $0 ~ "\\| \\*\\*" e "\\*\\* \\|" {
      name=$3
      gsub(/^[ \t]+|[ \t]+$/, "", name)
      print name
      exit
    }
  ' docs/plan/ROADMAP.md
}

find_matching_milestone() {
  local epic="$1"
  local epic_name="$2"
  local titles
  titles="$(gh_try api repos/{owner}/{repo}/milestones --paginate --jq '.[].title' 2>/dev/null || true)"
  [[ -z "$titles" ]] && return 1

  local t
  while IFS= read -r t; do
    [[ -z "$t" ]] && continue
    if [[ "$t" == "$epic" || "$t" == "$epic - $epic_name" || "$t" == "$epic: $epic_name" ]]; then
      echo "$t"
      return 0
    fi
  done <<< "$titles"

  while IFS= read -r t; do
    [[ -z "$t" ]] && continue
    if echo "$t" | grep -Eq "(^|[[:space:]])${epic}([[:space:]]|:|-|$)"; then
      echo "$t"
      return 0
    fi
  done <<< "$titles"

  while IFS= read -r t; do
    [[ -z "$t" || -z "$epic_name" ]] && continue
    if echo "$t" | grep -Fqi "$epic_name"; then
      echo "$t"
      return 0
    fi
  done <<< "$titles"

  return 1
}

ensure_milestone() {
  local epic="$1"
  local epic_name="$2"
  local wanted="$epic"
  [[ -n "$epic_name" ]] && wanted="${epic} - ${epic_name}"
  local found
  found="$(find_matching_milestone "$epic" "$epic_name" || true)"
  if [[ -n "$found" ]]; then
    echo "$found"
    return 0
  fi
  gh_try api repos/{owner}/{repo}/milestones -X POST -f title="$wanted" >/dev/null
  echo "$wanted"
}

repo_owner() {
  gh_try repo view --json owner --jq '.owner.login'
}

repo_name_with_owner() {
  gh_try repo view --json nameWithOwner --jq '.nameWithOwner'
}

find_project_by_title() {
  local owner="$1"
  local title="$2"
  gh_try project list --owner "$owner" --format json --jq '.projects[] | select(.title=="'"$title"'") | .title' 2>/dev/null | head -n1
}

ensure_project() {
  local owner="$1"
  local title="$2"
  local repo_full="$3"
  local existing
  existing="$(find_project_by_title "$owner" "$title" || true)"
  if [[ -n "$existing" ]]; then
    if [[ -n "$repo_full" ]]; then
      gh_try project link "$(gh_try project list --owner "$owner" --format json --jq '.projects[] | select(.title=="'"$title"'") | .number' | head -n1)" --owner "$owner" --repo "$repo_full" >/dev/null 2>&1 || true
    fi
    echo "$existing"
    return 0
  fi
  gh_try project create --owner "$owner" --title "$title" >/dev/null 2>&1 || return 1
  local created
  created="$(find_project_by_title "$owner" "$title" || true)"
  if [[ -n "$repo_full" ]]; then
    gh_try project link "$(gh_try project list --owner "$owner" --format json --jq '.projects[] | select(.title=="'"$title"'") | .number' | head -n1)" --owner "$owner" --repo "$repo_full" >/dev/null 2>&1 || true
  fi
  echo "$created"
}

project_number_by_title() {
  local owner="$1"
  local title="$2"
  gh_try project list --owner "$owner" --format json --jq '.projects[] | select(.title=="'"$title"'") | .number' 2>/dev/null | head -n1
}

attach_issue_to_project_v2() {
  local owner="$1"
  local title="$2"
  local issue_url="$3"
  local pnum
  pnum="$(project_number_by_title "$owner" "$title" || true)"
  [[ -z "$pnum" ]] && return 1
  gh_try project item-add "$pnum" --owner "$owner" --url "$issue_url" >/dev/null
}

checklist_checked() {
  local body="$1"
  local item="$2"
  local item_re
  item_re="$(regex_escape "$item")"
  echo "$body" | grep -Eiq "^[[:space:]]*-[[:space:]]*\[[xX]\][[:space:]]*${item_re}[[:space:]]*$"
}

has_execution_plan_comment() {
  local issue="$1"
  issue_comments "$issue" | grep -Eiq '\bExecution Plan\b'
}

is_nonempty_field() {
  local content="$1"
  local prefix="$2"
  local prefix_re
  prefix_re="$(regex_escape "$prefix")"
  local line
  line="$(echo "$content" | grep -Ei "^[[:space:]]*${prefix_re}[[:space:]]*" | head -n1 || true)"
  if [[ -z "$line" ]]; then
    return 1
  fi
  local rhs
  rhs="$(echo "$line" | sed -E 's/^[^:]*:[[:space:]]*//')"
  local rhs_lower
  rhs_lower="$(echo "$rhs" | tr '[:upper:]' '[:lower:]')"
  case "$rhs_lower" in
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
  gh_try label create "draft" --color "8B949E" --description "Execution packet state" --force >/dev/null
  gh_try label create "ready-for-review" --color "1D76DB" --description "Execution packet state" --force >/dev/null
  gh_try label create "ready-to-execute" --color "0E8A16" --description "Execution packet state" --force >/dev/null
  gh_try label create "in-progress" --color "FBCA04" --description "Execution packet state" --force >/dev/null
  gh_try label create "done" --color "5319E7" --description "Execution packet state" --force >/dev/null
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

  local cmd=("gh_try" "issue" "edit" "$issue" "--add-label" "$target")
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

to_https_repo_url() {
  local remote="$1"
  if echo "$remote" | grep -q '^git@github.com:'; then
    echo "$remote" | sed -E 's#^git@github.com:#https://github.com/#; s#\.git$##'
    return 0
  fi
  if echo "$remote" | grep -q '^https://github.com/'; then
    echo "$remote" | sed -E 's#\.git$##'
    return 0
  fi
  echo "$remote"
}

default_pr_link() {
  local remote repo_url head
  remote="$(git remote get-url origin 2>/dev/null || true)"
  head="$(git rev-parse HEAD 2>/dev/null || true)"
  repo_url="$(to_https_repo_url "$remote")"
  if [[ -n "$repo_url" && -n "$head" ]]; then
    echo "${repo_url}/commit/${head}"
    return 0
  fi
  echo "https://github.com/<owner>/<repo>/commit/<hash>"
}

issue_backlog_id() {
  local issue="$1"
  issue_body "$issue" | sed -n 's/^Backlog ID:[[:space:]]*//p' | head -n1
}

current_branch() {
  git rev-parse --abbrev-ref HEAD 2>/dev/null || true
}

sanitize_branch_component() {
  echo "$1" | tr '[:upper:]' '[:lower:]' | sed -E 's/[^a-z0-9._-]+/-/g'
}

ensure_packet_branch() {
  local issue="$1"
  local backlog epic branch target
  branch="$(current_branch)"
  if [[ "$branch" != "main" && "$branch" != "master" ]]; then
    echo "$branch"
    return 0
  fi

  backlog="$(issue_backlog_id "$issue")"
  [[ -z "$backlog" ]] && backlog="issue-${issue}"
  epic="$(epic_code_from_backlog "$backlog")"
  [[ -z "$epic" ]] && epic="$backlog"
  target="pkt/$(sanitize_branch_component "$epic")"

  if git rev-parse --verify "$target" >/dev/null 2>&1; then
    git checkout "$target" >/dev/null
  else
    git checkout -b "$target" >/dev/null
  fi
  echo "$target"
}

ensure_branch_pushed() {
  local branch="$1"
  if git rev-parse --abbrev-ref --symbolic-full-name '@{u}' >/dev/null 2>&1; then
    git push >/dev/null
  else
    git push -u origin "$branch" >/dev/null
  fi
}

branch_has_commits_ahead_main() {
  local branch="$1"
  local count
  count="$(git rev-list --count "origin/main..${branch}" 2>/dev/null || echo 0)"
  [[ "${count:-0}" -gt 0 ]]
}

current_branch_pr_url() {
  gh_try pr view --json url --jq '.url' 2>/dev/null || true
}

create_epic_pr_for_issue() {
  local issue="$1"
  local branch backlog epic epic_name title body out url
  branch="$(current_branch)"
  backlog="$(issue_backlog_id "$issue")"
  epic="$(epic_code_from_backlog "$backlog")"
  epic_name="$(roadmap_epic_name "$epic" || true)"
  if [[ -n "$epic_name" ]]; then
    title="[PR] Epic ${epic}: ${epic_name}"
  else
    title="[PR] Epic ${epic}: implementation"
  fi
  body="Epic: ${epic}
Refs #${issue}
Related issues:
- #${issue}

This PR is maintained at epic level and can contain multiple issue-level commits."
  out="$(gh_try pr create --draft --base main --head "$branch" --title "$title" --body "$body")"
  url="$(echo "$out" | grep -Eo 'https://github.com/[^ ]+/pull/[0-9]+' | tail -n1 || true)"
  [[ -n "$url" ]] && echo "$url" || echo "$out"
}

ensure_pr_references_issue() {
  local pr_ref="$1"
  local issue="$2"
  local body number tmp marker
  marker="Refs #${issue}"

  body="$(gh_try pr view "$pr_ref" --json body --jq '.body' 2>/dev/null || true)"
  [[ -z "$body" ]] && return 0
  if echo "$body" | grep -Eq "(^|[[:space:]])#${issue}([[:space:]]|$)"; then
    return 0
  fi

  number="$(gh_try pr view "$pr_ref" --json number --jq '.number' 2>/dev/null || true)"
  [[ -z "$number" ]] && return 0
  tmp="$(mktemp)"
  {
    printf "%s\n\n" "$body"
    printf "%s\n" "$marker"
  } > "$tmp"
  gh_try pr edit "$number" --body-file "$tmp" >/dev/null 2>&1 || true
  rm -f "$tmp"
}

ensure_pr_link_for_issue() {
  local issue="$1"
  local branch pr_url
  branch="$(ensure_packet_branch "$issue")"
  if [[ -z "$branch" ]]; then
    err "unable to determine current git branch"
    return 1
  fi

  ensure_branch_pushed "$branch"
  if ! branch_has_commits_ahead_main "$branch"; then
    err "branch ${branch} has no commits ahead of origin/main; commit your changes before done"
    return 1
  fi
  pr_url="$(current_branch_pr_url)"
  if [[ -n "$pr_url" ]]; then
    ensure_pr_references_issue "$pr_url" "$issue"
    echo "$pr_url"
    return 0
  fi

  pr_url="$(create_epic_pr_for_issue "$issue" || true)"
  if [[ -z "$pr_url" ]]; then
    err "failed to create epic-level PR for branch ${branch}"
    return 1
  fi
  ensure_pr_references_issue "$pr_url" "$issue"
  echo "$pr_url"
}

changed_paths() {
  {
    # Include local working-tree deltas.
    git diff --name-only 2>/dev/null || true
    git diff --cached --name-only 2>/dev/null || true
    git ls-files --others --exclude-standard 2>/dev/null || true

    # Include committed branch deltas so closeout evidence still reports files
    # when the branch is clean/already pushed.
    if git rev-parse --verify origin/main >/dev/null 2>&1; then
      git diff --name-only origin/main...HEAD 2>/dev/null || true
    fi
    if git rev-parse --abbrev-ref --symbolic-full-name '@{u}' >/dev/null 2>&1; then
      git diff --name-only '@{u}'...HEAD 2>/dev/null || true
    fi
  } | awk 'NF{print}' | sort -u
}

run_default_closeout_tests() {
  local tests_report=""
  local failed=0
  local cmd
  local cmds=(
    "cargo check"
    "cargo fmt --check"
    "cargo clippy -- -D warnings"
    "cargo test"
  )
  for cmd in "${cmds[@]}"; do
    if sh -c "$cmd" >/tmp/icebox-closeout.$$ 2>&1; then
      tests_report="${tests_report}- ${cmd} (pass)\n"
    else
      tests_report="${tests_report}- ${cmd} (fail)\n"
      failed=1
    fi
  done
  rm -f /tmp/icebox-closeout.$$ >/dev/null 2>&1 || true
  printf "%b" "$tests_report"
  return "$failed"
}

post_closeout_comment() {
  local issue="$1"
  local pr_link="$2"
  local tests_run="$3"
  local docs_paths="$4"
  local files_paths="$5"
  local adr_link="$6"
  local excluded_paths="${7:-}"

  local tmp
  tmp="$(mktemp)"
  cat > "$tmp" <<EOF
## Closeout Evidence

### References
PR link: ${pr_link}
ADR link: ${adr_link}

### Verification
Tests run (commands + result):
${tests_run}

### Artifact Deltas
Docs updated (paths):
${docs_paths}
Files added/changed (paths):
${files_paths}
EOF
  if [[ -n "$excluded_paths" ]]; then
    cat >> "$tmp" <<EOF
Excluded as out-of-packet scope:
${excluded_paths}
EOF
  fi

  gh_try issue comment "$issue" --body-file "$tmp" >/dev/null
  rm -f "$tmp"
}

sync_closeout_fields_to_body() {
  local issue="$1"
  local pr_link="$2"
  local tests_summary="$3"
  local docs_summary="$4"
  local files_summary="$5"
  local adr_link="$6"

  local body tmp
  body="$(issue_body "$issue")"
  tmp="$(mktemp)"
  echo "$body" | awk \
    -v pr="PR link: ${pr_link}" \
    -v tests="Tests run (commands + result): ${tests_summary}" \
    -v docs="Docs updated (paths): ${docs_summary}" \
    -v files="Files added/changed (paths): ${files_summary}" \
    -v adr="ADR link: ${adr_link}" '
    BEGIN {
      done_pr=0; done_tests=0; done_docs=0; done_files=0; done_adr=0;
    }
    /^PR link:[[:space:]]*/ { print pr; done_pr=1; next }
    /^Tests run \(commands \+ result\):[[:space:]]*/ { print tests; done_tests=1; next }
    /^Docs updated \(paths\):[[:space:]]*/ { print docs; done_docs=1; next }
    /^Files added\/changed \(paths\):[[:space:]]*/ { print files; done_files=1; next }
    /^ADR link:[[:space:]]*/ { print adr; done_adr=1; next }
    { print }
    END {
      if (!done_pr) print pr
      if (!done_tests) print tests
      if (!done_docs) print docs
      if (!done_files) print files
      if (!done_adr) print adr
    }' > "$tmp"
  gh_try issue edit "$issue" --body-file "$tmp" >/dev/null
  rm -f "$tmp"
}

closeout_issue() {
  local issue="" pr_link="" adr_link="" run_tests="yes"
  local -a override_docs=()
  local -a override_files=()
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --issue) issue="$(normalize_issue "$2")"; shift 2 ;;
      --pr-link) pr_link="$2"; shift 2 ;;
      --adr-link) adr_link="$2"; shift 2 ;;
      --skip-tests) run_tests="no"; shift 1 ;;
      --doc-path) override_docs+=("$2"); shift 2 ;;
      --file-path) override_files+=("$2"); shift 2 ;;
      *) err "unknown arg: $1"; usage; return 1 ;;
    esac
  done

  [[ -z "$issue" ]] && { err "closeout requires --issue"; usage; return 1; }
  if [[ -z "$pr_link" ]]; then
    pr_link="$(ensure_pr_link_for_issue "$issue")" || return 1
  fi
  [[ -z "$adr_link" ]] && adr_link="n/a"

  local tests_run
  local tests_failed=0
  if [[ "$run_tests" == "yes" ]]; then
    tests_run="$(run_default_closeout_tests)" || tests_failed=1
  else
    tests_run="- skipped (manual override: --skip-tests)"
  fi

  local paths docs_paths files_paths excluded_paths
  paths="$(changed_paths)"
  docs_paths="$(echo "$paths" | grep -E '^docs/' || true)"
  files_paths="$paths"

  if [[ "${#override_docs[@]}" -gt 0 ]]; then
    docs_paths="$(printf "%s\n" "${override_docs[@]}" | awk 'NF' | sort -u)"
  fi
  if [[ "${#override_files[@]}" -gt 0 ]]; then
    files_paths="$(printf "%s\n" "${override_files[@]}" | awk 'NF' | sort -u)"
    excluded_paths="$(comm -23 <(echo "$paths" | awk 'NF' | sort -u) <(echo "$files_paths" | awk 'NF' | sort -u) || true)"
  fi

  [[ -z "$docs_paths" ]] && docs_paths="- none"
  [[ -z "$files_paths" ]] && files_paths="- none"
  [[ -n "$excluded_paths" ]] && excluded_paths="$(echo "$excluded_paths" | sed 's/^/- /')"
  docs_paths="$(echo "$docs_paths" | sed 's/^/- /')"
  files_paths="$(echo "$files_paths" | sed 's/^/- /')"

  post_closeout_comment "$issue" "$pr_link" "$tests_run" "$docs_paths" "$files_paths" "$adr_link" "$excluded_paths"
  sync_closeout_fields_to_body \
    "$issue" \
    "$pr_link" \
    "see latest Closeout Evidence comment" \
    "see latest Closeout Evidence comment" \
    "see latest Closeout Evidence comment" \
    "$adr_link"
  if [[ "$tests_failed" -ne 0 ]]; then
    err "one or more closeout validation commands failed; evidence comment posted but done transition blocked"
    return 1
  fi
  echo "ok: closeout evidence comment posted for #$issue"
}

done_issue() {
  local issue="" pr_link="" adr_link="" run_tests="yes"
  local -a passthrough_args=()
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --issue) issue="$(normalize_issue "$2")"; shift 2 ;;
      --pr-link) pr_link="$2"; passthrough_args+=("--pr-link" "$2"); shift 2 ;;
      --adr-link) adr_link="$2"; passthrough_args+=("--adr-link" "$2"); shift 2 ;;
      --skip-tests) run_tests="no"; passthrough_args+=("--skip-tests"); shift 1 ;;
      --doc-path|--file-path) passthrough_args+=("$1" "$2"); shift 2 ;;
      *) err "unknown arg: $1"; usage; return 1 ;;
    esac
  done
  [[ -z "$issue" ]] && { err "done requires --issue"; usage; return 1; }

  if [[ "${#passthrough_args[@]}" -gt 0 ]]; then
    closeout_issue --issue "$issue" "${passthrough_args[@]}"
  else
    closeout_issue --issue "$issue"
  fi
  validate_closeout "$issue"
  transition "$issue" "done" "false"
}

usage() {
  cat <<'EOF'
usage: issue_packet.sh <command> [args]

commands:
  create --backlog <id> [--title <text>] [--packet-id <id>] [--spec-path <path>]
  load --issue <id> --backlog <id> [--adr-required yes|no] [--spec-path <path>]
  ensure-labels
  transition --issue <id> --to <state> [--dry-run]
  validate-execute --issue <id>
  closeout --issue <id> [--pr-link <url>] [--adr-link <path-or-url>] [--skip-tests] [--doc-path <path>]... [--file-path <path>]...
  validate-closeout --issue <id>
  done --issue <id> [--pr-link <url>] [--adr-link <path-or-url>] [--skip-tests] [--doc-path <path>]... [--file-path <path>]...
EOF
}

validate_backlog_id() {
  local backlog="$1"
  if ! echo "$backlog" | grep -Eq '^E[0-9]+(\.[0-9]+)?(-[0-9]+[a-z]?)?$'; then
    err "backlog ID must use E* format (examples: E1, E4, E7.5; optional suffixes like E1-02)"
    return 1
  fi
}

backlog_exists() {
  local backlog="$1"
  rg -q "^\|[[:space:]]*${backlog//./\\.}[[:space:]]*\|" docs/plan/BACKLOG.md
}

mapped_tests_for_backlog() {
  local backlog="$1"
  awk -F'|' -v b="$backlog" '
    $0 ~ "\\|[[:space:]]*" b "[[:space:]]*\\|" {
      gsub(/^[ \t]+|[ \t]+$/, "", $2);
      if ($2 != "") print $2;
    }
  ' docs/plan/TESTING.md
}

backlog_use_case() {
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

backlog_description() {
  local backlog="$1"
  awk -F'|' -v b="$backlog" '
    $0 ~ "^\\|[[:space:]]*" b "[[:space:]]*\\|" {
      val=$4
      gsub(/^[ \t]+|[ \t]+$/, "", val)
      print val
      exit
    }
  ' docs/plan/BACKLOG.md
}

is_scaffold_only_backlog() {
  local backlog="$1"
  local use_case="$2"
  local desc="$3"
  if [[ "$backlog" == "E1-01" ]]; then
    return 0
  fi
  local combined
  combined="$(printf "%s %s" "$use_case" "$desc" | tr '[:upper:]' '[:lower:]')"
  if echo "$combined" | grep -Eq 'cargo init|scaffold|bootstrap'; then
    return 0
  fi
  return 1
}

default_execution_plan() {
  local backlog="$1"
  local spec_path="$2"
  local scaffold_only="$3"
  local tests_present="$4"
  local commit_split validation risks

  if [[ "$scaffold_only" == "true" ]]; then
    commit_split="- commit 1: scaffold setup for ${backlog} (\`cargo init\`, crate metadata sanity)
- commit 2: baseline validation/docs sync (spec + testing mapping updates)"
    validation="- cargo check
- cargo fmt --check
- cargo clippy -- -D warnings"
    risks="- risk: accidental scope creep into CLI behavior before E1-02
- mitigation: limit changes to scaffold artifacts and planning docs"
  else
    commit_split="- commit 1: core implementation for ${backlog}
- commit 2: tests and docs alignment updates"
    validation="- cargo fmt --check
- cargo clippy -- -D warnings
- cargo test"
    if [[ "$tests_present" == "true" ]]; then
      risks="- risk: implementation diverges from mapped tests
- mitigation: run mapped tests before closeout and update spec if contract changes"
    else
      risks="- risk: no mapped tests currently in docs/plan/TESTING.md for ${backlog}
- mitigation: add or link test mapping before ready-to-execute"
    fi
  fi

  cat <<EOF
${commit_split}
@@VALIDATION@@
${validation}
@@RISKS@@
${risks}
EOF
}

spec_needs_refresh() {
  local spec_path="$1"
  [[ ! -f "$spec_path" ]] && return 0
  if rg -q "^- AC1:$|^- AC2:$|^- AC3:$|Define and deliver .*\\.$" "$spec_path"; then
    return 0
  fi
  if rg -q "AC1: .*implemented per backlog description|docs/SUMMARY\\.md|docs/plan/BACKLOG\\.md" "$spec_path"; then
    return 0
  fi
  if ! rg -q "^## Rust Implementation Plan" "$spec_path"; then
    return 0
  fi
  return 1
}

upsert_spec() {
  local backlog="$1"
  local spec_path="$2"
  local use_case="$3"
  local desc="$4"
  local tests_block="$5"
  local adr_required="$6"
  local docs_block="$7"
  local ac1="$8"
  local ac2="$9"
  mkdir -p "$(dirname "$spec_path")"
  if spec_needs_refresh "$spec_path"; then
    cat > "$spec_path" <<EOF
# ${backlog} Execution Spec

## Objective

- Deliver ${backlog} (${use_case}).
- Backlog contract: ${desc}

## Problem

- Why this exists: implement the backlog contract in a way that is testable, deterministic, and easy to extend.

## Scope

- In scope:
  - ${desc}
- Out of scope:
  - Unrelated backlog items outside ${backlog}
  - Cross-epic behavior changes not requested by ${backlog}

## Acceptance Criteria

- AC1: ${ac1}
- AC2: ${ac2}
- AC3: Changes are validated with mapped tests.

## Rust Implementation Plan

- Crate/module touch points:
  - \`src/main.rs\` (CLI wiring) and focused domain module(s) only.
- Keep interfaces explicit:
  - prefer small pure functions for parsing/validation paths.
  - avoid hidden global state.
- Error handling:
  - return \`Result<T, E>\` from fallible logic.
  - avoid \`unwrap()\` / \`expect()\` in non-test code paths.
- I/O behavior:
  - perform atomic/checked writes where files are modified.
  - keep side effects localized and observable.

## Security/Runtime Notes

- Keep secret-handling boundaries unchanged unless explicitly in scope.
- Preserve direct-exec/no-shell guarantees where relevant.
- Preserve user-safe default errors (no sensitive internals in normal mode).

## Test Mapping

- Linked tests from \`docs/plan/TESTING.md\`:
${tests_block}
- Add at least:
  - one happy-path test
  - one failure-path test

## ADR Triage

- ADR required? (${adr_required}):
- Rationale: keep in spec unless long-lived cross-feature decision exists.

## Docs Impact

${docs_block}

## Validation Commands

- \`cargo fmt --check\`
- \`cargo clippy -- -D warnings\`
- \`cargo test\`

## Execution Notes

- Commit split plan will be finalized in the issue \`Execution Plan\` comment during \`execute\`.
EOF
  fi
}

load_issue() {
  local issue="" backlog="" adr_required="no" spec_path=""
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --issue) issue="$(normalize_issue "$2")"; shift 2 ;;
      --backlog) backlog="$2"; shift 2 ;;
      --adr-required) adr_required="$(echo "$2" | tr '[:upper:]' '[:lower:]')"; shift 2 ;;
      --spec-path) spec_path="$2"; shift 2 ;;
      *) err "unknown arg: $1"; usage; return 1 ;;
    esac
  done

  [[ -z "$issue" || -z "$backlog" ]] && { err "load requires --issue and --backlog"; usage; return 1; }
  validate_backlog_id "$backlog" || return 1
  [[ "$adr_required" != "yes" && "$adr_required" != "no" ]] && { err "--adr-required must be yes or no"; return 1; }

  if ! backlog_exists "$backlog"; then
    err "backlog item not found in docs/plan/BACKLOG.md: $backlog"
    return 1
  fi

  local packet_id="PKT-${backlog}-work-item"
  if [[ -z "$spec_path" ]]; then
    spec_path="docs/plan/spec/${packet_id}.md"
  fi
  local use_case desc
  use_case="$(backlog_use_case "$backlog" || true)"
  desc="$(backlog_description "$backlog" || true)"
  [[ -z "$use_case" ]] && use_case="Work item"
  [[ -z "$desc" ]] && desc="See backlog entry for details."

  local tests
  tests="$(mapped_tests_for_backlog "$backlog" || true)"
  local scaffold_only="false"
  if is_scaffold_only_backlog "$backlog" "$use_case" "$desc"; then
    scaffold_only="true"
  fi

  local backlog_mark="x"
  local spec_mark="x"
  local tests_mark=" "
  local adr_mark="x"
  local docs_mark="x"
  [[ -n "$tests" ]] && tests_mark="x"

  local tests_block tests_block_md
  if [[ -n "$tests" ]]; then
    tests_block="$(echo "$tests" | sed 's/^/- /')"
    tests_block_md="$(echo "$tests" | sed 's/^/- /')"
  elif [[ "$scaffold_only" == "true" ]]; then
    tests_mark="x"
    tests_block="- scaffold-only validation mapping:
- verify \`Cargo.toml\` exists and package name is \`icebox-cli\`
- verify \`src/main.rs\` exists
- run \`cargo check\`"
    tests_block_md="- Scaffold-only validation mapping (no runtime feature code in scope):
  - verify \`Cargo.toml\` exists and package name is \`icebox-cli\`
  - verify \`src/main.rs\` exists
  - run \`cargo check\`"
  else
    tests_block="- none found yet in docs/plan/TESTING.md for ${backlog}"
    tests_block_md="- none mapped yet for ${backlog}"
  fi

  local has_tests="false"
  [[ -n "$tests" ]] && has_tests="true"
  local plan_blob execution_commit_plan execution_validation execution_risks
  plan_blob="$(default_execution_plan "$backlog" "$spec_path" "$scaffold_only" "$has_tests")"
  execution_commit_plan="$(echo "$plan_blob" | awk '/^@@VALIDATION@@$/{exit} {print}')"
  execution_validation="$(echo "$plan_blob" | awk 'seen&&$0!~/^@@RISKS@@$/{print} /^@@VALIDATION@@$/{seen=1} /^@@RISKS@@$/{exit}')"
  execution_risks="$(echo "$plan_blob" | awk 'seen{print} /^@@RISKS@@$/{seen=1}' | sed '1d')"

  local ac1 ac2 docs_block
  if [[ "$scaffold_only" == "true" ]]; then
    ac1="Running \`cargo init\` for \`icebox-cli\` yields a valid Rust binary crate scaffold with \`Cargo.toml\` and \`src/main.rs\`."
    ac2="Scaffold creation is non-interactive and reproducible for the same inputs."
  else
    ac1="${backlog} behavior matches backlog description: ${desc}"
    ac2="CLI output/errors are deterministic and user-safe."
  fi
  docs_block="- [x] ${spec_path}
- [ ] docs/plan/TESTING.md (if test mappings are added/changed)
- [ ] docs/architecture/decisions/ADR-*.md (if ADR required)
- [ ] docs/README.md (if user-facing behavior changed)"

  upsert_spec "$backlog" "$spec_path" "$use_case" "$desc" "$tests_block_md" "$adr_required" "$docs_block" "$ac1" "$ac2"

  local tmp
  tmp="$(mktemp)"
  {
    printf "Backlog ID: %s\n" "$backlog"
    printf "Use Case: %s\n" "$use_case"
    printf "Description: %s\n" "$desc"
    printf "Spec path: %s\n" "$spec_path"
    printf "\n## Objective\n"
    printf -- "- Deliver %s (%s).\n" "$backlog" "$use_case"
    printf -- "- Implement backlog contract exactly as documented.\n"
    printf "\n## Scope\n"
    printf "In scope:\n"
    printf -- "- %s\n" "$desc"
    printf "Out of scope:\n"
    printf -- "- Other backlog items outside %s\n" "$backlog"
    printf "\n## Acceptance Criteria\n"
    printf -- "- AC1: %s\n" "$ac1"
    printf -- "- AC2: %s\n" "$ac2"
    printf -- "- AC3: mapped tests cover happy path and failure path.\n"
    printf "\n## Tests Mapped\n%s\n" "$tests_block"
    printf "\nADR required?: %s\n" "$adr_required"
    printf "ADR link:\n"
    printf "\n## Docs impact listed\n"
    printf -- "- %s\n" "$spec_path"
    printf -- "- docs/plan/TESTING.md (if test mappings are added/changed)\n"
    printf -- "- docs/architecture/decisions/ADR-*.md (if ADR required)\n"
    printf -- "- docs/README.md (if user-facing behavior changed)\n"
    printf "\n## Definition Of Loaded (Required For ready-to-execute)\n"
    printf -- "- [%s] Backlog mapped\n" "$backlog_mark"
    printf -- "- [%s] Spec linked\n" "$spec_mark"
    printf -- "- [%s] Tests mapped\n" "$tests_mark"
    printf -- "- [%s] ADR triaged\n" "$adr_mark"
    printf -- "- [%s] Docs impact listed\n" "$docs_mark"
    printf "\n## Execution Plan (Required Before Coding)\n"
    printf "Commit split plan:\n"
    printf "%s\n" "$execution_commit_plan"
    printf "Planned validation commands:\n"
    printf "%s\n" "$execution_validation"
    printf "Risk notes:\n"
    printf "%s\n" "$execution_risks"
    printf "\n## Closeout Evidence (Required For done)\n"
    printf "PR link:\n"
    printf "Tests run (commands + result):\n"
    printf "Docs updated (paths):\n"
    printf "Files added/changed (paths):\n"
    printf "ADR link:\n"
  } > "$tmp"

  gh_try issue edit "$issue" --body-file "$tmp" >/dev/null
  rm -f "$tmp"

  local state
  state="$(current_state "$issue")"
  if [[ "$state" == "draft" ]]; then
    transition "$issue" "ready-for-review" "false" >/dev/null
  fi

  echo "ok: load updated #$issue for ${backlog}"
  if [[ -z "$tests" && "$scaffold_only" != "true" ]]; then
    echo "warn: no tests currently mapped for ${backlog} in docs/plan/TESTING.md"
  fi
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

  validate_backlog_id "$backlog" || return 1

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

  local epic epic_name project_name milestone project_title owner repo_full
  epic="$(epic_code_from_backlog "$backlog")"
  epic_name="$(roadmap_epic_name "$epic" || true)"
  if [[ -n "$epic_name" ]]; then
    project_name="${epic} - ${epic_name}"
  else
    project_name="${epic}"
  fi
  milestone="$(ensure_milestone "$epic" "$epic_name" || true)"
  project_title="$project_name"
  owner="$(repo_owner || true)"
  repo_full="$(repo_name_with_owner || true)"
  if [[ -n "$owner" ]]; then
    ensure_project "$owner" "$project_title" "$repo_full" >/dev/null 2>&1 || true
  fi

  local body
  body="$(cat <<EOF
Backlog ID: ${backlog}
Epic: ${epic}
Project: ${project_name}
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
Files added/changed (paths):
ADR link:
EOF
)"

  local out issue_url
  local cmd=("gh_try" "issue" "create" "--title" "$title" "--body" "$body" "--label" "draft")
  if [[ -n "$milestone" ]]; then
    cmd+=("--milestone" "$milestone")
  fi
  out="$("${cmd[@]}")"
  issue_url="$out"
  echo "ok: created issue $out"
  if [[ -n "$milestone" ]]; then
    echo "ok: attached milestone '$milestone'"
  else
    echo "warn: no matching GitHub milestone found for ${epic} (${epic_name:-unknown epic name})"
  fi
  if [[ -n "$project_title" ]]; then
    if [[ -n "$owner" ]] && attach_issue_to_project_v2 "$owner" "$project_title" "$issue_url" >/dev/null 2>&1; then
      echo "ok: attached project '$project_title'"
    else
      echo "warn: project attach failed for '$project_title' (requires gh project scope and Projects v2 access)"
    fi
  fi
}

main() {
  need_gh
  local cmd="${1:-}"
  shift || true

  case "$cmd" in
    create)
      create_issue "$@"
      ;;
    load)
      load_issue "$@"
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
    closeout)
      closeout_issue "$@"
      ;;
    done)
      done_issue "$@"
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
