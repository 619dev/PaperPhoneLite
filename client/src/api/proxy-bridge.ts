import { Capacitor, registerPlugin } from '@capacitor/core'
import type { ProxyConfig } from '../store'

/**
 * Bridge between the web-layer proxy config and the native Android ProxyPlugin.
 *
 * On native platforms (Android APK), this calls the ProxyPlugin via Capacitor
 * to configure the WebView's network proxy using AndroidX WebKit ProxyController.
 *
 * On web platforms (browser), this is a no-op — browser fetch/WebSocket cannot
 * be proxied from JavaScript; users should configure their system or browser proxy.
 */

interface ProxyPluginInterface {
  applyProxy(opts: {
    type: string
    host: string
    port: string
    username: string
    password: string
  }): Promise<{ success: boolean; proxy?: string; fallback?: boolean; message?: string }>
  clearProxy(): Promise<{ success: boolean }>
}

const ProxyPlugin = registerPlugin<ProxyPluginInterface>('ProxyPlugin')

/**
 * Apply the proxy configuration on native platforms.
 * No-op on web.
 */
export async function applyNativeProxy(config: ProxyConfig): Promise<void> {
  if (!Capacitor.isNativePlatform()) return

  if (!config.host || !config.port) {
    await clearNativeProxy()
    return
  }

  try {
    const result = await ProxyPlugin.applyProxy({
      type: config.type,
      host: config.host,
      port: config.port,
      username: config.username || '',
      password: config.password || '',
    })
    console.log('[Proxy] Applied:', result)
  } catch (err) {
    console.error('[Proxy] Failed to apply:', err)
  }
}

/**
 * Clear all proxy settings on native platforms.
 * No-op on web.
 */
export async function clearNativeProxy(): Promise<void> {
  if (!Capacitor.isNativePlatform()) return

  try {
    await ProxyPlugin.clearProxy()
    console.log('[Proxy] Cleared')
  } catch (err) {
    console.error('[Proxy] Failed to clear:', err)
  }
}

/**
 * Test latency through the currently configured proxy by timing a HEAD request
 * to the server URL. Returns latency in milliseconds, or -1 on failure.
 *
 * This works because on Android, once ProxyController applies the proxy,
 * all WebView fetch() calls automatically route through it.
 */
export async function testProxyLatency(serverUrl: string): Promise<number> {
  if (!serverUrl) return -1

  const start = performance.now()
  try {
    const controller = new AbortController()
    const timeout = setTimeout(() => controller.abort(), 10000) // 10s timeout
    await fetch(`${serverUrl}/api/ping`, {
      method: 'HEAD',
      cache: 'no-cache',
      signal: controller.signal,
    }).catch(() => {
      // /api/ping may 404, try base URL
      return fetch(serverUrl, {
        method: 'HEAD',
        cache: 'no-cache',
        signal: controller.signal,
      })
    })
    clearTimeout(timeout)
    return Math.round(performance.now() - start)
  } catch {
    return -1
  }
}
