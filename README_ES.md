# PaperPhoneLite 3.0.16

[简体中文](README.md) · [English](README_EN.md) · [日本語](README_JA.md) · [한국어](README_KO.md) · [Français](README_FR.md) · [Deutsch](README_DE.md) · [Русский](README_RU.md) · [Español](README_ES.md)

[Historial de cambios](changelog.md)

[![Rust](https://img.shields.io/badge/Rust-1.83+-orange)](#) [![Axum](https://img.shields.io/badge/Axum-0.8-black)](#) [![React](https://img.shields.io/badge/React-19-blue)](#) [![TypeScript](https://img.shields.io/badge/TypeScript-5.7-blue)](#) [![MySQL](https://img.shields.io/badge/MySQL-8.0-blue)](#) [![Redis](https://img.shields.io/badge/Redis-7.x-red)](#) [![Tor](https://img.shields.io/badge/Tor-v3-7D4698?logo=tor-project)](#) [![Version](https://img.shields.io/badge/Version-3.0.16-orange)](client/package.json) [![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](LICENSE)

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
- 2FA TOTP, notificaciones ntfy opcionales en Android y endpoints Bark configurados por el usuario en iOS. El servidor solo permite `api.day.app` por defecto; los hosts Bark propios se autorizan mediante `BARK_ALLOWED_HOSTS`.
- Archivos de hasta 500 MB almacenados únicamente en el volumen persistente del servidor; los adjuntos se descargan a través del servidor Rust sin abrir direcciones de almacenamiento de objetos ni `.onion` en un navegador externo y pueden guardarse mediante el panel del sistema del móvil.
- Ocho idiomas de interfaz.

## Tor es obligatorio

Los clientes de producción para Android, iOS, Windows y macOS deben integrar Tor, esperar a que finalice el arranque de Tor antes de iniciar sesión y usar circuitos SOCKS5 aislados para `.onion`, sin salida alternativa a clearnet. `client/` contiene solo el código fuente compartido de la interfaz. Los navegadores y las PWA no son clientes de producción compatibles.

Se eliminaron Moments, Timeline, todas las llamadas, LiveKit, Cloudflare R2, los reportes y el panel de moderación. Producción usa exclusivamente Docker Compose y un servicio onion Tor v3. Consulte [DEPLOY_EN.md](DEPLOY_EN.md).

Al arrancar, el servidor elimina automáticamente las tablas retiradas de Web Push y OneSignal, junto con los identificadores obsoletos de instalaciones anteriores.

Después de iniciar Docker Compose, el contenedor Tor guarda automáticamente la dirección completa `http://...onion` en el archivo del host `logs/onion-address.log`; también aparece en `docker compose logs tor`.

La aplicación iOS se distribuirá mediante Apple App Store. Los APK de Android se publican únicamente en las [GitHub Releases](https://github.com/619dev/ppl-android/releases) del repositorio del cliente Android, no en Google Play, y no incluyen marcos de servicios o push de Google.

## Repositorios de los clientes

| Plataforma | Código fuente y versiones |
|------------|---------------------------|
| Android | [619dev/ppl-android](https://github.com/619dev/ppl-android) |
| iOS | [619dev/ppl-ios](https://github.com/619dev/ppl-ios) |
| macOS | [619dev/ppl-mac](https://github.com/619dev/ppl-mac) |
| Windows | [619dev/ppl-win](https://github.com/619dev/ppl-win) |

Si este proyecto te resulta útil, invítame a una lata de cola.

<img width="30%" height="30%" src="请我喝可乐.jpg" alt="Código QR de donación">

Grupo de Telegram: https://t.me/+vHJtvWJY_gEyMTUx

Licencia: AGPL-3.0.
