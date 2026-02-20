#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  discussion_proposal.sh next-id
  discussion_proposal.sh draft --link <url> [--d-id D#] [--title "..."] [--summary "..."] [--proposed-by "@user"] [--status "..."] [--backlog "..."]
  discussion_proposal.sh create --link <url> [--d-id D#] [--title "..."] [--summary "..."] [--proposed-by "@user"] [--status "..."] [--backlog "..."]

Defaults:
  status  = logged
  backlog = n/a
EOF
}

require_gh() {
  if ! command -v gh >/dev/null 2>&1; then
    echo "error: gh CLI is required" >&2
    exit 1
  fi
}

extract_discussion_number() {
  local link="$1"
  sed -nE 's#.*\/discussions\/([0-9]+).*#\1#p' <<<"$link"
}

next_id() {
  require_gh
  local max=0
  while IFS= read -r title; do
    if [[ "$title" =~ ^\[D([0-9]+)\] ]]; then
      local n="${BASH_REMATCH[1]}"
      if (( n > max )); then
        max="$n"
      fi
    fi
  done < <(gh issue list --state all --limit 500 --json title --jq '.[].title')
  printf 'D%s\n' "$((max + 1))"
}

render_title() {
  local did="$1"
  local link="$2"
  local discussion_num
  discussion_num="$(extract_discussion_number "$link" || true)"
  if [[ -n "$discussion_num" ]]; then
    printf '[%s] Proposal: from discussion #%s\n' "$did" "$discussion_num"
  else
    printf '[%s] Proposal: from external discussion\n' "$did"
  fi
}

render_body() {
  local link="$1"
  local proposed_by="$2"
  local summary="$3"
  local status="$4"
  local backlog="$5"
  cat <<EOF
## Source
[$link]($link) — proposed by ${proposed_by}

## Summary
${summary}

## Status
- [$( [[ "$status" == "logged" ]] && printf "x" || printf " " )] Logged
- [$( [[ "$status" == "under review" ]] && printf "x" || printf " " )] Under review
- [$( [[ "$status" == "added to backlog" ]] && printf "x" || printf " " )] Added to backlog as ${backlog}
- [$( [[ "$status" == "closed without adoption" ]] && printf "x" || printf " " )] Closed without adoption
EOF
}

parse_args() {
  LINK=""
  DID=""
  TITLE=""
  SUMMARY="Brief description of the proposal."
  PROPOSED_BY="@unknown"
  STATUS="logged"
  BACKLOG="n/a"

  while [[ $# -gt 0 ]]; do
    case "$1" in
      --link)
        LINK="${2:-}"
        shift 2
        ;;
      --d-id)
        DID="${2:-}"
        shift 2
        ;;
      --title)
        TITLE="${2:-}"
        shift 2
        ;;
      --summary)
        SUMMARY="${2:-}"
        shift 2
        ;;
      --proposed-by)
        PROPOSED_BY="${2:-}"
        shift 2
        ;;
      --status)
        STATUS="${2:-}"
        shift 2
        ;;
      --backlog)
        BACKLOG="${2:-}"
        shift 2
        ;;
      *)
        echo "error: unknown arg: $1" >&2
        usage
        exit 1
        ;;
    esac
  done

  if [[ -z "$LINK" ]]; then
    echo "error: --link is required" >&2
    usage
    exit 1
  fi
}

main() {
  local cmd="${1:-}"
  if [[ -z "$cmd" ]]; then
    usage
    exit 1
  fi
  shift || true

  case "$cmd" in
    next-id)
      next_id
      ;;
    draft|create)
      parse_args "$@"
      if [[ -z "$DID" ]]; then
        DID="$(next_id)"
      fi
      if [[ -z "$TITLE" ]]; then
        TITLE="$(render_title "$DID" "$LINK")"
      fi
      BODY="$(render_body "$LINK" "$PROPOSED_BY" "$SUMMARY" "$STATUS" "$BACKLOG")"

      if [[ "$cmd" == "draft" ]]; then
        printf 'D-ID: %s\n' "$DID"
        printf 'Title: %s\n\n' "$TITLE"
        printf '%s\n' "$BODY"
        exit 0
      fi

      require_gh
      issue_url="$(
        gh issue create \
          --title "$TITLE" \
          --body "$BODY" \
          --label discussion-proposal \
          --label proposal
      )"
      printf 'Created: %s\n' "$issue_url"
      printf 'D-ID: %s\n' "$DID"
      ;;
    *)
      echo "error: unknown command: $cmd" >&2
      usage
      exit 1
      ;;
  esac
}

main "$@"
