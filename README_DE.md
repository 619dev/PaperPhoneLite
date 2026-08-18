# PaperPhoneLite 3.0.0

PaperPhoneLite ist ein leichtgewichtiger Ende-zu-Ende-verschlüsselter Messenger mit Einzel-/Gruppenchats, Anhängen, Sprachnachrichten, zuverlässiger Synchronisierung, Offline-Cache, Push, QR-Codes und 2FA.

Version 3.0.0 entfernt Moments, Timeline, alle Sprach-/Videoanrufe, LiveKit und Cloudflare R2. Dateien liegen ausschließlich auf einem persistenten Server-Volume. Beim Self-Hosting leitet ein Tor-v3-Onion-Service ohne Nginx direkt an den Rust-Server weiter; MySQL und Redis bleiben privat. Der Web-Client enthält absichtlich kein Tor; zukünftige native Clients müssen Tor einbetten und Clearnet-Fallback verhindern. Siehe [DEPLOY_EN.md](DEPLOY_EN.md).
