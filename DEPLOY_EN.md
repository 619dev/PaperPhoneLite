# PaperPhoneLite 3.0.0 deployment

## Onion-only server (recommended)

Install Docker Engine and Compose v2 on a host with outbound Tor connectivity. No domain, TLS certificate, or Nginx is required. Do not expose the application, MySQL, or Redis ports; keep only your SSH or out-of-band management path open.

```bash
cp server/.env.example .env
# Set JWT_SECRET, DB_PASS, REDIS_PASS, MYSQL_ROOT_PASSWORD and ADMIN_PASSWORD
docker compose up -d --build
docker compose exec tor cat /var/lib/tor/paperphone-lite/hostname
```

Tor maps onion virtual port 80 directly to `server:3000`. Back up the `mysql_data`, `redis_data`, `uploads`, and `tor_hidden_service` volumes. Losing the last volume changes the onion address.

Future native Android, iOS, Windows, and macOS shells must embed Tor and reach `http://<address>.onion` through an isolated SOCKS5 circuit. The `client/` directory is the shared Web UI and intentionally does not embed Tor; ordinary browsers do not automatically support onion services.

This follows the SimpleX `HiddenServiceDir`/`HiddenServicePort` pattern, adapted to an HTTP/WebSocket application. Do not enable single-hop hidden services for an onion-only deployment.

## Zeabur

Import `zeabur.yaml` to deploy the client, server, MySQL, and Redis with server-local persistent uploads. Zeabur must expose a platform HTTP domain, so it does not hide the origin IP. Use the Compose deployment above when origin hiding is required.

Back up all volumes before upgrades. Verify `/health`, WebSocket messaging, all attachment types, and file persistence after restart.
