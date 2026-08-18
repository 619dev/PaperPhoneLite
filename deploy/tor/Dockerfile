FROM alpine:3.21
RUN apk add --no-cache tor su-exec
COPY torrc /etc/tor/torrc
COPY entrypoint.sh /usr/local/bin/paperphone-tor-entrypoint
RUN mkdir -p /var/lib/tor/paperphone-lite && chown -R tor:tor /var/lib/tor
RUN chmod 0755 /usr/local/bin/paperphone-tor-entrypoint
ENTRYPOINT ["/usr/local/bin/paperphone-tor-entrypoint"]
