# PaperPhoneLite 3.0.0

[简体中文](README.md) · [English](README_EN.md) · [日本語](README_JA.md) · [한국어](README_KO.md) · [Français](README_FR.md) · [Deutsch](README_DE.md) · [Русский](README_RU.md) · [Español](README_ES.md)

[Historial de cambios](changelog.md)

PaperPhoneLite es un mensajero ligero con cifrado de extremo a extremo y sin funciones sociales públicas.

<details>
<summary>Capturas de la aplicación</summary>

![Captura 1 de la aplicación PaperPhoneLite](screenshot/ui1.png)
![Captura 2 de la aplicación PaperPhoneLite](screenshot/ui2.png)
![Captura 3 de la aplicación PaperPhoneLite](screenshot/ui3.png)

</details>

## Funciones

- Acuerdo de claves híbrido X25519 + ML-KEM-768, XSalsa20-Poly1305 y números de seguridad.
- Chats privados y grupos cifrados con Sender Key de hasta 2.000 miembros.
- Texto, imágenes, vídeo, documentos, voz, emoji y stickers de Telegram.
- Sincronización WebSocket fiable, cola sin conexión y caché aislada por cuenta.
- Eliminación automática de mensajes tras 1, 3, 7 o 30 días.
- Solicitudes de amistad, alias, etiquetas, bloqueo e invitaciones QR a grupos.
- 2FA TOTP y notificaciones exclusivamente mediante ntfy en Android y APNs en iOS.
- Archivos de hasta 500 MB almacenados únicamente en el volumen persistente del servidor.
- Ocho idiomas de interfaz.

## Tor es obligatorio

Los clientes de producción para Android, iOS, Windows y macOS deben integrar Tor, esperar a que finalice el arranque de Tor antes de iniciar sesión y usar circuitos SOCKS5 aislados para `.onion`, sin salida alternativa a clearnet. `client/` contiene solo el código fuente compartido de la interfaz. Los navegadores y las PWA no son clientes de producción compatibles.

Se eliminaron Moments, Timeline, todas las llamadas, LiveKit, Cloudflare R2, los reportes y el panel de moderación. Producción usa exclusivamente Docker Compose y un servicio onion Tor v3. Consulte [DEPLOY_EN.md](DEPLOY_EN.md).

Después de iniciar Docker Compose, el contenedor Tor guarda automáticamente la dirección completa `http://...onion` en el archivo del host `logs/onion-address.log`; también aparece en `docker compose logs tor`.

La aplicación iOS se distribuirá mediante Apple App Store. Los APK de Android se publican únicamente en [GitHub Releases](https://github.com/619dev/PaperPhoneLite/releases), no en Google Play, y no incluyen marcos de servicios o push de Google.

Si este proyecto te resulta útil, invítame a una lata de cola.

<img width="30%" height="30%" src="请我喝可乐.jpg" alt="Código QR de donación">

Grupo de Telegram: https://t.me/+vHJtvWJY_gEyMTUx

Licencia: AGPL-3.0.
