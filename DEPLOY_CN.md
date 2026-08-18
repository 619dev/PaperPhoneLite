# PaperPhoneLite 3.0.0 部署

## Onion-only 服务端（推荐）

服务器需要 Docker Engine、Compose v2 和可访问 Tor 网络的出站连接。不需要域名、TLS 证书或 Nginx，防火墙无需开放应用、MySQL、Redis 端口；只保留 SSH 管理端口，或使用你已有的带外管理方式。

```bash
cp server/.env.example .env
# 编辑 .env，至少设置 JWT_SECRET、DB_PASS、REDIS_PASS、MYSQL_ROOT_PASSWORD
docker compose up -d --build
docker compose exec tor cat /var/lib/tor/paperphone-lite/hostname
```

最后一条命令输出服务端的 v3 `.onion` 地址。Tor 容器按 `HiddenServicePort 80 server:3000` 把 onion 虚拟端口直接转到 Rust 服务端；MySQL、Redis 和 3000 均不映射到宿主机。必须备份 `mysql_data`、`redis_data`、`uploads` 和 `tor_hidden_service` 四个卷，尤其是最后一个卷，丢失会导致 onion 地址变化。

Android/iOS/Windows/macOS 生产客户端必须内嵌 Tor，以隔离 SOCKS5 流访问 `http://<地址>.onion`，并在 Tor bootstrap 完成前禁止登录和网络请求。`client/` 仅是跨平台 UI 源码；普通浏览器/PWA 不属于受支持的生产客户端。开发时可临时使用本机后端，不得将该方式用于生产。

该模式参考 SimpleX 的 `HiddenServiceDir` 与 `HiddenServicePort` 配置，但 PaperPhoneLite 是 HTTP/WebSocket 应用，不是 SMP 服务器。不要启用 `HiddenServiceSingleHopMode`：onion-only 部署应保留标准三跳隐藏服务。

## Zeabur

导入 `zeabur.yaml` 仅部署 server、MySQL 和 Redis，不部署客户端。Zeabur 必须公开 HTTP 域名，不符合生产环境的 Tor-only 要求，仅用于开发或数据迁移。

升级前备份数据库和全部持久卷。部署后验证 `/health`、WebSocket、文字/图片/语音/文件消息和重启后的文件可读性。
