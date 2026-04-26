#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

ENV_FILE="$ROOT/docker/.env"
COMPOSE="docker/docker-compose.dev.yml"

# ── colours ────────────────────────────────────────────────────────────────
RED="\033[0;31m"; GREEN="\033[0;32m"; YELLOW="\033[1;33m"
CYAN="\033[0;36m"; BOLD="\033[1m"; RESET="\033[0m"

# ── spinner ────────────────────────────────────────────────────────────────
_SPINNER_PID=""

spinner_start() {
  local msg="$1"
  (
    local frames=("⠋" "⠙" "⠹" "⠸" "⠼" "⠴" "⠦" "⠧" "⠇" "⠏")
    local i=0
    while true; do
      printf "\r  \033[0;36m${frames[$((i % 10))]}\033[0m  %s" "$msg"
      i=$((i + 1))
      sleep 0.08
    done
  ) 2>/dev/null &
  _SPINNER_PID=$!
}

spinner_stop() {
  if [[ -n "$_SPINNER_PID" ]]; then
    kill "$_SPINNER_PID" 2>/dev/null
    wait "$_SPINNER_PID" 2>/dev/null || true
    _SPINNER_PID=""
    printf "\r\033[K"
  fi
}

status_ok()   { spinner_stop; printf "  \033[0;32m✓\033[0m  %s\n" "$1"; }
status_fail() { spinner_stop; printf "  \033[0;31m✗\033[0m  %s\n" "$1"; }
status_skip() { spinner_stop; printf "  \033[1;33m–\033[0m  %s\n" "$1"; }

trap 'spinner_stop; exit 130' INT TERM

# ── wait for container health ──────────────────────────────────────────────
wait_healthy() {
  local container="$1"
  local max=60
  local i=0
  while [[ $i -lt $max ]]; do
    local s
    s=$(docker inspect --format='{{.State.Health.Status}}' "$container" 2>/dev/null || echo "none")
    case "$s" in
      healthy)   return 0 ;;
      unhealthy) return 1 ;;
    esac
    sleep 1
    i=$((i + 1))
  done
  return 1
}

# ── per-service start ──────────────────────────────────────────────────────
start_service() {
  local svc="$1" label="$2" wait_health="${3:-false}"
  spinner_start "Starting $label..."
  if docker compose -f "$COMPOSE" up -d --build "$svc" &>/dev/null; then
    if [[ "$wait_health" == "true" ]]; then
      local container
      container=$(docker compose -f "$COMPOSE" ps -q "$svc" 2>/dev/null)
      if wait_healthy "$container"; then
        status_ok "$label — ready"
      else
        status_fail "$label — health check failed"
        return 1
      fi
    else
      status_ok "$label — started"
    fi
  else
    status_fail "$label — failed to start"
    return 1
  fi
}

# ── per-service stop ───────────────────────────────────────────────────────
stop_service() {
  local container="$1" label="$2"
  spinner_start "Stopping $label..."
  if docker stop "$container" &>/dev/null; then
    status_ok "$label — stopped"
  else
    status_skip "$label — not running"
  fi
}

# ── load .env ──────────────────────────────────────────────────────────────
if [[ ! -f "$ENV_FILE" ]]; then
  printf "${RED}✗${RESET}  docker/.env not found. Copy the example and fill in values:\n"
  printf "     cp docker/.env.example docker/.env\n"
  exit 1
fi

set -a
# shellcheck source=/dev/null
source "$ENV_FILE"
set +a

