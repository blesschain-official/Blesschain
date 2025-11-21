#!/bin/bash

# ================================
# BlessChain Build Watch Script
# 最高稳定性 + 超强日志 + T7910 优化
# ================================

# --- 必要环境变量 ---
export LIBROCKSDB_SYS_USE_PKG_CONFIG=1
export CARGO_BUILD_TIMINGS=1
export TMPDIR=/dev/shm/cargo-temp
export RUSTFLAGS="-C link-arg=-fuse-ld=lld"

mkdir -p /dev/shm/cargo-temp
mkdir -p /dev/shm/sccache

# --- 确保 sccache 运行 ---
if command -v sccache >/dev/null 2>&1; then
    export RUSTC_WRAPPER="sccache"
else
    echo "[警告] sccache 未安装，将使用 rustc 编译"
fi

# --- 显示环境 ---
echo "==============================="
echo " BlessChain Build Watch Script "
echo "==============================="
echo "LIBROCKSDB_SYS_USE_PKG_CONFIG = $LIBROCKSDB_SYS_USE_PKG_CONFIG"
echo "CARGO_BUILD_TIMINGS            = $CARGO_BUILD_TIMINGS"
echo "TMPDIR                         = $TMPDIR"
echo "==============================="

# --- 正式开始 build ---
echo "[BUILD] 开始编译 blesschain-node ..."

cargo build -p blesschain-node --release -j 8 -vv 2>&1 | tee build.log

# --- 导出错误到 error.txt ---
echo "[BUILD] 导出错误到 error.txt ..."
grep -iE "error|failed" build.log > error.txt

echo "==============================="
echo " Build 完成。请打开：build.log / error.txt"
echo " Build 完整性能报告：target/cargo-timings/cargo-timings.html"
echo "==============================="

