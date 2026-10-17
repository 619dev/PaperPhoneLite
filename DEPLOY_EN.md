# PaperPhoneLite 3.1.0 deployment

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

Files are written only to the persistent `uploads` volume and downloaded through the Rust server's `/api/files/` route. User/friend and group avatars are stored under `permanent/`; private and group-chat attachments are stored under `temporary/`. Set `CHAT_FILE_RETENTION_DAYS` in `.env` to control temporary attachment retention. It defaults to `14`; `0` disables cleanup. The server checks once at startup and then removes expired files hourly.

When upgrading, server startup automatically scans files previously stored directly in the `UPLOAD_DIR` root. User and group avatars still referenced by the database move to permanent storage, while other legacy uploads move to temporary storage. Legacy `/api/files/uploads/...` URLs remain readable. This migration moves files, so back up the `uploads` volume and ensure it is writable before upgrading. The new image entrypoint creates the directories, repairs volume ownership, and starts the service as a non-root user.

Clients request attachments in-app and invoke the mobile system save/share sheet instead of handing `.onion` file URLs to an external browser. After upgrading, verify friend and group avatars, attachment downloads, and persistence across a restart.

Back up all volumes before upgrades. Verify `/health`, WebSocket messaging, all attachment types, and file persistence after restart.

## Automatic database maintenance

The server performs idempotent schema maintenance on every startup. Version 3.1.0 automatically drops the retired Web Push `push_subscriptions` and OneSignal `onesignal_players` tables and their contents when they remain from pre-3.0.0 installations. Back up those obsolete identifiers first if they are needed for an audit; this release does not recreate either table.

## Coordinated signed-protocol upgrade (unreleased)

The security update in the current source tree is not a published 3.1.0 release. Upgrade the server and every Android, iOS, macOS and Windows client together; older clients cannot read signed envelopes or meet new group-send requirements. The native repositories need builds incorporating these shared-source changes. Pulling an existing `latest` image does not establish that it contains this protocol. Pin a tested image tag or digest.

Before migration, back up a consistent MySQL database, `.env`, and all persistent volumes. Preserve account-local identity secrets, trust records and historical sender keys on each client. Stop all old server writers during the maintenance window; mixed old/new server instances do not enforce the same ordering and revocation rules.

To build this checkout locally, run from the repository root:

```bash
docker build -t paperphone-lite-server:security-local ./server
```

Set `SERVER_IMAGE=paperphone-lite-server:security-local` in the existing `.env`, preserving its other settings. With backups complete and old writers stopped, start the dependencies and replace the server:

```bash
docker compose up -d mysql redis
docker compose up -d --no-deps --force-recreate server
docker compose up -d tor
docker compose logs --tail=100 server
```

Do not run `docker compose pull` for the local-only server tag. Wait for server health and successful schema initialization before admitting upgraded clients. Building or documenting this image does not publish it.

### Schema and group-key migration

Startup adds group `key_epoch` and `signed_protocol`, member `joined_epoch`, the `identity_verification` message type, a group-history sequence index, and the `message_sync_barrier` row used for committed sync ordering. Existing groups enter mandatory encrypted/signed mode once, advance their epoch and invalidate current sender-key metadata. Required schema state is checked at startup; inspect errors instead of bypassing a failed migration.

Every sender must publish a fresh random key encrypted for exactly the current membership, including itself, after all recipient identities have been verified privately. Encryption cannot be disabled. Join, leave and removal advance the epoch under a group lock; the server refuses stale sends and publications. Connected clients rotate on notification; offline clients refresh on reconnect and before sending. A blocked or rejected old queued message must be reviewed rather than assumed delivered.

Historical distributions remain available to continuing recipients, including those authored by a departed sender. Distributions addressed to a departed member are removed; current membership and join intervals restrict future downloads. Already-held old keys/plaintexts cannot be erased remotely. Legacy unsigned group history is labelled and has no trusted badge; pinned private peers reject unsigned messages. Keys lost before migration cannot be recovered by this upgrade.

### Acceptance and recovery

- Verify two contacts with an offline-agreed, locally unlocked text appearance password. Both must complete the `paperphone.app` proof and challenge receipts. Reopen profiles and check refreshed badges; a wrong password or changed identity must not silently establish trust.
- Open the group member list and check refreshed trust. Send signed messages, remove an online member and an offline member, and confirm every remaining sender changes epoch/key before sending again. Old epochs must fail; continuing members should retain readable legitimate history. Rejoining must not expose messages from the absence interval.
- Reconnect after multiple pages of messages, including equal timestamps. Confirm cursor persistence, duplicate suppression and retry after historical key arrival. Message auto-delete and temporary-file retention remain independent limits; local caches hold at most 2,000 messages per conversation.
- Run the [security and eight-language checks](client/tests/README.md), then verify native Tor networking, profile/member-list refresh, attachments and persistence after restart. A healthy Tor route does not establish contact identity.

A downgrade is not just an image change: the schema, signed messages and new key epochs may be unreadable to old clients. Plan recovery with a matched database backup, server build and compatible clients; retain client key material and the onion-service volume. Avoid restoring a database snapshot over active writers.

Message signatures protect encrypted message envelopes. They do not encrypt uploaded attachment bytes or hide account, membership and routing metadata from the service. The legacy `kem_pub` field contains an Ed25519 signing key; this update does not deploy ML-KEM post-quantum registration or MLS membership consensus.
