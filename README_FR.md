# PaperPhoneLite 3.1.0

[简体中文](README.md) · [English](README_EN.md) · [日本語](README_JA.md) · [한국어](README_KO.md) · [Français](README_FR.md) · [Deutsch](README_DE.md) · [Русский](README_RU.md) · [Español](README_ES.md)

[Historique des modifications](changelog.md)

[![Rust](https://img.shields.io/badge/Rust-1.83+-orange)](#) [![Axum](https://img.shields.io/badge/Axum-0.8-black)](#) [![React](https://img.shields.io/badge/React-19-blue)](#) [![TypeScript](https://img.shields.io/badge/TypeScript-5.7-blue)](#) [![MySQL](https://img.shields.io/badge/MySQL-8.0-blue)](#) [![Redis](https://img.shields.io/badge/Redis-7.x-red)](#) [![Tor](https://img.shields.io/badge/Tor-v3-7D4698?logo=tor-project)](#) [![Version](https://img.shields.io/badge/Version-3.1.0-orange)](client/package.json) [![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](LICENSE)

PaperPhoneLite est une messagerie légère chiffrée de bout en bout, sans fonctions sociales publiques.

<details>
<summary>📸 Captures d’écran (cliquer pour afficher)</summary>
<img width="30%" height="30%" src="screenshot/ui1.png" alt="Capture d’écran 1">
<img width="30%" height="30%" src="screenshot/ui2.png" alt="Capture d’écran 2">
<img width="30%" height="30%" src="screenshot/ui3.png" alt="Capture d’écran 3">
</details>

## Fonctionnalités

- X25519 · XSalsa20-Poly1305 · Ed25519.
- Discussions privées et groupes chiffrés par Sender Key jusqu’à 2 000 membres.
- Texte, images, vidéos, documents, messages vocaux, emoji et stickers Telegram.
- Synchronisation WebSocket fiable, file hors ligne et cache isolé par compte.
- Suppression automatique des messages après 1, 3, 7 ou 30 jours.
- Demandes d’amis, remarques, étiquettes, blocage et invitations de groupe par QR.
- 2FA TOTP, notifications ntfy facultatives sur Android et points de terminaison Bark fournis par l’utilisateur sur iOS. Le serveur n’autorise que `api.day.app` par défaut ; les hôtes Bark auto-hébergés sont ajoutés via `BARK_ALLOWED_HOSTS`.
- Fichiers jusqu’à 500 Mo sur le volume persistant du serveur ; les avatars utilisateur/ami et de groupe sont permanents, les pièces jointes expirent selon la configuration du serveur et les anciens fichiers sont migrés automatiquement.
- Huit langues d’interface.

## Vérification d’identité, signatures et rattrapage hors ligne

Ces modifications du protocole sont **non publiées** ; la version reste 3.1.0. Voir le [journal](changelog.md).

Convenez hors ligne du mot de passe d’apparence du texte, puis configurez-le et déverrouillez-le sur votre appareil. **Ce mot de passe sert à vérifier que votre interlocuteur est digne de confiance. Échangez-le uniquement hors ligne ; ne l’envoyez jamais sur Internet, sous quelque forme que ce soit.** La vérification chiffre exactement `paperphone.app` avec AES-GCM, lie les identités, le numéro de sécurité existant en 12 groupes et des défis aléatoires, puis signe l’enveloppe chiffrée avec Ed25519. Le profil affiche un ✅ vert seulement après validation mutuelle des preuves et accusés de réception des défis. Comparez les numéros de sécurité par un canal hors ligne fiable.

Ouvrir un profil privé actualise les clés, contrôle les identités épinglées et récupère les confirmations dans l’historique. Le mot de passe déverrouillé permet de démarrer ou reprendre automatiquement la vérification. Les réouvertures préservent le défi et limitent les tentatives. Un changement de clé révoque la confiance et exige une nouvelle vérification manuelle après contrôle hors ligne. Le retour au premier plan ou à une page visible actualise aussi l’état.

Tous les nouveaux messages de groupe exigent une signature et le chiffrement Sender Key ; les anciens groupes migrent et le chiffrement ne peut plus être désactivé. Vérifiez chaque membre en privé avant de distribuer les clés. La liste des membres actualise les clés et la confiance sans lancer de vérifications privées collectives. La signature couvre groupe, expéditeur, génération de clé, ID, type, clés d’identité, nom, nonce et texte chiffré. Une signature valide avec une clé fiable donne un ✅ vert ; le contenu reste verrouillé si la clé de déchiffrement manque.

Une arrivée, un départ ou une exclusion augmente la génération des clés. Les membres en ligne renouvellent immédiatement leurs clés d’envoi sur notification ; les membres hors ligne le font à la reconnexion et avant l’envoi. Le serveur refuse les anciens envois et publications de clés. Un ancien membre ne peut plus envoyer ni récupérer de nouveaux messages ou clés ; rejoindre à nouveau ne donne pas accès à la période d’absence. Les destinataires restants conservent les clés historiques. Les clés et textes déjà détenus par une personne partie ne peuvent pas être effacés à distance.

Le rattrapage conserve un curseur `server_seq` par compte, parcourt toutes les pages et réessaie avec les versions historiques des clés. L’historique de groupe utilise des séquences immuables sans plafond fixe du total ; le cache local reste limité à 2 000 messages par conversation. L’expiration reste applicable et les clés déjà perdues ne peuvent pas être recréées.

Le protocole utilise X25519, XSalsa20-Poly1305 et Ed25519. Le champ historique `kem_pub` contient une clé de signature, pas une inscription ML-KEM postquantique fonctionnelle. Le serveur gère l’appartenance et l’ordre des messages, sans consensus MLS. Les métadonnées et les fichiers joints téléversés eux-mêmes ne deviennent pas chiffrés de bout en bout grâce aux signatures.

Mettez à niveau le serveur et tous les clients ensemble : les anciens clients ne lisent pas les nouvelles enveloppes. La mise à jour des sources n’annonce pas une publication des applications ou images Docker. Voir [migration](DEPLOY_EN.md) et [tests](client/tests/README.md).

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
