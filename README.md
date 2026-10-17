# PaperPhoneLite 3.1.0

[简体中文](README.md) · [English](README_EN.md) · [日本語](README_JA.md) · [한국어](README_KO.md) · [Français](README_FR.md) · [Deutsch](README_DE.md) · [Русский](README_RU.md) · [Español](README_ES.md)

[更新历史](changelog.md)

[![Rust](https://img.shields.io/badge/Rust-1.83+-orange)](#) [![Axum](https://img.shields.io/badge/Axum-0.8-black)](#) [![React](https://img.shields.io/badge/React-19-blue)](#) [![TypeScript](https://img.shields.io/badge/TypeScript-5.7-blue)](#) [![MySQL](https://img.shields.io/badge/MySQL-8.0-blue)](#) [![Redis](https://img.shields.io/badge/Redis-7.x-red)](#) [![Tor](https://img.shields.io/badge/Tor-v3-7D4698?logo=tor-project)](#) [![Version](https://img.shields.io/badge/Version-3.1.0-orange)](client/package.json) [![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](LICENSE)

PaperPhoneLite 是轻量级端到端加密即时通讯项目，使用 React 19/TypeScript Web 客户端与 Rust/Axum 服务端。

保留功能：私聊、群聊、文字/图片/视频/语音/文档消息、联系人、群组、消息同步、离线缓存、阅后定时删除、Android ntfy 与 iOS Bark 后台提醒、二维码、2FA 与拉黑。

## 特性

| 功能 | 说明 |
|------|------|
| 🔐 端到端加密 | X25519 与 XSalsa20-Poly1305 消息加密、Ed25519 签名及安全号码验证 |
| 🗝️ 加密范围 | 消息正文加密，身份私钥与 Sender Key 保留在本地；服务端仍处理必要元数据并保存上传文件 |
| 🎭 文本外观与额外加密 | 可为本设备上的聊天设置额外密码和 8 种文本外观，支持手动锁定和离开前台后自动锁定 |
| 🎤 语音消息与变声 | 录制语音消息，并支持 0.8x 低沉、1.0x 正常、1.2x 高音三档本地变声 |
| 📱 会话保持 | 30 分钟 Access Token + 90 天设备 Refresh Token；网络、IP、VPN 或代理变化时自动重建连接 |
| 📨 可靠消息同步 | WebSocket 双向心跳、持久化发件箱、客户端消息 ID 幂等和服务端序号增量补齐 |
| 📴 离线访问 | 按账户隔离缓存联系人、群组、每会话最多 2000 条消息和媒体；离线发送会在联网后重试 |
| 🔎 Unicode 好友搜索 | 支持中文输入法组合状态、NFC 归一化与 UTF-8 查询 |
| 👥 群聊 | 最多 2000 人；新群消息强制使用签名与 Sender Key 加密，并支持免打扰、公告和成员管理 |
| 👫 好友系统 | 好友申请需对方确认，支持验证消息、备注和多标签分组 |
| ⏱️ 消息自动删除 | 支持永不、1 天、3 天、7 天、30 天五档；私聊双方可设置，群聊仅群主可设置 |
| 🔔 后台通知 | Android 可使用 ntfy；iOS 可通过用户配置的 Bark 地址接收提醒；服务端默认仅允许 `api.day.app`，可通过 `BARK_ALLOWED_HOSTS` 增加自建主机 |
| 🌐 多语言 | 中文、英文、日语、韩语、法语、德语、俄语、西班牙语 |
| 💬 丰富消息 | 文字、图片、视频、文档、语音、Emoji、Telegram 贴纸、已读状态和输入状态 |
| 📤 服务端文件存储 | 单文件最大 500MB；好友/用户头像和群头像永久保存，私聊及群聊附件进入临时目录并按服务端配置定期清理；升级时自动迁移旧版文件；附件由 Rust 服务端中转下载 |
| 🏷️ 好友标签 | 可给好友设置多个标签，并按标签筛选通讯录 |
| 🔑 两步验证 | 兼容 Google Authenticator 的 TOTP，提供 8 个一次性恢复码 |
| 📷 扫码加好友/入群 | 扫描二维码添加好友或加入群聊，群邀请可设置有效期 |
| 🚫 拉黑 | 可拉黑其他用户，阻止后续消息交互 |
| 🦅 Tor 隐藏服务 | Docker Compose 使用 Tor v3 onion service 直连 Rust 后端，MySQL 与 Redis 不公开端口 |
| 🏗️ 可自托管 | 生产环境使用 Docker Compose + Tor v3 onion service，无需公开域名或应用端口 |

3.0.0 已彻底移除朋友圈、时间线、单聊语音/视频通话、群聊语音/视频通话、LiveKit 与 Cloudflare R2。文件只写入服务端持久卷并由服务端传输。自托管模板不使用 Nginx，Tor v3 onion service 直接转发至 Rust 服务端；数据库和 Redis 不公开端口。

服务端启动时会自动删除旧版本遗留的 Web Push `push_subscriptions` 和 OneSignal `onesignal_players` 表及其中已废弃的推送标识；当前版本不会重新创建或使用这些表。

Docker Compose 启动后，Tor 容器会自动把完整的 `http://...onion` 服务地址写入宿主机的 `logs/onion-address.log`；也可通过 `docker compose logs tor` 查看。部署细节见 [DEPLOY_CN.md](DEPLOY_CN.md)。

UI 源码位于 `client/`，供 Android、iOS、Windows、macOS 原生壳复用。生产客户端必须内嵌 Tor，必须等待 Tor bootstrap 完成后才能登录，并禁止 `.onion` 地址回退到明网。普通浏览器/PWA 不是受支持的生产客户端。

## 身份验证、消息签名与离线追赶

以下为当前源码中的未发布安全协议更新，版本号仍为 3.1.0；发布状态见[更新日志](changelog.md)。

添加联系人后，先在线下约定并在本机设置、解锁文本外观密码。**文本外观密码是检验对方是否可信任的凭证，文本外观密码只在线下交流，不要以任何形式发到网上。** 验证消息使用该密码通过 AES-GCM 加密固定文本 `paperphone.app`，绑定双方身份、安全号码和随机挑战，再对加密信封进行 Ed25519 签名。双方都通过验证并确认对方收到自己的挑战后，个人资料昵称才显示绿色 ✅。保留原有 12 组安全号码算法，号码一致仍需通过可信线下渠道核对。

打开私聊个人信息页会刷新身份密钥、重新检查本地信任记录，并从历史中恢复验证回执；密码已解锁时自动发起或继续验证。重复打开保留未完成的挑战并限制重试频率。密钥变化会撤销可信状态，必须重新线下核对并手动验证，不会自动替换可信密钥。页面重新获得焦点或恢复可见时也会刷新。

所有新群消息都要求签名和 Sender Key 端到端加密，不能关闭；旧群升级时迁移。分发群密钥前须先通过私聊验证各成员身份。打开群成员列表会刷新成员密钥与可信状态，不会批量发起私聊验证。群消息签名绑定群、发送者、密钥代次、消息 ID、类型、身份密钥、昵称、随机数和密文。只有可信密钥的签名正确才显示绿色 ✅；若尚未收到解密密钥，内容仍显示锁定并等待重试。

加入、退群或移除成员都会递增群密钥代次。在线成员收到通知后立即轮换各自的发送密钥，离线成员重连及发送前同步新代次；服务端拒绝旧代次发送和密钥发布。离群成员无法继续收发群消息或领取密钥，重新加入也不能补取离群期间的消息。其他成员仍可使用归档密钥读取合法历史；无法远程删除离群者已经持有的旧密钥或明文。

离线追赶按账号持久化 `server_seq` 游标，分页到完成，并保存待解密密文、按历史密钥版本重试。群历史按不可变序号分页，没有固定的历史总条数上限；本地每会话缓存最多 2,000 条，自动删除期限仍有效。此前已经丢失的密钥不能恢复。

当前签名路径使用 X25519、XSalsa20-Poly1305 与 Ed25519。旧 API 字段 `kem_pub` 实际承载签名公钥，不能据此宣称已实现 ML-KEM 后量子密钥协商。群成员关系和消息排序由服务端管理，并非 MLS 共识协议。服务端仍处理账号、群成员、路由等元数据；上传的附件文件本身不因消息签名而获得端到端加密。

服务端与所有平台客户端必须协调升级；旧客户端无法识别新签名信封。这里更新的是共享源码，不代表原生客户端或 Docker 镜像已经发布。参见[部署与迁移](DEPLOY_CN.md)、[测试说明](client/tests/README.md)。

## 应用发布

- iOS 客户端计划通过 Apple App Store 发布；可将用户提供的 Bark 地址用于后台提醒，PaperPhoneLite 客户端本身不注册 APNs token。
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
