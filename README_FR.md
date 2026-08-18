# PaperPhoneLite 3.0.0

[简体中文](README.md) · [English](README_EN.md) · [日本語](README_JA.md) · [한국어](README_KO.md) · [Français](README_FR.md) · [Deutsch](README_DE.md) · [Русский](README_RU.md) · [Español](README_ES.md)

[Historique des modifications](changelog.md)

PaperPhoneLite est une messagerie légère chiffrée de bout en bout, sans fonctions sociales publiques.

## Fonctionnalités

- Accord de clés hybride X25519 + ML-KEM-768, XSalsa20-Poly1305 et numéros de sécurité.
- Discussions privées et groupes chiffrés par Sender Key jusqu’à 2 000 membres.
- Texte, images, vidéos, documents, messages vocaux, emoji et stickers Telegram.
- Synchronisation WebSocket fiable, file hors ligne et cache isolé par compte.
- Suppression automatique des messages après 1, 3, 7 ou 30 jours.
- Demandes d’amis, remarques, étiquettes, blocage et invitations de groupe par QR.
- 2FA TOTP et notifications exclusivement via ntfy sur Android et APNs sur iOS.
- Fichiers jusqu’à 500 Mo conservés uniquement sur le volume persistant du serveur.
- Huit langues d’interface.

## Tor est obligatoire

Les clients de production Android, iOS, Windows et macOS doivent intégrer Tor, attendre la fin du démarrage de Tor avant l’authentification et utiliser des circuits SOCKS5 isolés pour `.onion`, sans repli clearnet. `client/` contient uniquement le code source partagé de l’interface. Les navigateurs et PWA ne sont pas des clients de production pris en charge.

Moments, Timeline, tous les appels, LiveKit, Cloudflare R2, les signalements et le tableau de modération ont été supprimés. La production utilise exclusivement Docker Compose et un service onion Tor v3. Voir [DEPLOY_EN.md](DEPLOY_EN.md).

Après le démarrage de Docker Compose, le conteneur Tor enregistre automatiquement l’adresse complète `http://...onion` dans le fichier hôte `logs/onion-address.log` ; elle apparaît aussi dans `docker compose logs tor`.

L’application iOS est destinée à l’Apple App Store. Les APK Android sont publiés uniquement dans les [GitHub Releases](https://github.com/619dev/PaperPhoneLite/releases), jamais sur Google Play, sans framework de services ou de push Google.

Si ce projet vous est utile, offrez-moi une canette de cola.

<img width="30%" height="30%" src="请我喝可乐.jpg" alt="QR code de don">

Groupe Telegram : https://t.me/+vHJtvWJY_gEyMTUx

Licence : AGPL-3.0.
