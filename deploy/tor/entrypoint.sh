#!/bin/sh
set -eu

hidden_service_dir=/var/lib/tor/paperphone-lite
hostname_file="${hidden_service_dir}/hostname"
log_dir=/var/log/paperphone-lite
address_log="${log_dir}/onion-address.log"

mkdir -p "$hidden_service_dir" "$log_dir"
chown -R tor:tor /var/lib/tor

su-exec tor tor -f /etc/tor/torrc &
tor_pid=$!

terminate() {
  kill -TERM "$tor_pid" 2>/dev/null || true
  wait "$tor_pid" 2>/dev/null || true
}
trap terminate INT TERM

(
  while [ ! -s "$hostname_file" ]; do
    kill -0 "$tor_pid" 2>/dev/null || exit 1
    sleep 1
  done

  onion_address="$(tr -d '\r\n' < "$hostname_file")"
  printf 'http://%s\n' "$onion_address" > "$address_log"
  chmod 0644 "$address_log"
  printf 'PaperPhoneLite onion address: http://%s\n' "$onion_address"
) &
watcher_pid=$!

status=0
wait "$tor_pid" || status=$?
kill "$watcher_pid" 2>/dev/null || true
wait "$watcher_pid" 2>/dev/null || true
exit "$status"
