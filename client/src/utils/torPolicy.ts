const DEV_HOSTS = new Set(['localhost', '127.0.0.1', '[::1]'])

/** Production traffic is onion-only. Loopback is permitted solely in Vite dev mode. */
export function requireTorServer(rawUrl: string): string {
  const value = rawUrl.trim().replace(/\/+$/, '')
  if (!value) throw new Error('A Tor onion server address is required')

  let url: URL
  try {
    url = new URL(value)
  } catch {
    throw new Error('Enter a valid http://<address>.onion server address')
  }

  const onion = /^[a-z2-7]{56}\.onion$/.test(url.hostname.toLowerCase())
  const developmentLoopback = import.meta.env.DEV && DEV_HOSTS.has(url.hostname.toLowerCase())
  if (!onion && !developmentLoopback) {
    throw new Error('PaperPhoneLite requires a Tor .onion server; clearnet fallback is disabled')
  }
  if (url.username || url.password || url.search || url.hash) {
    throw new Error('The server address must not contain credentials, query parameters, or fragments')
  }
  if (url.protocol !== 'http:' && url.protocol !== 'https:') {
    throw new Error('The onion server address must use http:// or https://')
  }
  return value
}
