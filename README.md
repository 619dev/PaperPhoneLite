# PaperPhoneLite 3.0.0

[简体中文](README.md) · [English](README_EN.md) · [日本語](README_JA.md) · [한국어](README_KO.md) · [Français](README_FR.md) · [Deutsch](README_DE.md) · [Русский](README_RU.md) · [Español](README_ES.md)

[更新历史](changelog.md)

PaperPhoneLite 是轻量级端到端加密即时通讯项目，使用 React 19/TypeScript Web 客户端与 Rust/Axum 服务端。

保留功能：私聊、群聊、文字/图片/视频/语音/文档消息、联系人、群组、消息同步、离线缓存、阅后定时删除、推送、二维码、2FA 与拉黑。

<details>
<summary>应用截图</summary>

![PaperPhoneLite 应用截图 1](screenshot/ui1.png)
![PaperPhoneLite 应用截图 2](screenshot/ui2.png)
![PaperPhoneLite 应用截图 3](screenshot/ui3.png)

</details>

## 特性

| 功能 | 说明 |
|------|------|
| 🔐 端到端加密 | X25519 + ML-KEM-768 混合密钥协商与 XSalsa20-Poly1305 消息加密，支持 Signal 风格安全号码验证 |
| 🗝️ 零知识加密范围 | 服务器保存加密会话密文，仅处理账号、好友/群组、路由和推送等必要元数据；身份私钥与 Sender Key 保留在本地 |
| 🎭 文本外观与额外加密 | 可为本设备上的聊天设置额外密码和 8 种文本外观，支持手动锁定和离开前台后自动锁定 |
| 🎤 语音消息与变声 | 录制语音消息，并支持 0.8x 低沉、1.0x 正常、1.2x 高音三档本地变声 |
| 📱 会话保持 | 30 分钟 Access Token + 90 天设备 Refresh Token；网络、IP、VPN 或代理变化时自动重建连接 |
| 📨 可靠消息同步 | WebSocket 双向心跳、持久化发件箱、客户端消息 ID 幂等和服务端序号增量补齐 |
| 📴 离线访问 | 按账户隔离缓存联系人、群组、每会话最多 2000 条消息和媒体；离线发送会在联网后重试 |
| 🔎 Unicode 好友搜索 | 支持中文输入法组合状态、NFC 归一化与 UTF-8 查询 |
| 👥 群聊 | 最多 2000 人；加密群使用 Sender Key 协议，并支持免打扰、公告和成员管理 |
| 👫 好友系统 | 好友申请需对方确认，支持验证消息、备注和多标签分组 |
| ⏱️ 消息自动删除 | 支持永不、1 天、3 天、7 天、30 天五档；私聊双方可设置，群聊仅群主可设置 |
| 🔔 消息推送 | Android 使用 ntfy，iOS 使用 APNs；不集成任何 Google 推送框架 |
| 🌐 多语言 | 中文、英文、日语、韩语、法语、德语、俄语、西班牙语 |
| 💬 丰富消息 | 文字、图片、视频、文档、语音、Emoji、Telegram 贴纸、已读状态和输入状态 |
| 📤 服务端文件存储 | 单文件最大 500MB，文件仅保存在服务端持久卷，由 Rust 服务端传输 |
| 🏷️ 好友标签 | 可给好友设置多个标签，并按标签筛选通讯录 |
| 🔑 两步验证 | 兼容 Google Authenticator 的 TOTP，提供 8 个一次性恢复码 |
| 📷 扫码加好友/入群 | 扫描二维码添加好友或加入群聊，群邀请可设置有效期 |
| 🚫 拉黑 | 可拉黑其他用户，阻止后续消息交互 |
| 🦅 Tor 隐藏服务 | Docker Compose 使用 Tor v3 onion service 直连 Rust 后端，MySQL 与 Redis 不公开端口 |
| 🏗️ 可自托管 | 生产环境使用 Docker Compose + Tor v3 onion service，无需公开域名或应用端口 |

3.0.0 已彻底移除朋友圈、时间线、单聊语音/视频通话、群聊语音/视频通话、LiveKit 与 Cloudflare R2。文件只写入服务端持久卷并由服务端传输。自托管模板不使用 Nginx，Tor v3 onion service 直接转发至 Rust 服务端；数据库和 Redis 不公开端口。

Docker Compose 启动后，Tor 容器会自动把完整的 `http://...onion` 服务地址写入宿主机的 `logs/onion-address.log`；也可通过 `docker compose logs tor` 查看。部署细节见 [DEPLOY_CN.md](DEPLOY_CN.md)。

UI 源码位于 `client/`，供 Android、iOS、Windows、macOS 原生壳复用。生产客户端必须内嵌 Tor，必须等待 Tor bootstrap 完成后才能登录，并禁止 `.onion` 地址回退到明网。普通浏览器/PWA 不是受支持的生产客户端。

## 应用发布

- iOS 客户端计划通过 Apple App Store 发布，并使用 APNs。
- Android APK 仅通过 Android 客户端仓库的 [GitHub Releases](https://github.com/619dev/ppl-android/releases) 发布，使用 ntfy，不发布到 Google Play，也不集成 Google 推送或服务框架。

## 客户端仓库

| 平台 | 源码与版本发布 |
|------|----------------|
| Android | [619dev/ppl-android](https://github.com/619dev/ppl-android) |
| iOS | [619dev/ppl-ios](https://github.com/619dev/ppl-ios) |
| macOS | [619dev/ppl-mac](https://github.com/619dev/ppl-mac) |
| Windows | [619dev/ppl-win](https://github.com/619dev/ppl-win) |

中文部署指南见 [DEPLOY_CN.md](DEPLOY_CN.md)，英文部署指南见 [DEPLOY_EN.md](DEPLOY_EN.md)。

```bash
cd client && npm install && npm run build
cd ../server && cargo check
```

许可证：AGPL-3.0。

---

如果这个项目对你有用，请我喝罐可乐吧。

<img width="30%" height="30%" src="请我喝可乐.jpg" alt="打赏二维码">

Telegram 群组：https://t.me/+vHJtvWJY_gEyMTUx
