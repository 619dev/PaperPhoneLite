# PaperPhoneLite 3.1.0

[简体中文](README.md) · [English](README_EN.md) · [日本語](README_JA.md) · [한국어](README_KO.md) · [Français](README_FR.md) · [Deutsch](README_DE.md) · [Русский](README_RU.md) · [Español](README_ES.md)

[Changelog](changelog.md)

[![Rust](https://img.shields.io/badge/Rust-1.83+-orange)](#) [![Axum](https://img.shields.io/badge/Axum-0.8-black)](#) [![React](https://img.shields.io/badge/React-19-blue)](#) [![TypeScript](https://img.shields.io/badge/TypeScript-5.7-blue)](#) [![MySQL](https://img.shields.io/badge/MySQL-8.0-blue)](#) [![Redis](https://img.shields.io/badge/Redis-7.x-red)](#) [![Tor](https://img.shields.io/badge/Tor-v3-7D4698?logo=tor-project)](#) [![Version](https://img.shields.io/badge/Version-3.1.0-orange)](client/package.json) [![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](LICENSE)

PaperPhoneLite is a lightweight end-to-end encrypted messenger built with React/TypeScript and Rust/Axum. It keeps private/group chat, attachments, voice messages, reliable synchronization, offline cache, expiring messages, Android ntfy and iOS Bark background alerts, QR codes, 2FA, and blocking.

## Features

| Feature | Description |
|---------|-------------|
| 🔐 End-to-End Encryption | X25519 and XSalsa20-Poly1305 message encryption, Ed25519 signatures and safety numbers |
| 🗝️ Encryption scope | Message bodies are encrypted; the server still processes service metadata and stores uploaded files |
| 🎭 Text appearance and extra encryption | An optional device-local password and eight text appearances with manual and background auto-lock |
| 🎤 Voice messages and effects | Voice recording plus local 0.8x, 1.0x, and 1.2x voice effects |
| 📨 Reliable synchronization | Bidirectional WebSocket heartbeat, persistent outbox, idempotent client IDs, and server-sequence catch-up |
| 📴 Offline access | Account-isolated contacts, groups, up to 2,000 messages per conversation, and media caching; queued sends retry when online |
| 👥 Group chat | Up to 2,000 members, mandatory signatures and Sender Key encryption, Do Not Disturb, notices, and member management |
| 💬 Rich messaging | Text, images, video, documents, voice, emoji, Telegram stickers, read receipts, and typing indicators |
| ⏱️ Expiring messages | Never, 1 day, 3 days, 7 days, or 30 days; configurable by either DM participant and by group owners |
| 🔔 Background notifications | Android can use ntfy; iOS can use a user-supplied Bark endpoint; the server allows only `api.day.app` by default and accepts self-hosted hosts through `BARK_ALLOWED_HOSTS` |
| 🔑 Two-factor authentication | Google Authenticator-compatible TOTP with eight one-time recovery codes |
| 📷 QR invitations | Add friends or join groups by QR code, with expiring group invitations |
| 📤 Server-local files | Files up to 500MB remain on a persistent volume; user/friend and group avatars are permanent, private/group chat attachments expire according to server configuration, and legacy uploads migrate automatically |
| 🦅 Tor onion service | Compose maps a Tor v3 onion service directly to the Rust backend while MySQL and Redis remain private |
| 🌐 Eight languages | Chinese, English, Japanese, Korean, French, German, Russian, and Spanish |

Version 3.0.0 removes Moments, Timeline, all direct/group voice and video calls, LiveKit, and Cloudflare R2. Files remain on a persistent server volume and are transferred by the Rust server. Self-hosting uses no Nginx: a Tor v3 onion service maps directly to the backend while MySQL and Redis remain private.

At startup, the server automatically drops the retired Web Push `push_subscriptions` and OneSignal `onesignal_players` tables, including obsolete identifiers left by older installations. Current releases do not recreate or use them.

After Docker Compose starts, the Tor container automatically records the complete `http://...onion` service URL in the host file `logs/onion-address.log`; it is also available through `docker compose logs tor`. See [DEPLOY_EN.md](DEPLOY_EN.md).

`client/` contains the shared UI source for Android, iOS, Windows, and macOS native shells. Production clients must embed Tor, wait for Tor bootstrap before authentication, and never fall back to clearnet for `.onion` addresses. Ordinary browsers and PWAs are not supported production clients.

## Identity verification, signatures and offline catch-up

These security protocol changes are **unreleased source changes**; the version remains 3.1.0. See the [changelog](changelog.md).

Before verifying a new contact, agree on the text appearance password offline, then set and unlock it locally. **The text appearance password is the credential for checking whether the other person is trustworthy. Exchange it only offline; never send it online in any form.** Verification encrypts exactly `paperphone.app` with AES-GCM, binds identities, the existing 12-group safety number and fresh challenges, and signs the encrypted envelope with Ed25519. Both parties must validate the proof and acknowledge the other's challenge before the profile nickname receives a green ✅. Compare safety numbers through a trusted offline channel.

Opening a private profile refreshes published keys, checks local trust pins and recovers verification receipts from history. An unlocked password automatically starts or resumes verification. Repeated opens preserve pending challenges and throttle retries. Changed keys revoke trust and require an explicit new verification after an offline check. Focus and visibility changes also refresh the view.

Every new group message requires a signature and Sender Key end-to-end encryption; existing groups migrate and encryption cannot be disabled. Verify each participant through the private handshake before distributing keys. Opening the member list refreshes keys and trust without initiating private handshakes for everyone. Signatures bind the group, sender, epoch, message ID, type, identity keys, nickname, nonce and ciphertext. A valid signature from a trusted key produces the message's green ✅; content can remain locked while its decryption key is pending.

Joining, leaving or removing a member advances the group epoch. Online members immediately rotate their own sender keys on notification; offline members refresh on reconnect and before sending. The server rejects old-epoch sends and publications. Departed members cannot send or fetch new messages or keys; rejoining does not grant access to the absence interval. Continuing recipients retain historical key distributions. Keys and plaintext already held by a departed member cannot be remotely erased.

Catch-up persists an account-scoped `server_seq` cursor, paginates to completion, retains pending ciphertext and retries exact historical key versions. Group history uses immutable sequence pagination with no fixed total-history cap. Local caching remains limited to 2,000 messages per conversation, and auto-delete retention still applies. Previously lost keys cannot be recreated.

The signed path uses X25519, XSalsa20-Poly1305 and Ed25519. The legacy API field `kem_pub` contains a signing public key, so this is not working ML-KEM post-quantum registration. Group membership and ordering remain server-authoritative rather than MLS consensus. Account, membership and routing metadata remain visible to the service; uploaded attachment bytes are not made end-to-end encrypted by message signing.

Coordinate the server and all platform clients: older clients cannot read the signed envelopes. Shared-source updates do not imply published native releases or Docker images. See [deployment and migration](DEPLOY_EN.md) and [security tests](client/tests/README.md).

## Application distribution

- The iOS client is intended for Apple App Store distribution and can send alerts through a user-supplied Bark endpoint; PaperPhoneLite itself does not register an APNs token.
- Android APKs are distributed only through the Android client repository's [GitHub Releases](https://github.com/619dev/ppl-android/releases), use ntfy, and will not be published on Google Play or include Google push/service frameworks.

## Client repositories

| Platform | Source code and releases |
|----------|--------------------------|
| Android | [619dev/ppl-android](https://github.com/619dev/ppl-android) |
| iOS | [619dev/ppl-ios](https://github.com/619dev/ppl-ios) |
| macOS | [619dev/ppl-mac](https://github.com/619dev/ppl-mac) |
| Windows | [619dev/ppl-win](https://github.com/619dev/ppl-win) |

See [DEPLOY_EN.md](DEPLOY_EN.md). License: AGPL-3.0.

---

If this project is useful to you, please buy me a can of cola.

<img width="30%" height="30%" src="请我喝可乐.jpg" alt="Donation QR code">

Telegram group: https://t.me/+vHJtvWJY_gEyMTUx