# ── usage ──────────────────────────────────────────────────────────────────
usage() {
  printf "${BOLD}Usage:${RESET} ./scripts/manage.sh <command>\n\n"
  printf "${BOLD}Services:${RESET}\n"
  printf "  up           Start all services with hot-reload\n"
  printf "  down         Stop all services\n"
  printf "  stop         Alias for down\n"
  printf "  restart      Restart all services\n\n"
  printf "${BOLD}Logs:${RESET}\n"
  printf "  logs         Follow logs from all services\n"
  printf "  backend      Logs from backend only\n"
  printf "  frontend     Logs from frontend only\n"
  printf "  db           Logs from postgres only\n"
  printf "  redis        Logs from redis only\n\n"
  printf "${BOLD}Database:${RESET}\n"
  printf "  migrate      Run all pending migrations\n"
  printf "  migrate:down Revert last migration\n\n"
  printf "${BOLD}Utility:${RESET}\n"
  printf "  ps           Show running containers\n"
  printf "  clean        Remove all volumes ${RED}(WARNING: deletes DB data)${RESET}\n"
}

# ── commands ───────────────────────────────────────────────────────────────
case "${1:-}" in
  up)
    printf "${BOLD}▶  Starting dev stack${RESET}\n"
    start_service postgres  "PostgreSQL"  true
    start_service redis     "Redis"       true
    start_service backend   "Backend"     false
    start_service frontend  "Frontend"    false
    printf "${GREEN}${BOLD}✓  Dev stack is up${RESET}\n"
    ;;

  down|stop)
    printf "${BOLD}▶  Stopping dev stack${RESET}\n"
    stop_service frontend  "Frontend"
    stop_service backend   "Backend"
    stop_service redis     "Redis"
    stop_service postgres  "PostgreSQL"
    printf "${GREEN}${BOLD}✓  All services stopped${RESET}\n"
    ;;

  restart)
    printf "${BOLD}▶  Restarting dev stack${RESET}\n"
    stop_service frontend  "Frontend"
    stop_service backend   "Backend"
    stop_service redis     "Redis"
    stop_service postgres  "PostgreSQL"
    printf "\n"
    start_service postgres  "PostgreSQL"  true
    start_service redis     "Redis"       true
    start_service backend   "Backend"     false
    start_service frontend  "Frontend"    false
    printf "${GREEN}${BOLD}✓  Dev stack restarted${RESET}\n"
    ;;

  logs)
    docker compose -f "$COMPOSE" logs -f
    ;;
  backend)
    docker compose -f "$COMPOSE" logs -f backend
    ;;
  frontend)
    docker compose -f "$COMPOSE" logs -f frontend
    ;;
  db)
    docker compose -f "$COMPOSE" logs -f postgres
    ;;
  redis)
    docker compose -f "$COMPOSE" logs -f redis
    ;;

  migrate)
    spinner_start "Running migrations..."
    DB_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:${POSTGRES_PORT}/${POSTGRES_DB}"
    if DATABASE_URL="$DB_URL" sqlx migrate run --source src/backend/migrations &>/dev/null; then
      status_ok "Migrations applied"
    else
      status_fail "Migration failed"
      exit 1
    fi
    ;;

  migrate:down)
    spinner_start "Reverting last migration..."
    DB_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:${POSTGRES_PORT}/${POSTGRES_DB}"
    if DATABASE_URL="$DB_URL" sqlx migrate revert --source src/backend/migrations &>/dev/null; then
      status_ok "Migration reverted"
    else
      status_fail "Revert failed"
      exit 1
    fi
    ;;

  ps)
    docker compose -f "$COMPOSE" ps
    ;;

  clean)
    printf "${YELLOW}⚠  This deletes all volumes (DB data).${RESET}\n"
    read -rp "   Continue? [y/N] " confirm
    [[ "$confirm" == [yY] ]] || exit 0
    printf "${BOLD}▶  Removing volumes${RESET}\n"
    stop_service frontend  "Frontend"
    stop_service backend   "Backend"
    stop_service redis     "Redis"
    stop_service postgres  "PostgreSQL"
    spinner_start "Removing containers and volumes..."
    docker rm -f frontend backend redis postgres &>/dev/null || true
    docker compose -f "$COMPOSE" down -v &>/dev/null || true
    status_ok "Volumes removed"
    ;;

  *)
    usage
    ;;
esac
