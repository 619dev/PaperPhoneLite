# PaperPhoneLite 3.0.21 部署

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

Android 可通过 `NTFY_BASE_URL`/`NTFY_TOKEN` 配置 ntfy，不需要 Google 服务。iOS 用户可在客户端粘贴 Bark 推送地址，通过 Bark App 接收后台提醒；默认只允许 Bark 官方主机 `api.day.app`。自建 Bark 时由服务器运营者使用逗号分隔的 `BARK_ALLOWED_HOSTS` 显式加入主机名。PaperPhoneLite 自身无需注册 APNs token，但 Bark iOS App 最终仍使用 Apple APNs。通知只发送发件人名称和通用的新消息提示，不发送消息正文。

iOS 客户端计划提交 Apple App Store；Android APK 仅上传至 [Android 客户端仓库的 GitHub Releases](https://github.com/619dev/ppl-android/releases)，不发布到 Google Play。

文件只写入 `uploads` 持久卷，并通过 Rust 服务端的 `/api/files/` 路由下载。用户/好友头像和群头像保存在 `permanent/`，私聊与群聊附件保存在 `temporary/`。通过 `.env` 中的 `CHAT_FILE_RETENTION_DAYS` 设置临时附件保留天数，默认 `14`；设为 `0` 可关闭自动清理。服务端启动后立即检查一次，随后每小时清理过期文件。

从旧版本升级时，服务端启动会自动扫描原先直接存放在 `UPLOAD_DIR` 根目录中的文件。数据库仍引用的用户头像和群头像迁入永久目录，其余旧上传文件迁入临时目录；旧 `/api/files/uploads/...` 地址仍可读取。迁移会移动文件，因此升级前必须备份 `uploads` 卷，并确认容器拥有该卷的写权限。新版镜像的入口脚本会自动创建目录、修复卷权限并以非 root 用户启动服务。

客户端会在应用内请求附件并调用手机系统的保存/分享面板，不应把 `.onion` 文件地址交给外部浏览器。升级后请重点验证好友头像、群头像、文件消息下载，以及重启后的文件可读性。

升级前备份数据库和全部持久卷。部署后验证 `/health`、WebSocket、文字/图片/语音/文件消息和重启后的文件可读性。

## 数据库自动维护

服务端每次启动都会幂等执行数据库结构维护。3.0.21 会自动删除 3.0.0 以前遗留、现已停用的 Web Push `push_subscriptions` 与 OneSignal `onesignal_players` 表及其内容。升级前如需审计这些旧推送标识，请先备份；删除后当前版本不会重建这两张表。
