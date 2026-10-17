# PaperPhoneLite 3.1.0

[简体中文](README.md) · [English](README_EN.md) · [日本語](README_JA.md) · [한국어](README_KO.md) · [Français](README_FR.md) · [Deutsch](README_DE.md) · [Русский](README_RU.md) · [Español](README_ES.md)

[Änderungsverlauf](changelog.md)

[![Rust](https://img.shields.io/badge/Rust-1.83+-orange)](#) [![Axum](https://img.shields.io/badge/Axum-0.8-black)](#) [![React](https://img.shields.io/badge/React-19-blue)](#) [![TypeScript](https://img.shields.io/badge/TypeScript-5.7-blue)](#) [![MySQL](https://img.shields.io/badge/MySQL-8.0-blue)](#) [![Redis](https://img.shields.io/badge/Redis-7.x-red)](#) [![Tor](https://img.shields.io/badge/Tor-v3-7D4698?logo=tor-project)](#) [![Version](https://img.shields.io/badge/Version-3.1.0-orange)](client/package.json) [![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](LICENSE)

PaperPhoneLite ist ein leichtgewichtiger Ende-zu-Ende-verschlüsselter Messenger ohne öffentliche Social-Funktionen.

<details>
<summary>📸 Screenshots (zum Aufklappen klicken)</summary>
<img width="30%" height="30%" src="screenshot/ui1.png" alt="Screenshot 1">
<img width="30%" height="30%" src="screenshot/ui2.png" alt="Screenshot 2">
<img width="30%" height="30%" src="screenshot/ui3.png" alt="Screenshot 3">
</details>

## Funktionen

- X25519 · XSalsa20-Poly1305 · Ed25519.
- Einzel- und Gruppenchats mit Sender-Key-Verschlüsselung für bis zu 2.000 Mitglieder.
- Text, Bilder, Videos, Dokumente, Sprachnachrichten, Emoji und Telegram-Sticker.
- Zuverlässige WebSocket-Synchronisierung, Offline-Warteschlange und kontogetrennter Cache.
- Automatische Nachrichtenlöschung nach 1, 3, 7 oder 30 Tagen.
- Freundschaftsanfragen, Notizen, Tags, Sperren und QR-Gruppeneinladungen.
- TOTP-Zwei-Faktor-Authentifizierung, optionale ntfy-Benachrichtigungen auf Android und benutzerdefinierte Bark-Endpunkte auf iOS. Standardmäßig erlaubt der Server nur `api.day.app`; selbst gehostete Bark-Hosts werden über `BARK_ALLOWED_HOSTS` freigegeben.
- Dateien bis 500 MB auf dem persistenten Server-Volume; Benutzer-/Freundes- und Gruppenbilder bleiben dauerhaft, Chat-Anhänge werden nach der serverseitig konfigurierten Frist gelöscht und ältere Uploads automatisch migriert.
- Acht Oberflächensprachen.

## Identitätsprüfung, Signaturen und Offline-Abgleich

Diese Protokolländerungen sind **noch nicht veröffentlicht**; die Version bleibt 3.1.0. Siehe [Änderungsprotokoll](changelog.md).

Vereinbaren Sie das Passwort für die Textdarstellung offline und richten Sie es lokal ein. **Dieses Passwort dient als Nachweis, ob die andere Person vertrauenswürdig ist. Tauschen Sie es ausschließlich offline aus und senden Sie es niemals in irgendeiner Form ins Internet.** Zur Prüfung wird exakt `paperphone.app` mit AES-GCM verschlüsselt, an beide Identitäten, die bisherige Sicherheitsnummer aus 12 Gruppen und zufällige Herausforderungen gebunden und mit Ed25519 signiert. Erst nach gegenseitigem Nachweis und Empfangsbestätigung der Herausforderungen erhält der Profilname ein grünes ✅. Vergleichen Sie Sicherheitsnummern über einen vertrauenswürdigen Offline-Kanal.

Beim Öffnen eines privaten Profils werden Schlüssel und lokale Vertrauenseinträge geprüft sowie Bestätigungen aus dem Verlauf wiederhergestellt. Ein entsperrtes Passwort startet die Prüfung automatisch oder setzt sie fort. Erneutes Öffnen bewahrt offene Herausforderungen und begrenzt Wiederholungen. Geänderte Schlüssel entziehen das Vertrauen und verlangen eine manuelle Prüfung nach Offline-Abgleich. Auch Fokus und erneute Sichtbarkeit aktualisieren den Zustand.

Alle neuen Gruppennachrichten benötigen Signaturen und Sender-Key-Verschlüsselung. Bestehende Gruppen werden migriert; die Verschlüsselung lässt sich nicht deaktivieren. Vor der Schlüsselverteilung muss jedes Mitglied privat geprüft werden. Die Mitgliederliste aktualisiert Schlüssel und Vertrauen, ohne private Prüfungen für alle zu starten. Signaturen schützen Gruppe, Absender, Schlüsselgeneration, Nachrichten-ID, Typ, Identitätsschlüssel, Namen, Nonce und Geheimtext. Eine gültige Signatur eines vertrauenswürdigen Schlüssels ergibt ein grünes ✅; bei fehlendem Entschlüsselungsschlüssel bleibt der Inhalt gesperrt.

Beitritt, Austritt und Ausschluss erhöhen die Schlüsselgeneration. Online-Mitglieder erneuern nach der Benachrichtigung sofort ihre eigenen Sendeschlüssel; Offline-Mitglieder beim Wiederverbinden und vor dem Senden. Alte Generationen werden abgelehnt. Ausgetretene Mitglieder können keine neuen Nachrichten oder Schlüssel abrufen oder senden. Ein erneuter Beitritt eröffnet keinen Zugriff auf die Abwesenheitszeit. Verbleibende Empfänger behalten historische Schlüssel. Bereits vorhandene Schlüssel und Klartexte lassen sich auf fremden Geräten nicht fernlöschen.

Der Abgleich speichert `server_seq` je Konto, lädt sämtliche Seiten und wiederholt die Entschlüsselung mit historischen Schlüsselversionen. Der Gruppenverlauf nutzt unveränderliche Sequenzen ohne feste Gesamtobergrenze. Der lokale Cache bleibt auf 2.000 Nachrichten pro Gespräch begrenzt; automatische Löschfristen gelten weiterhin. Bereits verlorene Schlüssel sind nicht wiederherstellbar.

Verwendet werden X25519, XSalsa20-Poly1305 und Ed25519. Das alte API-Feld `kem_pub` enthält einen Signaturschlüssel, keine funktionierende postquantenfähige ML-KEM-Registrierung. Mitgliedschaft und Reihenfolge bestimmt der Server; dies ist kein MLS-Konsens. Metadaten und hochgeladene Anhangsdateien selbst werden durch Nachrichtensignaturen nicht Ende-zu-Ende-verschlüsselt.

Server und alle Plattform-Clients müssen gemeinsam aktualisiert werden. Ältere Clients verstehen die neuen Signaturformate nicht. Quellcodeänderungen bedeuten keine Veröffentlichung von Apps oder Docker-Images. Siehe [Migration](DEPLOY_EN.md) und [Tests](client/tests/README.md).

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
