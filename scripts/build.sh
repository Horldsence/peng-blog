#!/usr/bin/env bash

set -e

echo "==================================="
echo "  Peng Blog Build Script"
echo "==================================="
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Get the project root directory
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

# Parse arguments
BUILD_BACKEND=true
BUILD_FRONTEND=true
RELEASE_MODE=false

while [[ $# -gt 0 ]]; do
  case $1 in
    --frontend-only)
      BUILD_BACKEND=false
      shift
      ;;
    --backend-only)
      BUILD_FRONTEND=false
      shift
      ;;
    --release)
      RELEASE_MODE=true
      shift
      ;;
    --help)
      echo "Usage: $0 [OPTIONS]"
      echo ""
      echo "Options:"
      echo "  --frontend-only   Only build frontend"
      echo "  --backend-only    Only build backend"
      echo "  --release         Build in release mode (optimized)"
      echo "  --help            Show this help message"
      echo ""
      exit 0
      ;;
    *)
      echo -e "${RED}Unknown option: $1${NC}"
      exit 1
      ;;
  esac
done

# Build frontend
if [ "$BUILD_FRONTEND" = true ]; then
  echo -e "${BLUE}[1/2] Building Frontend...${NC}"
  echo "-----------------------------------"
  
  cd "$PROJECT_ROOT/frontend"
  
  # Install dependencies if node_modules doesn't exist
  if [ ! -d "node_modules" ]; then
    echo "Installing frontend dependencies..."
    npm install
  fi
  
  # Build frontend
  echo "Building frontend assets..."
  npm run build
  
  # Verify dist directory was created
  if [ ! -d "$PROJECT_ROOT/dist" ]; then
    echo -e "${RED}Error: Frontend build failed - dist directory not created${NC}"
    exit 1
  fi
  
  echo -e "${GREEN}✓ Frontend build complete${NC}"
  echo ""
else
  echo -e "${BLUE}[1/2] Skipping frontend build${NC}"
  echo ""
fi

# Build backend
if [ "$BUILD_BACKEND" = true ]; then
  echo -e "${BLUE}[2/2] Building Backend...${NC}"
  echo "-----------------------------------"
  
  cd "$PROJECT_ROOT"
  
  if [ "$RELEASE_MODE" = true ]; then
    echo "Building backend in release mode..."
    cargo build --release
    BINARY_PATH="target/release/peng-blog"
  else
    echo "Building backend in debug mode..."
    cargo build
    BINARY_PATH="target/debug/peng-blog"
  fi
  
  # Verify binary was created
  if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${RED}Error: Backend build failed - binary not created${NC}"
    exit 1
  fi
  
  echo -e "${GREEN}✓ Backend build complete${NC}"
  echo ""
else
  echo -e "${BLUE}[2/2] Skipping backend build${NC}"
  echo ""
fi

# Summary
echo "==================================="
echo -e "${GREEN}Build Complete!${NC}"
echo "==================================="
echo ""

if [ "$BUILD_FRONTEND" = true ]; then
  echo "Frontend assets: ./dist"
fi

if [ "$BUILD_BACKEND" = true ]; then
  if [ "$RELEASE_MODE" = true ]; then
    echo "Backend binary: ./target/release/peng-blog"
    echo ""
    echo "Run with: ./target/release/peng-blog"
  else
    echo "Backend binary: ./target/debug/peng-blog"
    echo ""
    echo "Run with: cargo run"
  fi
fi

echo ""
echo "The server will serve frontend at http://localhost:3000"
echo "API will be available at http://localhost:3000/api"
echo ""