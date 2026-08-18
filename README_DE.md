# PaperPhoneLite 3.0.0

PaperPhoneLite ist ein leichtgewichtiger Ende-zu-Ende-verschlüsselter Messenger ohne öffentliche Social-Funktionen.

## Funktionen

- Hybride X25519- und ML-KEM-768-Schlüsselvereinbarung, XSalsa20-Poly1305 und Sicherheitsnummern.
- Einzel- und Gruppenchats mit Sender-Key-Verschlüsselung für bis zu 2.000 Mitglieder.
- Text, Bilder, Videos, Dokumente, Sprachnachrichten, Emoji und Telegram-Sticker.
- Zuverlässige WebSocket-Synchronisierung, Offline-Warteschlange und kontogetrennter Cache.
- Automatische Nachrichtenlöschung nach 1, 3, 7 oder 30 Tagen.
- Freundschaftsanfragen, Notizen, Tags, Sperren und QR-Gruppeneinladungen.
- TOTP-Zwei-Faktor-Authentifizierung; Push ausschließlich über ntfy auf Android und APNs auf iOS.
- Dateien bis 500 MB ausschließlich auf dem persistenten Server-Volume.
- Acht Oberflächensprachen.

## Tor ist erforderlich

Produktionsclients für Android, iOS, Windows und macOS müssen Tor einbetten, vor der Anmeldung den Tor-Bootstrap abwarten und `.onion`-Verbindungen ohne Clearnet-Fallback über isolierte SOCKS5-Verbindungen führen. `client/` enthält nur den gemeinsam genutzten UI-Quellcode. Browser und PWAs sind keine unterstützten Produktionsclients.

Moments, Timeline, alle Audio-/Videoanrufe, LiveKit, Cloudflare R2, Meldungen und das Moderations-Dashboard wurden entfernt. Die Produktion wird ausschließlich mit Docker Compose und einem Tor-v3-Onion-Service betrieben. Siehe [DEPLOY_EN.md](DEPLOY_EN.md).

Die iOS-App ist für den Apple App Store vorgesehen. Android-APKs werden ausschließlich über [GitHub Releases](https://github.com/619dev/PaperPhoneLite/releases) verteilt, nicht über Google Play; Google-Push- und Service-Frameworks werden nicht integriert.

Wenn Ihnen das Projekt hilft, spendieren Sie mir bitte eine Dose Cola.

<img width="30%" height="30%" src="请我喝可乐.jpg" alt="Spenden-QR-Code">

Telegram-Gruppe: https://t.me/+vHJtvWJY_gEyMTUx

Lizenz: AGPL-3.0.
