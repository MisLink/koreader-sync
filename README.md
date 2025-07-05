# KOReader Sync Server

一个基于 Cloudflare Workers 的 KOReader 阅读进度同步服务，使用 Rust 和 WebAssembly 构建。

## 快速开始

### 前置要求

- Rust 1.70+
- Node.js 18+
- wrangler CLI
- Cloudflare 账户

### 本地开发

1. **生成项目**
    ```bash
    cargo generate MisLink/koreader-sync
    ```

2. **本地开发**
    ```bash
    npx wrangler dev -e local
    ```


### 部署到生产环境

1. **配置 wrangler.toml**

    确保 `wrangler.toml` 中的 KV 命名空间 ID 正确配置。

2. **部署**
    ```bash
    npx wrangler deploy -e ""
    ```
