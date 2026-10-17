import { getKeys } from '../crypto/keystore'
import { startIdentityVerification, refreshContactVerification, refreshIdentityTrust, IDENTITY_STATUS_KEYS, type IdentityStatus } from '../crypto/authenticatedMessaging'
import IdentityBadge from '../components/IdentityBadge'
import { useEffect, useRef, useState } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { get, post, put, del } from '../api/http'
import { useStore, refreshFriendIdentityKeys } from '../store'
import { useI18n } from '../hooks/useI18n'
import { deriveSafetyNumber } from '../crypto/safetyNumber'
import { Check, ChevronDown, ChevronLeft, ChevronRight, Copy, Fingerprint, MessageCircle, Pencil, ShieldCheck, Ban } from 'lucide-react'

export default function UserProfile() {
  const { id } = useParams<{ id: string }>()
  const { t } = useI18n()
  const navigate = useNavigate()
  const me = useStore(s => s.user)
  const friends = useStore(s => s.friends)
  const friend = friends.find(f => f.id === id)

  const [user, setUser] = useState<any>(null)
  const [loading, setLoading] = useState(true)
  const [verificationStatus, setVerificationStatus] = useState<IdentityStatus | 'refreshing' | 'error'>('refreshing')
  const [verificationRefreshing, setVerificationRefreshing] = useState(true)
  const refreshProfile = useRef<() => void>(() => {})

  // Remark
  const [remark, setRemark] = useState('')
  const [editingRemark, setEditingRemark] = useState(false)
  const [remarkInput, setRemarkInput] = useState('')

  // Safety number
  const [safetyNumber, setSafetyNumber] = useState('')
  const [showSafetyNumber, setShowSafetyNumber] = useState(false)
  const [copied, setCopied] = useState(false)

  // Block
  const blockedUsers = useStore(s => s.blockedUsers)
  const addBlockedUser = useStore(s => s.addBlockedUser)
  const removeBlockedUser = useStore(s => s.removeBlockedUser)
  const isBlocked = blockedUsers.includes(id || '')

  useEffect(() => {
    if (!id) return
    const owner = me?.id
    let active = true
    let fetching = false
    let refreshAfterUnlock = false
    let latestPeer: any = null
    let statusRevision = 0
    const current = () => active && useStore.getState().user?.id === owner
    setUser(null)
    setLoading(true)
    setSafetyNumber('')
    setRemark(friend?.remark || '')
    const refresh = async () => {
      if (fetching) return
      fetching = true
      ++statusRevision
      setVerificationRefreshing(true)
      setVerificationStatus('refreshing')
      try {
        const peer = await get(`/api/users/${id}`)
        if (!current()) return
        latestPeer = { ...peer, id }
        setUser(peer)
        setLoading(false)
        refreshFriendIdentityKeys([latestPeer])
        const status = await refreshContactVerification(latestPeer)
        if (current()) setVerificationStatus(status)
      } catch {
        if (current()) setVerificationStatus('error')
      } finally {
        fetching = false
        if (current()) {
          setLoading(false)
          setVerificationRefreshing(false)
          if (refreshAfterUnlock) { refreshAfterUnlock = false; void refresh() }
        }
      }
    }
    const onPasswordChanged = () => { if (fetching) refreshAfterUnlock = true; else void refresh() }
    const onVisible = () => { if (document.visibilityState === 'visible') void refresh() }
    const onTrustChanged = () => {
      if (fetching || !latestPeer) return
      const revision = ++statusRevision
      void refreshIdentityTrust(latestPeer).then(status => {
        if (current() && revision === statusRevision) setVerificationStatus(status)
      }).catch(() => { if (current() && revision === statusRevision) setVerificationStatus('error') })
    }
    refreshProfile.current = refresh
    void refresh()
    window.addEventListener('focus', refresh)
    window.addEventListener('paperphone:presentation-state-changed', onPasswordChanged)
    window.addEventListener('paperphone:identity-changed', onTrustChanged)
    document.addEventListener('visibilitychange', onVisible)
    return () => {
      active = false
      refreshProfile.current = () => {}
      window.removeEventListener('focus', refresh)
      window.removeEventListener('paperphone:presentation-state-changed', onPasswordChanged)
      window.removeEventListener('paperphone:identity-changed', onTrustChanged)
      document.removeEventListener('visibilitychange', onVisible)
    }
  }, [id, me?.id])

  // Compute from the real local identity and the freshly published peer key.
  useEffect(() => {
    const keys = getKeys()
    if (!user?.ik_pub || !keys) return
    let cancelled = false
    void deriveSafetyNumber(keys.ik_pub, user.ik_pub).then(value => {
      if (!cancelled) setSafetyNumber(value)
    }).catch(() => { if (!cancelled) setSafetyNumber('—') })
    return () => { cancelled = true }
  }, [user?.ik_pub, me?.id])

  const handleCopySafety = () => {
    navigator.clipboard.writeText(safetyNumber).then(() => {
      setCopied(true)
      setTimeout(() => setCopied(false), 2000)
    })
  }

  // Update remark from friend data
  useEffect(() => {
    if (friend?.remark) setRemark(friend.remark)
  }, [friend?.remark])

  const saveRemark = async () => {
    const val = remarkInput.trim() || null
    try {
      await put('/api/friends/remark', { friend_id: id, remark: val })
      setRemark(val || '')
      setEditingRemark(false)
      // Refresh friends list
      const f = await get('/api/friends')
      useStore.getState().setFriends(f)
    } catch {}
  }

  if (loading) return <div className="page"><div className="loading-spinner" /></div>
  if (!user) return <div className="page"><div className="page-header"><button className="back-btn" onClick={() => navigate(-1)}><ChevronLeft size={20} /></button><h1>{t('verify.profile_title')}</h1></div><div className="page-body"><p role="status">{t('verify.profile_error')}</p><button className="btn btn-secondary" onClick={() => refreshProfile.current()}>{t('verify.retry')}</button></div></div>

  const displayName = remark || user.nickname

  const handleBlock = async () => {
    if (!confirm(t('block.confirm_desc'))) return
    try {
      await post('/api/users/block', { user_id: id })
      addBlockedUser(id!)
      alert(t('block.success'))
    } catch {
      alert(t('block.failed'))
    }
  }

  const handleUnblock = async () => {
    try {
      await del(`/api/users/block/${id}`)
      removeBlockedUser(id!)
      alert(t('unblock.success'))
    } catch {
      alert(t('unblock.failed'))
    }
  }

  return (
    <div className="page" id="user-profile-page">
      <div className="page-header">
        <button className="back-btn" onClick={() => navigate(-1)}><ChevronLeft size={20} /></button>
        <h1>{displayName}<IdentityBadge id={id!} ik={user.ik_pub} sign={user.kem_pub} refreshing={verificationRefreshing || verificationStatus === 'error'} /></h1>
      </div>
      <div className="page-body">
        {/* Profile card */}
        <div style={{
          display: 'flex', flexDirection: 'column', alignItems: 'center',
          padding: '28px 16px 20px',
          background: 'var(--bg-card)',
          borderRadius: 16, margin: '8px 12px',
        }}>
          <div className="avatar avatar-lg" style={{ marginBottom: 12 }}>
            {user.avatar ? <img src={user.avatar} alt="" /> : user.nickname?.[0]?.toUpperCase()}
          </div>
          <div style={{ fontSize: 20, fontWeight: 700 }}>{displayName}<IdentityBadge id={id!} ik={user.ik_pub} sign={user.kem_pub} refreshing={verificationRefreshing || verificationStatus === 'error'} /></div>
          {remark && (
            <div style={{ fontSize: 13, color: 'var(--text-muted)' }}>
              {t('friend.original_name')}: {user.nickname}
            </div>
          )}
          <div style={{ fontSize: 14, color: 'var(--text-muted)', marginBottom: 4 }}>@{user.username}</div>
          <div style={{
            fontSize: 12, padding: '2px 10px', borderRadius: 10,
            background: user.is_online ? 'rgba(76,175,80,0.15)' : 'rgba(158,158,158,0.15)',
            color: user.is_online ? '#4caf50' : '#9e9e9e',
            marginBottom: 16,
          }}>
            {user.is_online ? '● ' + t('contacts.online') : '○ ' + t('contacts.offline')}
          </div>
          <div style={{ display: 'flex', gap: 12, width: '100%', maxWidth: 280 }}>
            <button className="btn btn-primary btn-full" onClick={() => navigate(`/chat/${id}`)}>
              <MessageCircle size={16} /> {t('chat.send')}
            </button>
          </div>
        </div>

        {/* Remark */}
        <div className="section-title" style={{ padding: '16px 16px 6px' }}>
          <Pencil size={14} /> {t('friend.remark')}
        </div>
        {editingRemark ? (
          <div style={{ display: 'flex', gap: 8, padding: '0 16px 12px' }}>
            <input
              className="input"
              value={remarkInput}
              onChange={e => setRemarkInput(e.target.value)}
              placeholder={t('friend.remark_placeholder')}
              maxLength={128}
              autoFocus
              style={{ flex: 1 }}
            />
            <button className="btn btn-sm btn-primary" onClick={saveRemark}>{t('common.save')}</button>
            <button className="btn btn-sm btn-secondary" onClick={() => setEditingRemark(false)}>{t('common.cancel')}</button>
          </div>
        ) : (
          <div
            className="settings-item"
            onClick={() => { setRemarkInput(remark); setEditingRemark(true) }}
            style={{ cursor: 'pointer' }}
          >
            <span className="label" style={{ color: remark ? 'var(--text-primary)' : 'var(--text-muted)' }}>
              {remark || t('friend.no_remark')}
            </span>
            <span className="arrow"><ChevronRight size={14} /></span>
          </div>
        )}

        <div style={{ margin: 16, padding: 16, background: 'var(--bg-card)', borderRadius: 12 }}>
          <strong>{t('verify.title')}</strong>
          <p role="status" aria-live="polite" style={{ fontSize: 13, color: verificationStatus === 'verified' ? '#22c55e' : 'var(--text-muted)' }}>{t(IDENTITY_STATUS_KEYS[verificationStatus])}</p>
          <p style={{ fontSize: 13, lineHeight: 1.6 }}>{t('verify.notice')}</p>
          <p style={{ fontSize: 13 }}>{t('verify.instructions')}</p>
          <button className="btn btn-primary" disabled={verificationRefreshing} onClick={async () => {
            try { await startIdentityVerification(id!); navigate(`/chat/${id}`) }
            catch (error) { alert((error as Error).message) }
          }}>{t('verify.send')}</button>
        </div>
        {/* Safety Number — E2E verification */}
        <div className="section-title" style={{ padding: '16px 16px 6px' }}>
          <ShieldCheck size={14} /> {t('safety.title')}
        </div>
        <div
          className="settings-item"
          onClick={() => setShowSafetyNumber(!showSafetyNumber)}
          style={{ cursor: 'pointer' }}
        >
          <span className="label" style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
            <Fingerprint size={14} />
            {t('safety.verify_encryption')}
          </span>
          <span className="arrow">{showSafetyNumber ? <ChevronDown size={14} /> : <ChevronRight size={14} />}</span>
        </div>

        {showSafetyNumber && safetyNumber && (
          <div style={{
            margin: '0 12px 8px', padding: 20, borderRadius: 16,
            background: 'var(--bg-card)',
            display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 14,
          }}>
            <div style={{
              fontSize: 13, color: 'var(--text-muted)', textAlign: 'center', lineHeight: 1.5,
            }}>
              {t('safety.description')}
            </div>

            {/* Safety number grid */}
            <div style={{
              background: 'var(--bg-primary)',
              borderRadius: 12, padding: '16px 20px',
              width: '100%', maxWidth: 300,
            }}>
              <div style={{
                fontFamily: '"SF Mono", "Fira Code", "Cascadia Code", monospace',
                fontSize: 18, lineHeight: 2.2,
                textAlign: 'center', letterSpacing: 2,
                color: 'var(--text-primary)',
              }}>
                {(() => {
                  const nums = safetyNumber.split(' ')
                  const rows: string[][] = []
                  for (let i = 0; i < nums.length; i += 4) rows.push(nums.slice(i, i + 4))
                  return rows.map((row, i) => <div key={i}>{row.join('  ')}</div>)
                })()}
              </div>
            </div>

            {/* Participants */}
            <div style={{ fontSize: 12, color: 'var(--text-muted)', textAlign: 'center' }}>
              {me?.nickname} ↔ {displayName}
            </div>

            {/* Copy button */}
            <button
              className="btn btn-sm btn-secondary"
              onClick={(e) => { e.stopPropagation(); handleCopySafety() }}
              style={{ minWidth: 140 }}
            >
              {copied ? <><Check size={12} /> {t('fingerprint.copied')}</> : <><Copy size={12} /> {t('fingerprint.copy')}</>}
            </button>

            {/* How to verify */}
            <div style={{
              fontSize: 12, color: 'var(--text-muted)', lineHeight: 1.6,
              background: 'var(--bg-primary)', borderRadius: 10, padding: 12, width: '100%',
            }}>
              <div style={{ fontWeight: 600, marginBottom: 4, color: 'var(--text-primary)' }}>
                {t('safety.how_to_verify')}
              </div>
              {t('safety.verify_steps')}
            </div>
          </div>
        )}

        {/* Local safety control */}
        <div className="settings-item" onClick={isBlocked ? handleUnblock : handleBlock} style={{ cursor: 'pointer' }}>
          <span className="label" style={{ display: 'flex', alignItems: 'center', gap: 6, color: isBlocked ? 'var(--accent)' : '#ef4444' }}>
            <Ban size={14} /> {isBlocked ? t('unblock.user') : t('block.user')}
          </span>
        </div>

      </div>
    </div>
  )
}
