#!/usr/bin/env bash
#
# Smoke test for the Rust MCP server.
#
# Validates the public surface end-to-end:
#   1. GET  /healthz                     -> 200 with ok=true
#   2. POST /mcp without bearer token    -> 401
#   3. POST /mcp coordinate distance     -> 200 with distance=5.0
#   4. POST /mcp safe_system_metrics     -> 200 with ok=true
#
# Inputs are resolved in this order:
#   --base-url / --api-key flags
#   BASE_URL / MCP_API_KEY environment variables
#   `azd env get-values` (SERVICE_API_ENDPOINT_URL, MCP_API_KEY)
#
# Exits non-zero on any failure so CI or pre-release gates can rely on it.

set -euo pipefail

BASE_URL="${BASE_URL:-}"
API_KEY="${MCP_API_KEY:-}"

usage() {
  cat <<USAGE
Usage: $0 [--base-url URL] [--api-key KEY]

Resolves BASE_URL and MCP_API_KEY from flags, env vars, or 'azd env get-values'.
Hits the four critical paths and exits non-zero on the first failure.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --base-url)
      BASE_URL="$2"
      shift 2
      ;;
    --api-key)
      API_KEY="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -z "$BASE_URL" || -z "$API_KEY" ]]; then
  if command -v azd >/dev/null 2>&1; then
    while IFS='=' read -r key value; do
      value="${value%\"}"
      value="${value#\"}"
      case "$key" in
        SERVICE_API_ENDPOINT_URL)
          if [[ -z "$BASE_URL" ]]; then BASE_URL="$value"; fi
          ;;
        MCP_API_KEY)
          if [[ -z "$API_KEY" ]]; then API_KEY="$value"; fi
          ;;
      esac
    done < <(azd env get-values 2>/dev/null || true)
  fi
fi

if [[ -z "$BASE_URL" ]]; then
  echo "BASE_URL is not set (use --base-url, BASE_URL env, or run 'azd env select <env>')" >&2
  exit 2
fi
if [[ -z "$API_KEY" ]]; then
  echo "MCP_API_KEY is not set (use --api-key, MCP_API_KEY env, or set it in azd env)" >&2
  exit 2
fi

BASE_URL="${BASE_URL%/}"

declare -i FAILURES=0
declare -i PASSES=0
declare -i SKIPS=0

# Detect host OS — used to skip Linux-only checks when the server is running
# locally on a non-Linux host (e.g. Windows native, where safe_system_metrics
# reads /proc and returns 500). Remote Azure deployments are always Linux,
# so the skip only triggers for localhost/127.0.0.1 base URLs.
HOST_IS_LINUX=1
case "$(uname -s 2>/dev/null)" in
  MINGW*|MSYS*|CYGWIN*|Windows_NT)
    HOST_IS_LINUX=0
    ;;
  Darwin)
    HOST_IS_LINUX=0
    ;;
esac

pass() {
  PASSES+=1
  printf '  PASS  %s\n' "$1"
}

fail() {
  FAILURES+=1
  printf '  FAIL  %s\n' "$1"
  if [[ -n "${2:-}" ]]; then
    printf '        %s\n' "$2"
  fi
}

skip() {
  SKIPS+=1
  printf '  SKIP  %s\n' "$1"
  if [[ -n "${2:-}" ]]; then
    printf '        %s\n' "$2"
  fi
}

http_call() {
  # http_call METHOD PATH [auth|noauth] [json-body]
  local method="$1"
  local path="$2"
  local auth_mode="${3:-noauth}"
  local body="${4:-}"

  local args=(-sS -o /tmp/smoke-body.$$ -w '%{http_code}' -X "$method" "$BASE_URL$path")
  if [[ "$auth_mode" == "auth" ]]; then
    args+=(-H "Authorization: Bearer $API_KEY")
  fi
  if [[ -n "$body" ]]; then
    args+=(-H 'Content-Type: application/json' --data-raw "$body")
  fi

  local status
  if ! status=$(curl "${args[@]}" 2>/dev/null); then
    echo "curl failed for $method $path"
    rm -f /tmp/smoke-body.$$
    return 1
  fi

  local body_content=''
  if [[ -f /tmp/smoke-body.$$ ]]; then
    body_content=$(cat /tmp/smoke-body.$$)
    rm -f /tmp/smoke-body.$$
  fi
  printf '%s\n%s' "$status" "$body_content"
}

