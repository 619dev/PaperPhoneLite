# PaperPhoneLite 3.1.0 部署

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

服务端每次启动都会幂等执行数据库结构维护。3.1.0 会自动删除 3.0.0 以前遗留、现已停用的 Web Push `push_subscriptions` 与 OneSignal `onesignal_players` 表及其内容。升级前如需审计这些旧推送标识，请先备份；删除后当前版本不会重建这两张表。

## 签名协议协调升级（未发布）

当前源码中的安全更新尚未发布，版本号保持 3.1.0。服务端与 Android、iOS、macOS、Windows 客户端须协调升级；旧客户端不能读取新签名信封，也无法满足新的群发送要求。各原生仓库需要纳入共享源码后重新构建。拉取现有 `latest` 镜像不代表包含这些改动，应固定经过验证的镜像标签或摘要。

维护前备份一致的 MySQL 数据库、`.env` 和全部持久化卷；各客户端保留账号身份私钥、本地信任记录及历史发送密钥。迁移期间停止所有旧服务端写入实例，避免新旧实例混用不同的排序和撤销规则。

从仓库根目录构建当前源码：

```bash
docker build -t paperphone-lite-server:security-local ./server
```

在现有 `.env` 中设置 `SERVER_IMAGE=paperphone-lite-server:security-local`，保留其他配置。备份完成且旧写入实例停止后，启动依赖并替换服务端：

```bash
docker compose up -d mysql redis
docker compose up -d --no-deps --force-recreate server
docker compose up -d tor
docker compose logs --tail=100 server
```

本地标签无需执行 `docker compose pull`。确认数据库迁移成功及服务健康后，再允许升级后的客户端接入。构建镜像不会发布镜像。

### 数据库与群密钥迁移

启动时补齐群 `key_epoch`、`signed_protocol`，成员 `joined_epoch`，`identity_verification` 消息类型，群历史序号索引及保证已提交消息同步顺序的 `message_sync_barrier` 记录。已有群只迁移一次：强制启用加密与签名、增加代次、撤销当前发送密钥元数据。启动会检查必要状态，迁移失败时应检查日志，不能绕过检查。

所有发送者须在私聊验证各接收者后，为准确的当前成员集合发布新随机发送密钥，包含加密的自身副本。群加密不能关闭。加入、退群与移除在群行锁内增加代次，服务端拒绝旧代次发送及密钥发布。在线客户端收到通知立即轮换，离线客户端重连和发送前刷新。旧队列发送被拒绝时须处理失败状态，不能视作已送达。

保留供继续留群者读取的历史密钥分发，包括后来退群者发送的历史密钥；删除发给退群者的分发记录，并按当前成员资格与加入区间限制访问。已经持有的旧密钥或明文无法远程擦除。旧无签名群历史明确标注且不显示可信对勾；已固定身份的私聊会拒绝无签名消息。升级不能恢复此前已丢失的密钥。

### 验收与恢复

- 两个联系人先线下约定文本外观密码并在本机解锁，完成双方 `paperphone.app` 证明及挑战回执。重新打开资料页检查刷新状态；错误密码或身份变化不能自动建立信任。
- 打开群成员列表检查可信状态；分别移除在线与离线成员，确认剩余发送者更新代次和发送密钥，旧代次被拒绝。留群者可读合法历史，重新入群者不能补取离群期间消息。
- 离线后追赶多页及相同时间戳的消息，检查游标持久化、去重与历史密钥到达后的解密重试。自动删除和附件保留期仍生效，本地每会话最多缓存 2,000 条。
- 执行[安全与八语言检查](client/tests/README.md)，并验证原生 Tor 连接、资料和成员列表刷新、附件及重启持久化。Tor 连通不代表联系人身份已验证。

回退不能仅切换旧镜像：新数据库状态、签名消息和密钥代次可能不被旧客户端理解。恢复方案需匹配数据库备份、服务端及兼容客户端，并保留客户端密钥与 onion 服务卷；不能在写入活跃时覆盖数据库快照。

消息签名保护加密信封，不会加密已上传的附件文件本身，也不隐藏服务端处理的账号、成员和路由元数据。旧 `kem_pub` 字段实际是 Ed25519 签名公钥，本次未部署 ML-KEM 后量子注册或 MLS 成员共识。
