# PaperPhoneLite 3.0.16

[简体中文](README.md) · [English](README_EN.md) · [日本語](README_JA.md) · [한국어](README_KO.md) · [Français](README_FR.md) · [Deutsch](README_DE.md) · [Русский](README_RU.md) · [Español](README_ES.md)

[Änderungsverlauf](changelog.md)

[![Rust](https://img.shields.io/badge/Rust-1.83+-orange)](#) [![Axum](https://img.shields.io/badge/Axum-0.8-black)](#) [![React](https://img.shields.io/badge/React-19-blue)](#) [![TypeScript](https://img.shields.io/badge/TypeScript-5.7-blue)](#) [![MySQL](https://img.shields.io/badge/MySQL-8.0-blue)](#) [![Redis](https://img.shields.io/badge/Redis-7.x-red)](#) [![Tor](https://img.shields.io/badge/Tor-v3-7D4698?logo=tor-project)](#) [![Version](https://img.shields.io/badge/Version-3.0.16-orange)](client/package.json) [![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](LICENSE)

PaperPhoneLite ist ein leichtgewichtiger Ende-zu-Ende-verschlüsselter Messenger ohne öffentliche Social-Funktionen.

<details>
<summary>App-Screenshots</summary>

![PaperPhoneLite App-Screenshot 1](screenshot/ui1.png)
![PaperPhoneLite App-Screenshot 2](screenshot/ui2.png)
![PaperPhoneLite App-Screenshot 3](screenshot/ui3.png)

</details>

## Funktionen

- Hybride X25519- und ML-KEM-768-Schlüsselvereinbarung, XSalsa20-Poly1305 und Sicherheitsnummern.
- Einzel- und Gruppenchats mit Sender-Key-Verschlüsselung für bis zu 2.000 Mitglieder.
- Text, Bilder, Videos, Dokumente, Sprachnachrichten, Emoji und Telegram-Sticker.
- Zuverlässige WebSocket-Synchronisierung, Offline-Warteschlange und kontogetrennter Cache.
- Automatische Nachrichtenlöschung nach 1, 3, 7 oder 30 Tagen.
- Freundschaftsanfragen, Notizen, Tags, Sperren und QR-Gruppeneinladungen.
- TOTP-Zwei-Faktor-Authentifizierung, optionale ntfy-Benachrichtigungen auf Android und benutzerdefinierte Bark-Endpunkte auf iOS. Standardmäßig erlaubt der Server nur `api.day.app`; selbst gehostete Bark-Hosts werden über `BARK_ALLOWED_HOSTS` freigegeben.
- Dateien bis 500 MB ausschließlich auf dem persistenten Server-Volume; Anhänge werden über den Rust-Server heruntergeladen, ohne Objekt-Speicher- oder Onion-URLs im externen Browser zu öffnen, und können über den Systemdialog des Mobilgeräts gespeichert werden.
- Acht Oberflächensprachen.

## Tor ist erforderlich

Produktionsclients für Android, iOS, Windows und macOS müssen Tor einbetten, vor der Anmeldung den Tor-Bootstrap abwarten und `.onion`-Verbindungen ohne Clearnet-Fallback über isolierte SOCKS5-Verbindungen führen. `client/` enthält nur den gemeinsam genutzten UI-Quellcode. Browser und PWAs sind keine unterstützten Produktionsclients.

Moments, Timeline, alle Audio-/Videoanrufe, LiveKit, Cloudflare R2, Meldungen und das Moderations-Dashboard wurden entfernt. Die Produktion wird ausschließlich mit Docker Compose und einem Tor-v3-Onion-Service betrieben. Siehe [DEPLOY_EN.md](DEPLOY_EN.md).

Beim Start entfernt der Server automatisch die stillgelegten Web-Push- und OneSignal-Tabellen samt veralteten Kennungen aus älteren Installationen.

Nach dem Start von Docker Compose schreibt der Tor-Container die vollständige `http://...onion`-Adresse automatisch in `logs/onion-address.log` auf dem Host; sie erscheint auch in `docker compose logs tor`.

Die iOS-App ist für den Apple App Store vorgesehen. Android-APKs werden ausschließlich über die [GitHub Releases](https://github.com/619dev/ppl-android/releases) des Android-Client-Repositorys verteilt, nicht über Google Play; Google-Push- und Service-Frameworks werden nicht integriert.

## Client-Repositorys

| Plattform | Quellcode und Releases |
|-----------|------------------------|
| Android | [619dev/ppl-android](https://github.com/619dev/ppl-android) |
| iOS | [619dev/ppl-ios](https://github.com/619dev/ppl-ios) |
| macOS | [619dev/ppl-mac](https://github.com/619dev/ppl-mac) |
| Windows | [619dev/ppl-win](https://github.com/619dev/ppl-win) |

Wenn Ihnen das Projekt hilft, spendieren Sie mir bitte eine Dose Cola.

<img width="30%" height="30%" src="请我喝可乐.jpg" alt="Spenden-QR-Code">

Telegram-Gruppe: https://t.me/+vHJtvWJY_gEyMTUx

Lizenz: AGPL-3.0.
