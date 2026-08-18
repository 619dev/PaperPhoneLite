import type { ReactNode } from 'react'
import { Baby, Bell, ChevronLeft, Database, Eye, Lock, Network, RefreshCw, Server, Shield, Trash2, Users } from 'lucide-react'
import { useNavigate } from 'react-router-dom'
import { useI18n } from '../hooks/useI18n'

export default function PrivacyPolicy() {
  const navigate = useNavigate()
  const { lang, t } = useI18n()
  const zh = lang === 'zh'

  return (
    <div className="page" id="privacy-policy-page">
      <div className="page-header">
        <button className="back-btn" onClick={() => navigate(-1)}><ChevronLeft size={20} /></button>
        <h1>{t('privacy.title')}</h1>
      </div>
      <div className="page-body privacy-policy-body">
        <div className="privacy-hero">
          <div className="privacy-hero-icon"><Shield size={40} /></div>
          <h2 className="privacy-hero-title">PaperPhoneLite</h2>
          <p className="privacy-hero-subtitle">{zh ? '隐私声明' : 'Privacy Policy'}</p>
        </div>
        <div className="privacy-updated">{zh ? '生效及更新日期：2026年8月18日' : 'Effective and updated: August 18, 2026'}</div>

        <Section icon={<Eye size={20} />} title={zh ? '1. 适用范围与责任主体' : '1. Scope and Data Controller'}>
          <p>{zh
            ? '本声明说明 PaperPhoneLite 客户端和服务端软件如何处理数据。PaperPhoneLite 是可自托管的开源项目；您选择连接的服务器运营者通常决定该实例的数据保留、备份、通知服务和法律合规，并可能是相关数据的实际控制者。请同时阅读该运营者公布的隐私规则。'
            : 'This Policy explains how the PaperPhoneLite client and server software process data. PaperPhoneLite is self-hostable open-source software. The operator of the server you select ordinarily determines that instance’s retention, backups, notification services, and legal compliance and may be the relevant data controller. Review any privacy terms published by that operator as well.'}</p>
        </Section>

        <Section icon={<Database size={20} />} title={zh ? '2. 我们处理的信息' : '2. Information Processed'}>
          <h4>{zh ? '账号与关系数据' : 'Account and relationship data'}</h4>
          <p>{zh
            ? '用户名、昵称、密码哈希、头像、两步验证状态、好友关系、备注、标签、拉黑关系、群组、成员和邀请信息。注册不强制要求手机号或电子邮箱。'
            : 'Username, nickname, password hash, avatar, two-factor status, friendships, remarks, tags, blocks, groups, memberships, and invitations. Registration does not require a phone number or email address.'}</p>
          <h4>{zh ? '通信和路由元数据' : 'Communication and routing metadata'}</h4>
          <p>{zh
            ? '发送方和接收方或群组标识、消息标识、类型、时间、投递/已读状态、客户端序号、自动删除设置、密钥版本，以及维持 WebSocket、同步和防重复投递所需的信息。项目没有朋友圈、时间线、公开帖子或举报审核数据。'
            : 'Sender and recipient or group identifiers, message identifiers, type, time, delivery/read status, client sequence, expiry settings, key version, and information needed for WebSocket operation, synchronization, and deduplication. The project has no social feed, timeline, public-post, or report-review data.'}</p>
          <h4>{zh ? '设备与会话数据' : 'Device and session data'}</h4>
          <p>{zh
            ? '设备名称和类型、操作系统、客户端或浏览器标识、会话标识、登录和最后活动时间、刷新令牌哈希，以及服务器可见的连接信息。Tor 用于降低来源 IP 暴露，但服务器、设备或推送服务仍可能产生其他可关联元数据。'
            : 'Device name and type, operating system, client or browser identifier, session identifier, login and last-active times, refresh-token hash, and connection information visible to the server. Tor reduces source-IP exposure, but the server, device, or push provider may still create other linkable metadata.'}</p>
        </Section>

        <Section icon={<Lock size={20} />} title={zh ? '3. 消息加密与限制' : '3. Message Encryption and Limits'}>
          <p>{zh
            ? '私聊使用 X25519 与 ML-KEM-768 混合密钥协商并以 XSalsa20-Poly1305 加密；加密群聊使用 Sender Key。身份私钥和 Sender Key 保存在本地设备，服务器存储并转发密文。可选的“消息隐私”密码在端到端加密前增加一层本地加密，密码不会上传或自动同步。'
            : 'Private chats use hybrid X25519 and ML-KEM-768 key agreement with XSalsa20-Poly1305 encryption; encrypted groups use Sender Keys. Identity private keys and Sender Keys stay on local devices while the server stores and relays ciphertext. The optional “message privacy” password adds local encryption before normal end-to-end encryption and is never uploaded or automatically synchronized.'}</p>
          <p>{zh
            ? '端到端加密不隐藏账号关系、群成员、时间、消息类型、大小和投递状态等运行所需元数据，也不能保护已被控制的设备、截图、收件人保存或转发的内容。安全号码应由通信双方独立核对。'
            : 'End-to-end encryption does not conceal operational metadata such as account relationships, group membership, time, message type, size, or delivery status, and cannot protect a compromised device, screenshots, or content saved or forwarded by a recipient. Participants should independently verify safety numbers.'}</p>
        </Section>

        <Section icon={<Server size={20} />} title={zh ? '4. 文件与数据存储' : '4. Files and Data Storage'}>
          <p>{zh
            ? '上传的图片、视频、语音和文档保存在您选择服务器的持久卷中，由 Rust 服务端传输，单文件上限为 500MB；本项目不使用 Cloudflare R2。客户端还会在 localStorage 和 IndexedDB 中保存登录状态、设置、本地密钥、联系人、群组、消息及媒体缓存。服务器运营者负责存储位置、备份、访问控制和保留期限。'
            : 'Uploaded images, video, voice, and documents remain on the selected server’s persistent volume and are transferred by the Rust server, with a 500MB per-file limit; the project does not use Cloudflare R2. The client also stores login state, settings, local keys, contacts, groups, messages, and media cache in localStorage and IndexedDB. The server operator is responsible for storage location, backups, access control, and retention.'}</p>
        </Section>

        <Section icon={<Network size={20} />} title={zh ? '5. Tor 网络' : '5. Tor Network'}>
          <p>{zh
            ? '生产客户端必须内嵌 Tor，等待 bootstrap 完成，并仅通过隔离的代理连接 v3 .onion 服务，不允许明网回退。Tor 旨在隐藏来源 IP 和服务器公开 IP，但无法保证绝对匿名，也无法消除设备指纹、账号行为、通知服务或用户主动披露形成的关联。Tor 软件和网络由 Tor Project 及其参与者独立提供。'
            : 'Production clients must embed Tor, wait for bootstrap, and connect only to a v3 .onion service through an isolated proxy with no clearnet fallback. Tor is intended to conceal source and public server IP addresses but cannot guarantee absolute anonymity or eliminate linkage from device fingerprints, account behavior, notification services, or voluntary disclosure. Tor software and the network are independently provided by the Tor Project and its participants.'}</p>
        </Section>

        <Section icon={<Bell size={20} />} title={zh ? '6. 通知服务与第三方' : '6. Notifications and Third Parties'}>
          <p>{zh
            ? 'Android 可使用 ntfy 接收后台通知：服务器会向运营者配置的 ntfy 实例发送主题、通知标题和正文；使用公共实例时，该实例运营者会按其政策处理这些数据。iOS 客户端不使用 Apple Push Notification service（APNs），当前也不提供系统后台远程通知；应用打开且保持连接时仍可显示应用内消息提醒。本项目不使用 APNs、Web Push、FCM、Firebase 或 OneSignal，也不集成 Apple 或 Google 的推送框架。ntfy 通知内容可能出现在锁屏，请在 ntfy App 和操作系统中调整预览设置。'
            : 'Android can use ntfy for background notifications: the server sends a topic, notification title, and body to the operator-configured ntfy instance, and a public instance processes that data under its operator’s policy. The iOS client does not use Apple Push Notification service (APNs) and currently provides no system background remote notifications; in-app message alerts can still appear while the app is open and connected. The project uses no APNs, Web Push, FCM, Firebase, or OneSignal and integrates neither Apple nor Google push frameworks. ntfy notification content may appear on a lock screen, so adjust preview settings in the ntfy app and operating system.'}</p>
        </Section>

        <Section icon={<Users size={20} />} title={zh ? '7. 使用目的与信息披露' : '7. Purposes and Disclosure'}>
          <p>{zh
            ? '数据仅用于创建和保护账号、连接联系人和群组、路由与同步消息、传输文件、提供已启用的通知、检测滥用、排查故障和履行法律义务。项目不出售个人信息，不提供广告或跨应用追踪。服务器运营者、启用 ntfy 时的实例运营者、基础设施供应商或依法提出要求的机构可能在各自角色和法律范围内接触相关数据；本项目不会为通知目的向 Apple 提交设备令牌或通知载荷。'
            : 'Data is used to create and secure accounts, connect contacts and groups, route and synchronize messages, transfer files, provide enabled notifications, detect abuse, troubleshoot, and comply with law. The project does not sell personal information and provides no advertising or cross-app tracking. A server operator, an ntfy instance operator when ntfy is enabled, an infrastructure provider, or a legally authorized authority may access relevant data within its role and applicable law. The project does not submit device tokens or notification payloads to Apple for notification delivery.'}</p>
        </Section>

        <Section icon={<Trash2 size={20} />} title={zh ? '8. 保留、删除与您的选择' : '8. Retention, Deletion, and Your Choices'}>
          <p>{zh
            ? '消息可设置永不删除或在 1、3、7、30 天后自动删除。您可以清理本地缓存、撤销设备会话、拉黑用户或提交账号删除请求。账号删除会触发服务器数据库中的关联记录删除，但离线设备、接收方副本、已启用的 ntfy 服务记录和服务器备份可能按各自周期继续存在。删除前请自行导出或备份需要的数据。具体访问、更正、删除或异议权取决于适用法律，应联系所选服务器运营者。'
            : 'Messages may be retained indefinitely or expire after 1, 3, 7, or 30 days. You may clear local cache, revoke device sessions, block users, or request account deletion. Account deletion triggers removal of related server database records, but offline devices, recipient copies, records held by an enabled ntfy service, and server backups may persist according to their own cycles. Export or back up anything needed before deletion. Rights of access, correction, deletion, or objection depend on applicable law and should be directed to the selected server operator.'}</p>
        </Section>

        <Section icon={<Baby size={20} />} title={zh ? '9. 未成年人' : '9. Children'}>
          <p>{zh
            ? 'PaperPhoneLite 不面向 13 岁以下儿童，也不会故意收集其个人信息。不同地区可能规定更高的数字同意年龄；未达到当地适用年龄的用户应在监护人同意和监督下使用，或停止使用。发现儿童数据时，请联系相应服务器运营者请求处理。'
            : 'PaperPhoneLite is not directed to children under 13 and does not knowingly collect their personal information. Some regions impose a higher age of digital consent; users below the locally applicable age should use the service only with guardian consent and supervision or stop using it. Contact the relevant server operator if child data is identified.'}</p>
        </Section>

        <Section icon={<RefreshCw size={20} />} title={zh ? '10. 安全、变更与联系' : '10. Security, Changes, and Contact'}>
          <p>{zh
            ? '我们采取加密、密码哈希、会话撤销和 Tor-only 连接等措施，但任何系统都无法保证绝对安全。本声明可能随版本更新；重大变更会在应用或仓库中公布。项目层面的问题可发送至 4722522@gmail.com；实例数据请求应优先联系您选择的服务器运营者。'
            : 'Measures include encryption, password hashing, session revocation, and Tor-only connections, but no system can guarantee absolute security. This Policy may change with a release; material changes will be published in the app or repository. Project-level questions may be sent to 4722522@gmail.com; instance-specific data requests should first go to the server operator you selected.'}</p>
        </Section>

        <div className="privacy-footer"><p>© {new Date().getFullYear()} FM619 Technolog LTD.</p></div>
      </div>
    </div>
  )
}

function Section({ icon, title, children }: { icon: ReactNode; title: string; children: ReactNode }) {
  return <div className="privacy-section"><div className="privacy-section-header">{icon}<h3>{title}</h3></div><div className="privacy-card">{children}</div></div>
}
