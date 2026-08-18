# PaperPhoneLite 3.0.0 deployment

## Onion-only server (recommended)

Install Docker Engine and Compose v2 on a host with outbound Tor connectivity. No domain, TLS certificate, or Nginx is required. Do not expose the application, MySQL, or Redis ports; keep only your SSH or out-of-band management path open.

```bash
cp server/.env.example .env
# Set JWT_SECRET, DB_PASS, REDIS_PASS and MYSQL_ROOT_PASSWORD
docker compose up -d --build
docker compose exec tor cat /var/lib/tor/paperphone-lite/hostname
```

Tor maps onion virtual port 80 directly to `server:3000`. Back up the `mysql_data`, `redis_data`, `uploads`, and `tor_hidden_service` volumes. Losing the last volume changes the onion address.

Production Android, iOS, Windows, and macOS clients must embed Tor, reach `http://<address>.onion` through an isolated SOCKS5 circuit, and block authentication and network requests until Tor bootstrap completes. `client/` is shared UI source only; ordinary browsers and PWAs are not supported production clients.

This follows the SimpleX `HiddenServiceDir`/`HiddenServicePort` pattern, adapted to an HTTP/WebSocket application. Do not enable single-hop hidden services for an onion-only deployment.

## Zeabur

Import `zeabur.yaml` only for a development or migration deployment of the server, MySQL, and Redis. It does not deploy a client. Zeabur must expose a platform HTTP domain and therefore does not satisfy the production Tor-only requirement.

Back up all volumes before upgrades. Verify `/health`, WebSocket messaging, all attachment types, and file persistence after restart.
