# Tor architecture

[![Rust](https://img.shields.io/badge/Rust-1.83+-orange)](#) [![Axum](https://img.shields.io/badge/Axum-0.8-black)](#) [![React](https://img.shields.io/badge/React-19-blue)](#) [![TypeScript](https://img.shields.io/badge/TypeScript-5.7-blue)](#) [![MySQL](https://img.shields.io/badge/MySQL-8.0-blue)](#) [![Redis](https://img.shields.io/badge/Redis-7.x-red)](#) [![Tor](https://img.shields.io/badge/Tor-v3-7D4698?logo=tor-project)](#) [![Version](https://img.shields.io/badge/Version-3.1.0-orange)](client/package.json) [![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](LICENSE)

The production onion path is `native client -> embedded Tor SOCKS -> v3 onion service -> Rust server:3000`. Database and cache stay on the private Compose network. Uploaded bytes are written to the `uploads` volume and downloaded through server routes; there is no R2 or other object store. User/friend and group avatars use permanent storage, while private/group chat attachments use temporary storage governed by `CHAT_FILE_RETENTION_DAYS` (14 days by default; `0` disables cleanup). Startup automatically classifies legacy root-level uploads and the server cleans expired temporary files hourly.

The UI in `client/` is source for native shells, not a standalone production Web/PWA deployment. Every production client must start embedded Tor before authentication, reject clearnet fallback for `.onion` servers, isolate circuits per server/account, proxy HTTP and WebSocket traffic, and surface bootstrap state without leaking DNS queries. Platform-specific Tor implementations belong in the native projects.

Reference: https://simplex.chat/docs/server.html#tor-installation-and-configuration

On server startup, schema maintenance removes the obsolete Web Push `push_subscriptions` and OneSignal `onesignal_players` tables left by older installations. Current notification paths are ntfy on Android and user-configured Bark endpoints on iOS.

## Identity and message security over Tor

Tor provides the transport path; contact trust comes from the offline-password identity handshake and pinned signing keys. Never send the text appearance password online, including through Tor. The verification payload contains an AES-GCM-encrypted `paperphone.app` proof, contextual identity/safety-number/challenge bindings and an Ed25519 signature. Both sides need proof and challenge receipts before the profile badge becomes green.

Private profiles refresh identity keys and trust on opening and focus/visibility changes. The group member list refreshes member trust without initiating handshakes for every member. A group message receives a green badge only after its signature verifies against a trusted identity; it can still await its decryption key. Transport availability alone must not set a trusted badge.

New group messages require signatures and sender-key encryption. Membership changes advance a durable epoch and require every remaining sender to rotate its key. Online clients react to notifications; offline clients refresh on reconnect and before sending. Catch-up and historical key retrieval use the same onion HTTP/WebSocket path, account-scoped sequence cursors and membership checks. Reconnecting Tor or switching circuits must not clear identity pins, historical keys or sync cursors. Departed members cannot fetch new messages or keys; already-held historical keys cannot be remotely erased.

The server still knows account, group membership and routing metadata and controls membership/order; this is not MLS consensus. Uploaded files are server-local bytes, not newly end-to-end encrypted by the signature layer. The deployed signed path uses X25519, XSalsa20-Poly1305 and Ed25519; `kem_pub` is a legacy signing-key field, not working ML-KEM registration.

These are unreleased source changes. Coordinate upgrades of the server and all native platforms; an existing Docker `latest` tag or native release does not imply compatibility. See [English deployment](DEPLOY_EN.md), [中文部署](DEPLOY_CN.md), and [security tests](client/tests/README.md). Local cache and retention limits still apply to offline history.
