# PaperPhoneLite 3.0.12 部署

## Onion-only 服务端（推荐）

服务器需要 Docker Engine、Compose v2 和可访问 Tor 网络的出站连接。不需要域名、TLS 证书或 Nginx，防火墙无需开放应用、MySQL、Redis 端口；只保留 SSH 管理端口，或使用你已有的带外管理方式。

```bash
cp server/.env.example .env
# 编辑 .env，至少设置 JWT_SECRET、DB_PASS、REDIS_PASS、MYSQL_ROOT_PASSWORD
docker compose pull
docker compose up -d
cat logs/onion-address.log
```

Tor 容器生成或读取 v3 onion hostname 后，会自动把完整的 `http://...onion` 地址写入宿主机的 `logs/onion-address.log`，并同步输出到 `docker compose logs tor`。无需进入容器即可读取该文件。Tor 按 `HiddenServicePort 80 server:3000` 把 onion 虚拟端口直接转到 Rust 服务端；MySQL、Redis 和 3000 均不映射到宿主机。必须备份 `mysql_data`、`redis_data`、`uploads` 和 `tor_hidden_service` 四个卷，尤其是最后一个卷，丢失会导致 onion 地址变化。

Android/iOS/Windows/macOS 生产客户端必须内嵌 Tor，以隔离 SOCKS5 流访问 `http://<地址>.onion`，并在 Tor bootstrap 完成前禁止登录和网络请求。`client/` 仅是跨平台 UI 源码；普通浏览器/PWA 不属于受支持的生产客户端。开发时可临时使用本机后端，不得将该方式用于生产。

该模式参考 SimpleX 的 `HiddenServiceDir` 与 `HiddenServicePort` 配置，但 PaperPhoneLite 是 HTTP/WebSocket 应用，不是 SMP 服务器。不要启用 `HiddenServiceSingleHopMode`：onion-only 部署应保留标准三跳隐藏服务。

## 通知与客户端发布

当前生产客户端只有 Android 可使用后台远程通知，通过 `NTFY_BASE_URL`/`NTFY_TOKEN` 配置 ntfy，不需要 Google 服务。iOS 客户端不使用 APNs，当前不提供系统后台远程通知；应用打开并保持连接时仍可显示应用内消息提醒。生产部署无需配置 APNs、Web Push、FCM、Firebase 或 OneSignal。

iOS 客户端计划提交 Apple App Store；Android APK 仅上传至 [Android 客户端仓库的 GitHub Releases](https://github.com/619dev/ppl-android/releases)，不发布到 Google Play。

附件只写入 `uploads` 持久卷，并通过 Rust 服务端的 `/api/files/` 路由下载。客户端会在应用内请求文件并调用手机系统的保存/分享面板，不应把 `.onion` 文件地址交给外部浏览器。升级后请重点验证文件消息点击下载与保存。

升级前备份数据库和全部持久卷。部署后验证 `/health`、WebSocket、文字/图片/语音/文件消息和重启后的文件可读性。
