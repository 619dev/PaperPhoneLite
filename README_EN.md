# PaperPhoneLite 3.0.0

PaperPhoneLite is a lightweight end-to-end encrypted messenger built with React/TypeScript and Rust/Axum. It keeps private/group chat, attachments, voice messages, reliable synchronization, offline cache, expiring messages, push, QR codes, 2FA, reporting/blocking, and administration.

Version 3.0.0 removes Moments, Timeline, all direct/group voice and video calls, LiveKit, and Cloudflare R2. Files remain on a persistent server volume and are transferred by the Rust server. Self-hosting uses no Nginx: a Tor v3 onion service maps directly to the backend while MySQL and Redis remain private.

`client/` is the shared Web UI for future Android, iOS, Windows, and macOS shells. The Web build intentionally has no embedded Tor. Native shells must embed Tor and must never fall back to clearnet for onion addresses.

See [DEPLOY_EN.md](DEPLOY_EN.md). License: AGPL-3.0.
