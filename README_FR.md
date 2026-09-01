# PaperPhoneLite 3.0.16

[简体中文](README.md) · [English](README_EN.md) · [日本語](README_JA.md) · [한국어](README_KO.md) · [Français](README_FR.md) · [Deutsch](README_DE.md) · [Русский](README_RU.md) · [Español](README_ES.md)

[Historique des modifications](changelog.md)

[![Rust](https://img.shields.io/badge/Rust-1.83+-orange)](#) [![Axum](https://img.shields.io/badge/Axum-0.8-black)](#) [![React](https://img.shields.io/badge/React-19-blue)](#) [![TypeScript](https://img.shields.io/badge/TypeScript-5.7-blue)](#) [![MySQL](https://img.shields.io/badge/MySQL-8.0-blue)](#) [![Redis](https://img.shields.io/badge/Redis-7.x-red)](#) [![Tor](https://img.shields.io/badge/Tor-v3-7D4698?logo=tor-project)](#) [![Version](https://img.shields.io/badge/Version-3.0.16-orange)](client/package.json) [![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](LICENSE)

PaperPhoneLite est une messagerie légère chiffrée de bout en bout, sans fonctions sociales publiques.

<details>
<summary>Captures d’écran de l’application</summary>

![Capture d’écran 1 de l’application PaperPhoneLite](screenshot/ui1.png)
![Capture d’écran 2 de l’application PaperPhoneLite](screenshot/ui2.png)
![Capture d’écran 3 de l’application PaperPhoneLite](screenshot/ui3.png)

</details>

## Fonctionnalités

- Accord de clés hybride X25519 + ML-KEM-768, XSalsa20-Poly1305 et numéros de sécurité.
- Discussions privées et groupes chiffrés par Sender Key jusqu’à 2 000 membres.
- Texte, images, vidéos, documents, messages vocaux, emoji et stickers Telegram.
- Synchronisation WebSocket fiable, file hors ligne et cache isolé par compte.
- Suppression automatique des messages après 1, 3, 7 ou 30 jours.
- Demandes d’amis, remarques, étiquettes, blocage et invitations de groupe par QR.
- 2FA TOTP, notifications ntfy facultatives sur Android et points de terminaison Bark fournis par l’utilisateur sur iOS. Le serveur n’autorise que `api.day.app` par défaut ; les hôtes Bark auto-hébergés sont ajoutés via `BARK_ALLOWED_HOSTS`.
- Fichiers jusqu’à 500 Mo conservés uniquement sur le volume persistant du serveur ; les pièces jointes sont téléchargées via le serveur Rust sans ouvrir d’URL de stockage objet ou `.onion` dans un navigateur externe et peuvent être enregistrées avec le panneau système du téléphone.
- Huit langues d’interface.

## Tor est obligatoire

Les clients de production Android, iOS, Windows et macOS doivent intégrer Tor, attendre la fin du démarrage de Tor avant l’authentification et utiliser des circuits SOCKS5 isolés pour `.onion`, sans repli clearnet. `client/` contient uniquement le code source partagé de l’interface. Les navigateurs et PWA ne sont pas des clients de production pris en charge.

Moments, Timeline, tous les appels, LiveKit, Cloudflare R2, les signalements et le tableau de modération ont été supprimés. La production utilise exclusivement Docker Compose et un service onion Tor v3. Voir [DEPLOY_EN.md](DEPLOY_EN.md).

Au démarrage, le serveur supprime automatiquement les anciennes tables Web Push et OneSignal ainsi que les identifiants obsolètes des installations précédentes.

Après le démarrage de Docker Compose, le conteneur Tor enregistre automatiquement l’adresse complète `http://...onion` dans le fichier hôte `logs/onion-address.log` ; elle apparaît aussi dans `docker compose logs tor`.

L’application iOS est destinée à l’Apple App Store. Les APK Android sont publiés uniquement dans les [GitHub Releases](https://github.com/619dev/ppl-android/releases) du dépôt du client Android, jamais sur Google Play, sans framework de services ou de push Google.

## Dépôts des clients

| Plateforme | Code source et versions |
|------------|-------------------------|
| Android | [619dev/ppl-android](https://github.com/619dev/ppl-android) |
| iOS | [619dev/ppl-ios](https://github.com/619dev/ppl-ios) |
| macOS | [619dev/ppl-mac](https://github.com/619dev/ppl-mac) |
| Windows | [619dev/ppl-win](https://github.com/619dev/ppl-win) |

Si ce projet vous est utile, offrez-moi une canette de cola.

<img width="30%" height="30%" src="请我喝可乐.jpg" alt="QR code de don">

Groupe Telegram : https://t.me/+vHJtvWJY_gEyMTUx

Licence : AGPL-3.0.