extract_field() {
  # Naive JSON extractor; avoids a hard dependency on jq.
  # extract_field BODY KEY
  local body="$1"
  local key="$2"
  # Match "key" : <number|true|false|null|"string">
  printf '%s' "$body" \
    | tr -d '\n' \
    | grep -oE "\"$key\"[[:space:]]*:[[:space:]]*([0-9.eE+-]+|true|false|null|\"[^\"]*\")" \
    | head -n1 \
    | sed -E "s/\"$key\"[[:space:]]*:[[:space:]]*//; s/^\"//; s/\"$//"
}

echo "Smoke testing ${BASE_URL}"

# 1. /healthz
result=$(http_call GET /healthz noauth)
status=$(printf '%s' "$result" | head -n1)
body=$(printf '%s' "$result" | tail -n +2)
if [[ "$status" == "200" && "$(extract_field "$body" ok)" == "true" ]]; then
  pass "GET /healthz returns 200 with ok=true"
else
  fail "GET /healthz" "status=$status body=$body"
fi

# 2. /mcp without auth
result=$(http_call POST /mcp noauth '{"tool":"safe_system_metrics","input":{"sections":["cpu"]}}')
status=$(printf '%s' "$result" | head -n1)
body=$(printf '%s' "$result" | tail -n +2)
if [[ "$status" == "401" ]]; then
  pass "POST /mcp without bearer is rejected with 401"
else
  fail "POST /mcp without bearer" "status=$status body=$body"
fi

# 3. /mcp coordinate distance
distance_payload='{"tool":"coordinate_grid_calculator","input":{"operation":"distance","from":{"x":0,"y":0},"to":{"x":3,"y":4}}}'
result=$(http_call POST /mcp auth "$distance_payload")
status=$(printf '%s' "$result" | head -n1)
body=$(printf '%s' "$result" | tail -n +2)
if [[ "$status" == "200" ]]; then
  distance_value=$(extract_field "$body" distance)
  if [[ "$distance_value" == "5" || "$distance_value" == "5.0" ]]; then
    pass "POST /mcp coordinate distance returns 5.0"
  else
    fail "POST /mcp coordinate distance" "expected distance=5.0, got distance=$distance_value body=$body"
  fi
else
  fail "POST /mcp coordinate distance" "status=$status body=$body"
fi

# 4. /mcp safe_system_metrics
# safe_system_metrics reads /proc, which only exists on Linux. When the server
# runs locally on Windows or macOS native (i.e. BASE_URL points at localhost),
# the call returns HTTP 500. Remote Azure deployments are always Linux, so we
# only skip when both conditions hold: non-Linux host *and* localhost target.
is_localhost=0
case "$BASE_URL" in
  *localhost*|*127.0.0.1*|*://[::1]*)
    is_localhost=1
    ;;
esac

if (( HOST_IS_LINUX == 0 )) && (( is_localhost == 1 )); then
  skip "POST /mcp safe_system_metrics" "non-Linux host + localhost target; /proc not available"
else
  metrics_payload='{"tool":"safe_system_metrics","input":{"sections":["cpu","memory","runtime"]}}'
  result=$(http_call POST /mcp auth "$metrics_payload")
  status=$(printf '%s' "$result" | head -n1)
  body=$(printf '%s' "$result" | tail -n +2)
  if [[ "$status" == "200" && "$(extract_field "$body" ok)" == "true" ]]; then
    pass "POST /mcp safe_system_metrics returns ok=true"
  else
    fail "POST /mcp safe_system_metrics" "status=$status body=$body"
  fi
fi

echo
echo "Summary: ${PASSES} passed, ${FAILURES} failed, ${SKIPS} skipped"

if (( FAILURES > 0 )); then
  exit 1
fi
