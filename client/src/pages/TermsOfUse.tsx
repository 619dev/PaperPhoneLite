import type { ReactNode } from 'react'
import { ChevronLeft, Database, FileText, Lock, Network, Shield, UserX } from 'lucide-react'
import { useNavigate } from 'react-router-dom'
import { useI18n } from '../hooks/useI18n'

export default function TermsOfUse() {
  const navigate = useNavigate()
  const { lang } = useI18n()
  const zh = lang === 'zh'

  return (
    <div className="page">
      <div className="page-header">
        <button className="back-btn" onClick={() => navigate(-1)}><ChevronLeft size={20} /></button>
        <h1>{zh ? '使用条款' : 'Terms of Use'}</h1>
      </div>
      <div className="page-body privacy-page">
        <div className="privacy-hero">
          <Shield size={36} /><h2>PaperPhoneLite</h2>
          <p>{zh ? '轻量级、Tor-only 的端到端加密通讯工具' : 'Lightweight, Tor-only, end-to-end encrypted messaging'}</p>
        </div>
        <div className="privacy-updated">{zh ? '生效及更新日期：2026年8月18日' : 'Effective and updated: August 18, 2026'}</div>

        <Section icon={<FileText size={20} />} title={zh ? '1. 条款适用与接受' : '1. Acceptance and Scope'}>
          <p>{zh
            ? '下载、注册、登录或使用 PaperPhoneLite，即表示您已阅读并同意本条款及隐私声明。如果您不同意，请停止使用并删除客户端。具体服务器可能由独立的自托管运营者提供；该运营者可以发布额外规则，但不得冒充本项目官方条款。'
            : 'By downloading, registering, signing in to, or using PaperPhoneLite, you agree to these Terms and the Privacy Policy. If you disagree, stop using and remove the client. A server may be run by an independent self-hosting operator who may publish additional rules but must not misrepresent them as official project terms.'}</p>
        </Section>

        <Section icon={<Network size={20} />} title={zh ? '2. 服务与发行范围' : '2. Service and Distribution'}>
          <p>{zh
            ? '本项目提供私聊、群聊、联系人、加密消息、语音消息、附件、消息同步、定时删除、二维码、两步验证和拉黑；Android 还可使用 ntfy 接收后台通知。不提供朋友圈、时间线、公开内容发布、举报审核后台、语音或视频通话。iOS 不使用 APNs，当前不提供系统后台远程通知；Android 与 iOS 均不集成 Apple 或 Google 推送服务。生产客户端必须内嵌 Tor，并仅连接由用户选择的 v3 .onion 服务地址。iOS 版本计划通过 Apple App Store 发布；Android APK 仅通过项目 GitHub Releases 发布，不通过 Google Play。'
            : 'The project provides private and group chat, contacts, encrypted messaging, voice messages, attachments, synchronization, expiring messages, QR features, two-factor authentication, and blocking; Android can additionally use ntfy for background notifications. It provides no social feed, timeline, public publishing, report-review dashboard, or voice/video calling. iOS does not use APNs and currently provides no system background remote notifications; neither Android nor iOS integrates Apple or Google push services. Production clients must embed Tor and connect only to a user-selected v3 .onion server. iOS distribution is intended for the Apple App Store, while Android APKs are distributed only through the project’s GitHub Releases and not Google Play.'}</p>
        </Section>

        <Section icon={<Lock size={20} />} title={zh ? '3. 账号、密钥与安全责任' : '3. Accounts, Keys, and Security'}>
          <p>{zh
            ? '您应保护密码、两步验证恢复码、设备和本地加密密钥。丢失密钥、额外消息密码或恢复码可能导致数据无法解密或账号无法恢复。请在首次敏感通信前核对安全号码。端到端加密降低内容暴露风险，但不能防止设备被控制、截图、收件人转发、恶意服务器元数据分析或错误配置。Tor 可隐藏网络来源，但不保证绝对匿名。'
            : 'You are responsible for safeguarding passwords, two-factor recovery codes, devices, and local encryption keys. Losing keys, an extra message password, or recovery codes may make data or an account unrecoverable. Verify safety numbers before sensitive communication. End-to-end encryption reduces content exposure but cannot prevent device compromise, screenshots, recipient forwarding, malicious-server metadata analysis, or misconfiguration. Tor conceals network origin but does not guarantee absolute anonymity.'}</p>
        </Section>

        <Section icon={<UserX size={20} />} title={zh ? '4. 可接受使用' : '4. Acceptable Use'}>
          <p>{zh
            ? '您不得使用本软件实施违法行为、欺诈、骚扰、威胁、恶意软件传播、未经授权的系统访问，或侵犯他人隐私、知识产权及其他合法权益。项目没有社交内容审核或举报审查服务；用户可拉黑联系人，自托管运营者仍可依法限制账号或服务访问。'
            : 'You must not use the software for unlawful conduct, fraud, harassment, threats, malware distribution, unauthorized system access, or infringement of privacy, intellectual property, or other rights. The project offers no social-content moderation or report-review service. Users may block contacts, and self-hosting operators may restrict accounts or access where lawful.'}</p>
        </Section>

        <Section icon={<Database size={20} />} title={zh ? '5. 数据、附件与删除' : '5. Data, Attachments, and Deletion'}>
          <p>{zh
            ? '账号和路由所需元数据、加密消息、服务器端附件，以及用户启用的 ntfy 订阅会保存在所选服务器。项目不创建或保存 APNs 设备令牌。文件不使用 Cloudflare R2，而保存在服务器持久卷并由服务器传输。定时删除、退出登录、清理本地缓存或删除账号的效果受离线设备、备份、收件人副本和服务器运营方式限制，不构成对所有副本即时、不可恢复删除的保证。上传内容前请确认您拥有必要权利。'
            : 'The selected server stores account and routing metadata, encrypted messages, server-side attachments, and user-enabled ntfy subscriptions. The project does not create or retain APNs device tokens. Files do not use Cloudflare R2; they remain on the server volume and are transferred by the server. Expiry, logout, cache clearing, or account deletion may be limited by offline devices, backups, recipient copies, and operator practices and does not guarantee immediate, irrecoverable deletion of every copy. Upload only content you have the right to use.'}</p>
        </Section>

        <Section icon={<Shield size={20} />} title={zh ? '6. 开源、自托管与第三方服务' : '6. Open Source, Self-hosting, and Third Parties'}>
          <p>{zh
            ? 'PaperPhoneLite 按 AGPL-3.0 开源许可证提供。自托管运营者负责服务器安全、Tor onion service、备份、保留期限、适用法律和通知配置。启用 ntfy 时，ntfy 服务商会按其条款处理通知所需数据；是否使用公共 ntfy 实例由运营者决定。本项目不使用 Apple APNs，也不控制或为其他第三方服务的行为承担责任。'
            : 'PaperPhoneLite is provided under the AGPL-3.0 license. Self-hosting operators are responsible for server security, the Tor onion service, backups, retention, applicable law, and notification configuration. When ntfy is enabled, its provider processes notification data under its own terms, and the operator chooses whether to use a public ntfy instance. The project does not use Apple APNs and does not control or accept responsibility for other third-party services.'}</p>
        </Section>

        <Section icon={<Shield size={20} />} title={zh ? '7. 可用性、免责声明与责任限制' : '7. Availability, Disclaimer, and Liability'}>
          <p>{zh
            ? '软件按“现状”和“可用”状态提供，不保证持续在线、无错误、消息必达、数据不丢失、绝对安全或适合特定用途。在法律允许的最大范围内，项目贡献者和分发者不对因使用、无法使用、数据丢失、安全事件或第三方服务导致的间接、附带或后果性损失负责。不可依法排除的责任不受本条影响。'
            : 'The software is provided “as is” and “as available,” without guarantees of uptime, error-free operation, delivery, preservation, absolute security, or fitness for a particular purpose. To the maximum extent permitted by law, contributors and distributors are not liable for indirect, incidental, or consequential loss arising from use, inability to use, data loss, security incidents, or third-party services. Liability that cannot lawfully be excluded remains unaffected.'}</p>
        </Section>

        <Section icon={<FileText size={20} />} title={zh ? '8. 变更、终止与联系' : '8. Changes, Termination, and Contact'}>
          <p>{zh
            ? '我们可随版本更新本条款，并在应用或仓库中公布。重大变更生效后继续使用即表示接受；您可以停止使用并请求所选服务器运营者删除账号。条款部分无效不影响其他部分。问题可发送至 4722522@gmail.com。'
            : 'These Terms may be updated with a release and published in the app or repository. Continued use after a material change takes effect signifies acceptance; you may stop using the app and ask the chosen server operator to delete your account. If one provision is unenforceable, the remainder stays effective. Questions may be sent to 4722522@gmail.com.'}</p>
        </Section>

        <div className="privacy-footer"><p>© {new Date().getFullYear()} FM619 Technolog LTD.</p></div>
      </div>
    </div>
  )
}

function Section({ icon, title, children }: { icon: ReactNode; title: string; children: ReactNode }) {
  return <div className="privacy-section"><div className="privacy-section-header">{icon}<h3>{title}</h3></div><div className="privacy-card">{children}</div></div>
}
