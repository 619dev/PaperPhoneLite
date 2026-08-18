# PaperPhoneLite 3.0.0

PaperPhoneLite est une messagerie légère chiffrée de bout en bout avec discussions privées/de groupe, pièces jointes, messages vocaux, synchronisation fiable, cache hors ligne, notifications, QR et 2FA.

La version 3.0.0 supprime Moments, Timeline, tous les appels audio/vidéo, LiveKit et Cloudflare R2. Les fichiers restent uniquement sur un volume persistant du serveur. L’auto-hébergement utilise un service onion Tor v3 sans Nginx, directement relié au serveur Rust ; MySQL et Redis restent privés. Le client Web n’intègre volontairement pas Tor ; les futurs clients natifs devront l’intégrer sans repli clearnet. Voir [DEPLOY_EN.md](DEPLOY_EN.md).
