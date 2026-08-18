import { useEffect, useState } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { get, post, put, del } from '../api/http'
import { useStore } from '../store'
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
    setLoading(true)

    // Load user info
    get(`/api/users/${id}`).then(setUser).catch(() => {})

    // Set remark from friend store
    if (friend?.remark) setRemark(friend.remark)

    setLoading(false)
  }, [id])

  // Compute the number from both public keys as published by the server. Using
  // a local key on one side and a published key on the other made the two views
  // disagree whenever a restored long-lived session had stale local key state.
  useEffect(() => {
    if (!user?.ik_pub) return
    let cancelled = false
    get('/api/users/me')
      .then((currentUser: any) => {
        if (!cancelled && currentUser?.ik_pub) computeSafetyNumber(currentUser.ik_pub, user.ik_pub)
      })
      .catch(() => { if (!cancelled) setSafetyNumber('—') })
    return () => { cancelled = true }
  }, [user?.ik_pub])

  const computeSafetyNumber = async (myIkPub: string, theirIkPub: string) => {
    try {
      setSafetyNumber(await deriveSafetyNumber(myIkPub, theirIkPub))
    } catch {
      setSafetyNumber('—')
    }
  }

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

  if (loading || !user) return <div className="page"><div className="loading-spinner" /></div>

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
        <h1>{displayName}</h1>
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
          <div style={{ fontSize: 20, fontWeight: 700 }}>{displayName}</div>
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
