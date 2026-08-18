import { useState, type ReactNode } from 'react'
import { useNavigate } from 'react-router-dom'
import { useI18n } from '../hooks/useI18n'
import { post } from '../api/http'
import { QRScanner } from '../components/QRCode'
import { ScanLine, ChevronRight } from 'lucide-react'

export default function Discover() {
  const { t } = useI18n()
  const navigate = useNavigate()
  const [scanning, setScanning] = useState(false)
  const [scanResult, setScanResult] = useState('')

  const items: Array<{ icon: ReactNode; label: string; action?: string; path?: string }> = [
    { icon: <ScanLine size={22} />, label: t('discover.scan'), action: 'scan' },
  ]

  const handleScan = async (data: string) => {
    setScanning(false)
    // Parse paperphonelite:// URIs
    if (data.startsWith('paperphonelite://friend/')) {
      const userId = data.replace('paperphonelite://friend/', '')
      if (userId) {
        // Send friend request then navigate to user profile
        try {
          await post('/api/friends/request', { friend_id: userId })
        } catch {}
        navigate(`/user/${userId}`)
      }
    } else if (data.startsWith('paperphonelite://invite/')) {
      const inviteId = data.replace('paperphonelite://invite/', '')
      if (inviteId) {
        try {
          const res = await post(`/api/groups/join/${inviteId}`)
          if (res.group_id) navigate(`/group/${res.group_id}`)
        } catch {
          setScanResult(t('discover.invite_expired'))
          setTimeout(() => setScanResult(''), 3000)
        }
      }
    } else if (data.startsWith('paperphonelite://group/')) {
      const groupId = data.replace('paperphonelite://group/', '')
      if (groupId) {
        navigate(`/group/${groupId}`)
      }
    } else {
      setScanResult(data)
      setTimeout(() => setScanResult(''), 3000)
    }
  }

  return (
    <div className="page" id="discover-page">
      <div className="page-header">
        <h1>{t('discover.title')}</h1>
      </div>
      <div className="page-body">
        {items.map((item, i) => (
          <div
            key={i}
            className="settings-item"
            onClick={() => {
              if (item.action === 'scan') setScanning(true)
              else if (item.path) navigate(item.path)
            }}
          >
            <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
              <span style={{ fontSize: 24 }}>{item.icon}</span>
              <span className="label">{item.label}</span>
            </div>
            <span className="arrow"><ChevronRight size={16} /></span>
          </div>
        ))}

        {scanResult && (
          <div style={{
            margin: '16px', padding: 12, borderRadius: 10,
            background: 'var(--bg-card)', fontSize: 13, color: 'var(--text-muted)',
            wordBreak: 'break-all',
          }}>
            {t('discover.scan_result')}: {scanResult}
          </div>
        )}
      </div>

      {scanning && (
        <QRScanner
          onScan={handleScan}
          onClose={() => setScanning(false)}
        />
      )}
    </div>
  )
}
