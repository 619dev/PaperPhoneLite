# PaperPhoneLite 3.0.0

PaperPhoneLite 是轻量级端到端加密即时通讯项目，使用 React 19/TypeScript Web 客户端与 Rust/Axum 服务端。

保留功能：私聊、群聊、文字/图片/视频/语音/文档消息、联系人、群组、消息同步、离线缓存、阅后定时删除、推送、二维码、2FA、举报/拉黑与管理后台。

3.0.0 已彻底移除朋友圈、时间线、单聊语音/视频通话、群聊语音/视频通话、LiveKit 与 Cloudflare R2。文件只写入服务端持久卷并由服务端传输。自托管模板不使用 Nginx，Tor v3 onion service 直接转发至 Rust 服务端；数据库和 Redis 不公开端口。

Web 代码位于 `client/`，供未来 Android、iOS、Windows、macOS 原生壳复用。Web 版本暂不内嵌 Tor；未来原生壳必须内嵌 Tor，并禁止 onion 地址回退到明网。

部署见 [DEPLOY_CN.md](DEPLOY_CN.md)，英文见 [README_EN.md](README_EN.md)。

```bash
cd client && npm install && npm run build
cd ../server && cargo check
```

许可证：AGPL-3.0。
