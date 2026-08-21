# PaperPhoneLite 3.0.12

[简体中文](README.md) · [English](README_EN.md) · [日本語](README_JA.md) · [한국어](README_KO.md) · [Français](README_FR.md) · [Deutsch](README_DE.md) · [Русский](README_RU.md) · [Español](README_ES.md)

[Changelog](changelog.md)

PaperPhoneLite is a lightweight end-to-end encrypted messenger built with React/TypeScript and Rust/Axum. It keeps private/group chat, attachments, voice messages, reliable synchronization, offline cache, expiring messages, Android ntfy background notifications, QR codes, 2FA, and blocking.

<details>
<summary>App screenshots</summary>

![PaperPhoneLite app screenshot 1](screenshot/ui1.png)
![PaperPhoneLite app screenshot 2](screenshot/ui2.png)
![PaperPhoneLite app screenshot 3](screenshot/ui3.png)

</details>

## Features

| Feature | Description |
|---------|-------------|
| 🔐 End-to-End Encryption | Hybrid X25519 + ML-KEM-768 key agreement and XSalsa20-Poly1305 message encryption with Signal-style safety number verification |
| 🗝️ Zero-knowledge encryption scope | The server stores encrypted conversation ciphertext while processing only the account, contact/group, routing, and push metadata needed to operate the service |
| 🎭 Text appearance and extra encryption | An optional device-local password and eight text appearances with manual and background auto-lock |
| 🎤 Voice messages and effects | Voice recording plus local 0.8x, 1.0x, and 1.2x voice effects |
| 📨 Reliable synchronization | Bidirectional WebSocket heartbeat, persistent outbox, idempotent client IDs, and server-sequence catch-up |
| 📴 Offline access | Account-isolated contacts, groups, up to 2,000 messages per conversation, and media caching; queued sends retry when online |
| 👥 Group chat | Up to 2,000 members, Sender Key encrypted mode, Do Not Disturb, notices, and member management |
| 💬 Rich messaging | Text, images, video, documents, voice, emoji, Telegram stickers, read receipts, and typing indicators |
| ⏱️ Expiring messages | Never, 1 day, 3 days, 7 days, or 30 days; configurable by either DM participant and by group owners |
| 🔔 Background notifications | Android can use ntfy; iOS does not use APNs and currently provides no system background remote notifications; no client integrates Apple or Google push frameworks |
| 🔑 Two-factor authentication | Google Authenticator-compatible TOTP with eight one-time recovery codes |
| 📷 QR invitations | Add friends or join groups by QR code, with expiring group invitations |
| 📤 Server-local files | Files up to 500MB remain on a persistent server volume; attachments are downloaded through the Rust server without opening object-storage or onion URLs in an external browser, and mobile clients can use the system save/share sheet |
| 🦅 Tor onion service | Compose maps a Tor v3 onion service directly to the Rust backend while MySQL and Redis remain private |
| 🌐 Eight languages | Chinese, English, Japanese, Korean, French, German, Russian, and Spanish |

Version 3.0.0 removes Moments, Timeline, all direct/group voice and video calls, LiveKit, and Cloudflare R2. Files remain on a persistent server volume and are transferred by the Rust server. Self-hosting uses no Nginx: a Tor v3 onion service maps directly to the backend while MySQL and Redis remain private.

After Docker Compose starts, the Tor container automatically records the complete `http://...onion` service URL in the host file `logs/onion-address.log`; it is also available through `docker compose logs tor`. See [DEPLOY_EN.md](DEPLOY_EN.md).

`client/` contains the shared UI source for Android, iOS, Windows, and macOS native shells. Production clients must embed Tor, wait for Tor bootstrap before authentication, and never fall back to clearnet for `.onion` addresses. Ordinary browsers and PWAs are not supported production clients.

## Application distribution

- The iOS client is intended for Apple App Store distribution; it does not use APNs and currently provides no system background remote notifications.
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
