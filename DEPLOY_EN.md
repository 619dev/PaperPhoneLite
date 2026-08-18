# PaperPhoneLite 3.0.0 deployment

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

Among the current production clients, only Android supports background remote notifications. Configure ntfy with `NTFY_BASE_URL`/`NTFY_TOKEN`; no Google services are required. The iOS client does not use APNs and currently provides no system background remote notifications, although in-app message alerts remain available while the app is open and connected. Production deployments do not need APNs, Web Push, FCM, Firebase, or OneSignal configuration.

The iOS client is intended for Apple App Store submission. Android APKs are uploaded only to the [Android client repository's GitHub Releases](https://github.com/619dev/ppl-android/releases) and are not published on Google Play.

Back up all volumes before upgrades. Verify `/health`, WebSocket messaging, all attachment types, and file persistence after restart.
