# Tor architecture

[![Rust](https://img.shields.io/badge/Rust-1.83+-orange)](#) [![Axum](https://img.shields.io/badge/Axum-0.8-black)](#) [![React](https://img.shields.io/badge/React-19-blue)](#) [![TypeScript](https://img.shields.io/badge/TypeScript-5.7-blue)](#) [![MySQL](https://img.shields.io/badge/MySQL-8.0-blue)](#) [![Redis](https://img.shields.io/badge/Redis-7.x-red)](#) [![Tor](https://img.shields.io/badge/Tor-v3-7D4698?logo=tor-project)](#) [![Version](https://img.shields.io/badge/Version-3.0.16-orange)](client/package.json) [![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](LICENSE)

The production onion path is `native client -> embedded Tor SOCKS -> v3 onion service -> Rust server:3000`. Database and cache stay on the private Compose network. Uploaded bytes are written to the `uploads` volume and downloaded through server routes; there is no R2 or other object store.

The UI in `client/` is source for native shells, not a standalone production Web/PWA deployment. Every production client must start embedded Tor before authentication, reject clearnet fallback for `.onion` servers, isolate circuits per server/account, proxy HTTP and WebSocket traffic, and surface bootstrap state without leaking DNS queries. Platform-specific Tor implementations belong in the native projects.

Reference: https://simplex.chat/docs/server.html#tor-installation-and-configuration

On server startup, schema maintenance removes the obsolete Web Push `push_subscriptions` and OneSignal `onesignal_players` tables left by older installations. Current notification paths are ntfy on Android and user-configured Bark endpoints on iOS.
