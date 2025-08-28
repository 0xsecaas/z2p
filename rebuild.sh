#!/usr/bin/env bash
set -euo pipefail

APP_NAME="z2p"
IMAGE_NAME="z2p:latest"

echo "[1] Stopping old container..."
docker stop "$APP_NAME" 2>/dev/null || true
docker rm "$APP_NAME" 2>/dev/null || true

echo "[2] Building new image..."
DOCKER_BUILDKIT=1 docker build -t "$IMAGE_NAME" .

echo "[3] Starting updated container..."
docker run -d --name "$APP_NAME" \
  --restart unless-stopped \
  -p 8000:8000 \
  "$IMAGE_NAME"

echo "[4] Cleanup dangling images..."
docker image prune -f

echo "[OK] Container updated."
