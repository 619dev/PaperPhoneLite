# Tor architecture

The production onion path is `native client -> embedded Tor SOCKS -> v3 onion service -> Rust server:3000`. Database and cache stay on the private Compose network. Uploaded bytes are written to the `uploads` volume and downloaded through server routes; there is no R2 or other object store.

The Web client is deliberately outside this trust path. Native wrappers must start Tor before authentication, reject clearnet fallback for `.onion` servers, isolate circuits per server/account, proxy HTTP and WebSocket traffic, and surface bootstrap state without leaking DNS queries. Platform-specific implementations belong in the future native projects, not in `client/`.

Reference: https://simplex.chat/docs/server.html#tor-installation-and-configuration
