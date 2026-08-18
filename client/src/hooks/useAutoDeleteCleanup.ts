/**
 * useAutoDeleteCleanup — Background periodic cleanup of expired messages
 *
 * Runs every 60 seconds. For each chat in the Zustand store,
 * checks the corresponding friend/group auto_delete setting and
 * removes messages older than the cutoff from the local cache.
 */
import { useEffect } from 'react'
import { useStore } from '../store'

const CLEANUP_INTERVAL_MS = 60_000 // 60 seconds

export function useAutoDeleteCleanup() {
  useEffect(() => {
    const cleanup = () => {
      const { messages, friends, groups, setMessages } = useStore.getState()

      for (const chatId of Object.keys(messages)) {
        const msgs = messages[chatId]
        if (!msgs || msgs.length === 0) continue

        // Determine auto_delete setting for this chat
        const group = groups.find(g => g.id === chatId)
        const friend = friends.find(f => f.id === chatId)
        const autoDeleteSec = group
          ? (group.auto_delete ?? 0)
          : friend
            ? (friend.auto_delete ?? 0)
            : 0

        if (autoDeleteSec <= 0) continue // auto-delete disabled

        const cutoff = Date.now() - autoDeleteSec * 1000
        const filtered = msgs.filter(m => m.ts > cutoff)

        // Only update if messages were actually removed
        if (filtered.length < msgs.length) {
          setMessages(chatId, filtered)
        }
      }
    }

    // Run once immediately on mount
    cleanup()

    // Then run periodically
    const timer = setInterval(cleanup, CLEANUP_INTERVAL_MS)
    return () => clearInterval(timer)
  }, [])
}
