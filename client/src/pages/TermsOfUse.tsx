import type { ReactNode } from 'react'
import { ChevronLeft, Lock, MessageCircle, Shield, UserX } from 'lucide-react'
import { useNavigate } from 'react-router-dom'
import { useI18n } from '../hooks/useI18n'

export default function TermsOfUse() {
  const navigate = useNavigate()
  const { lang } = useI18n()
  const isZH = lang === 'zh'

  return (
    <div className="page">
      <div className="page-header">
        <button className="back-btn" onClick={() => navigate(-1)}><ChevronLeft size={20} /></button>
        <h1>{isZH ? '使用条款' : 'Terms of Use'}</h1>
      </div>
      <div className="page-body privacy-page">
        <div className="privacy-hero">
          <Shield size={36} /><h2>PaperPhoneLite</h2>
          <p>{isZH ? '轻量端到端加密通讯工具' : 'Lightweight end-to-end encrypted communication'}</p>
        </div>
        <Section icon={<MessageCircle size={20} />} title={isZH ? '1. 服务范围' : '1. Service Scope'}>
          <p>{isZH ? 'PaperPhoneLite 提供私聊、群聊和文件传输。本项目不提供朋友圈、时间线或其他公开社交发布功能。' : 'PaperPhoneLite provides private chat, group chat, and file transfer. It does not provide Moments, Timeline, or other public social publishing features.'}</p>
        </Section>
        <Section icon={<Lock size={20} />} title={isZH ? '2. 隐私与加密' : '2. Privacy and Encryption'}>
          <p>{isZH ? '用户应妥善保管账号、密码和加密密钥。服务端会处理消息路由所需的账号与会话元数据；加密消息正文以密文传输和保存。' : 'Users must protect their accounts, passwords, and encryption keys. The server processes metadata required for routing; encrypted message bodies are transferred and stored as ciphertext.'}</p>
        </Section>
        <Section icon={<UserX size={20} />} title={isZH ? '3. 用户责任与拉黑' : '3. User Responsibility and Blocking'}>
          <p>{isZH ? '用户不得利用本服务从事违法活动、骚扰或欺诈。用户可在客户端拉黑其他用户，阻止后续消息交互。' : 'Users must not use the service for illegal activity, harassment, or fraud. Users can block other users to prevent further message interaction.'}</p>
        </Section>
        <Section icon={<Shield size={20} />} title={isZH ? '4. 服务可用性' : '4. Service Availability'}>
          <p>{isZH ? '自托管运营者对服务可用性、备份、本地文件存储和 Tor 服务配置负责。软件按开源许可证和现状提供。' : 'Self-hosting operators are responsible for availability, backups, local file storage, and Tor configuration. The software is provided under its open-source license and as-is.'}</p>
        </Section>
        <div className="privacy-footer"><p>© {new Date().getFullYear()} FM619 Technolog LTD.</p></div>
      </div>
    </div>
  )
}

function Section({ icon, title, children }: { icon: ReactNode; title: string; children: ReactNode }) {
  return <div className="privacy-section"><div className="privacy-section-header">{icon}<h3>{title}</h3></div><div className="privacy-card">{children}</div></div>
}
