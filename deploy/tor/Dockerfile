FROM alpine:3.21
RUN apk add --no-cache tor
COPY torrc /etc/tor/torrc
RUN mkdir -p /var/lib/tor/paperphone-lite && chown -R tor:tor /var/lib/tor
USER tor
CMD ["tor", "-f", "/etc/tor/torrc"]
