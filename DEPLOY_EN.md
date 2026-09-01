# PaperPhoneLite 3.0.16 deployment

## Onion-only server (recommended)

Install Docker Engine and Compose v2 on a host with outbound Tor connectivity. No domain, TLS certificate, or Nginx is required. Do not expose the application, MySQL, or Redis ports; keep only your SSH or out-of-band management path open.

```bash
cp server/.env.example .env
# Set JWT_SECRET, DB_PASS, REDIS_PASS and MYSQL_ROOT_PASSWORD
docker compose pull
docker compose up -d
cat logs/onion-address.log
```

After Tor creates or reads its v3 onion hostname, the container automatically writes the complete `http://...onion` URL to the host file `logs/onion-address.log` and also prints it in `docker compose logs tor`. No container shell is required. Tor maps onion virtual port 80 directly to `server:3000`. Back up the `mysql_data`, `redis_data`, `uploads`, and `tor_hidden_service` volumes. Losing the last volume changes the onion address.

Production Android, iOS, Windows, and macOS clients must embed Tor, reach `http://<address>.onion` through an isolated SOCKS5 circuit, and block authentication and network requests until Tor bootstrap completes. `client/` is shared UI source only; ordinary browsers and PWAs are not supported production clients.

This follows the SimpleX `HiddenServiceDir`/`HiddenServicePort` pattern, adapted to an HTTP/WebSocket application. Do not enable single-hop hidden services for an onion-only deployment.

## Notifications and client distribution

Android can use ntfy through `NTFY_BASE_URL`/`NTFY_TOKEN` without Google services. On iOS, users can paste a Bark endpoint in the client and receive background alerts in the Bark app. The official `api.day.app` host is allowed by default; self-hosted operators must explicitly add comma-separated hostnames with `BARK_ALLOWED_HOSTS`. PaperPhoneLite itself does not register an APNs token, although the Bark iOS app ultimately uses Apple APNs. Notifications contain only the sender name and a generic new-message alert, not message content.

The iOS client is intended for Apple App Store submission. Android APKs are uploaded only to the [Android client repository's GitHub Releases](https://github.com/619dev/ppl-android/releases) and are not published on Google Play.

Attachments are written only to the persistent `uploads` volume and downloaded through the Rust server's `/api/files/` route. Clients request files in-app and invoke the mobile system save/share sheet instead of handing `.onion` file URLs to an external browser. After upgrading, specifically verify file-message download and save behavior.

Back up all volumes before upgrades. Verify `/health`, WebSocket messaging, all attachment types, and file persistence after restart.

## Automatic database maintenance

The server performs idempotent schema maintenance on every startup. Version 3.0.16 automatically drops the retired Web Push `push_subscriptions` and OneSignal `onesignal_players` tables and their contents when they remain from pre-3.0.0 installations. Back up those obsolete identifiers first if they are needed for an audit; this release does not recreate either table.
