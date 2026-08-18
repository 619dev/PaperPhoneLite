# Tor architecture

The production onion path is `native client -> embedded Tor SOCKS -> v3 onion service -> Rust server:3000`. Database and cache stay on the private Compose network. Uploaded bytes are written to the `uploads` volume and downloaded through server routes; there is no R2 or other object store.

The UI in `client/` is source for native shells, not a standalone production Web/PWA deployment. Every production client must start embedded Tor before authentication, reject clearnet fallback for `.onion` servers, isolate circuits per server/account, proxy HTTP and WebSocket traffic, and surface bootstrap state without leaking DNS queries. Platform-specific Tor implementations belong in the native projects.

Reference: https://simplex.chat/docs/server.html#tor-installation-and-configuration
