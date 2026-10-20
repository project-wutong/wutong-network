# Wutong Network · 梧桐网络

[![CI](https://github.com/project-wutong/wutong-network/actions/workflows/ci.yml/badge.svg)](https://github.com/project-wutong/wutong-network/actions/workflows/ci.yml)
Qt 6 GUI (C++23) + Rust 2024 engine · `AGPL-3.0-only`

## 中文

本地知识浏览与节点配置界面：身份/Ed25519 与 X25519 密钥与撤销管理、联邦与同步状态。可启停并连接用户可选的 Rust 后台 `wutong-networkd`；后台关闭时仍可浏览本地缓存。

**离线优先**：无需账户、网络、Server、后台或 AI 即可完成本地核心功能（OFFLINE-001）。

### 架构（Rust 优先）

```
src/ (C++ / Qt Widgets, 仅 GUI)
   │  cxx bridge (crates/wutong-network-bridge)
   ▼
crates/wutong-network-core (纯 Rust 领域逻辑，可独立测试)
   ▼
wutong-core (协议核心：解析、规范化、签名、策略)
```

### 构建

```bash
cargo test --workspace                       # Rust 引擎与桥接
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release && cmake --build build   # 需要 Qt ≥ 6.4（目标基线 6.12 LTS）
./build/wutong-network
```

## English

Wutong Network — the GUI layer is C++/Qt; everything else (document model, protocol, storage) is Rust,
reached through a `cxx` bridge. Works fully offline.
