#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

exec docker run --rm -i \
  -v "$repo_root:/io" \
  -v wowdecomp-xwin-cache:/xwin-cache \
  -e XWIN_CACHE_DIR=/xwin-cache \
  -w /io \
  messense/cargo-xwin \
  bash -s <<'CONTAINER'
set -euo pipefail

env_file="$(mktemp)"
trap 'rm -f -- "$env_file"' EXIT

cargo xwin env \
  --manifest-path rust/gameplay/Cargo.toml \
  --target x86_64-pc-windows-msvc \
  > "$env_file"

source "$env_file"

build_dir=rust/gameplay/target/cmake-win64

cmake -S rust -B "$build_dir" -G Ninja \
  -DCMAKE_TOOLCHAIN_FILE="$CMAKE_TOOLCHAIN_FILE_x86_64_pc_windows_msvc" \
  -DRust_CARGO_TARGET=x86_64-pc-windows-msvc \
  -DCMAKE_BUILD_TYPE=RelWithDebInfo \
  -DCMAKE_MSVC_RUNTIME_LIBRARY=MultiThreadedDLL

cmake --build "$build_dir" \
  --target wow_cooldown_bridge_test

wine "$build_dir/wow_cooldown_bridge_test.exe"
CONTAINER
