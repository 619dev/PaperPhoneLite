# PaperPhoneLite 3.0.0 部署

## Onion-only 服务端（推荐）

服务器需要 Docker Engine、Compose v2 和可访问 Tor 网络的出站连接。不需要域名、TLS 证书或 Nginx，防火墙无需开放应用、MySQL、Redis 端口；只保留 SSH 管理端口，或使用你已有的带外管理方式。

```bash
cp server/.env.example .env
# 编辑 .env，至少设置 JWT_SECRET、DB_PASS、REDIS_PASS、MYSQL_ROOT_PASSWORD、ADMIN_PASSWORD
docker compose up -d --build
docker compose exec tor cat /var/lib/tor/paperphone-lite/hostname
```

最后一条命令输出服务端的 v3 `.onion` 地址。Tor 容器按 `HiddenServicePort 80 server:3000` 把 onion 虚拟端口直接转到 Rust 服务端；MySQL、Redis 和 3000 均不映射到宿主机。必须备份 `mysql_data`、`redis_data`、`uploads` 和 `tor_hidden_service` 四个卷，尤其是最后一个卷，丢失会导致 onion 地址变化。

原生 Android/iOS/Windows/macOS 客户端应内嵌 Tor，以 SOCKS5/隔离流方式访问 `http://<地址>.onion`。`client/` 是跨平台 UI 的 Web 基础代码，按要求不内嵌 Tor；普通浏览器也不会自动访问 onion。开发时可由开发者自行临时绑定本机后端端口，不要把该配置用于生产。

该模式参考 SimpleX 的 `HiddenServiceDir` 与 `HiddenServicePort` 配置，但 PaperPhoneLite 是 HTTP/WebSocket 应用，不是 SMP 服务器。不要启用 `HiddenServiceSingleHopMode`：onion-only 部署应保留标准三跳隐藏服务。

## Zeabur

导入 `zeabur.yaml` 会部署 client、server、MySQL、Redis，文件写入 server 持久卷。Zeabur 的 HTTP 服务需要公开平台域名，因此此方案不隐藏源站 IP；模板不虚假宣称 Tor 隐私。需要隐藏 IP 时使用上面的自托管 onion-only 方案。

升级前备份数据库和全部持久卷。部署后验证 `/health`、WebSocket、文字/图片/语音/文件消息和重启后的文件可读性。
