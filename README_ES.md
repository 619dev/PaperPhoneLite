# PaperPhoneLite 3.1.0

[简体中文](README.md) · [English](README_EN.md) · [日本語](README_JA.md) · [한국어](README_KO.md) · [Français](README_FR.md) · [Deutsch](README_DE.md) · [Русский](README_RU.md) · [Español](README_ES.md)

[Historial de cambios](changelog.md)

[![Rust](https://img.shields.io/badge/Rust-1.83+-orange)](#) [![Axum](https://img.shields.io/badge/Axum-0.8-black)](#) [![React](https://img.shields.io/badge/React-19-blue)](#) [![TypeScript](https://img.shields.io/badge/TypeScript-5.7-blue)](#) [![MySQL](https://img.shields.io/badge/MySQL-8.0-blue)](#) [![Redis](https://img.shields.io/badge/Redis-7.x-red)](#) [![Tor](https://img.shields.io/badge/Tor-v3-7D4698?logo=tor-project)](#) [![Version](https://img.shields.io/badge/Version-3.1.0-orange)](client/package.json) [![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](LICENSE)

PaperPhoneLite es un mensajero ligero con cifrado de extremo a extremo y sin funciones sociales públicas.

<details>
<summary>📸 Capturas de pantalla (haz clic para desplegar)</summary>
<img width="30%" height="30%" src="screenshot/ui1.png" alt="Captura de pantalla 1">
<img width="30%" height="30%" src="screenshot/ui2.png" alt="Captura de pantalla 2">
<img width="30%" height="30%" src="screenshot/ui3.png" alt="Captura de pantalla 3">
</details>

## Funciones

- X25519 · XSalsa20-Poly1305 · Ed25519.
- Chats privados y grupos cifrados con Sender Key de hasta 2.000 miembros.
- Texto, imágenes, vídeo, documentos, voz, emoji y stickers de Telegram.
- Sincronización WebSocket fiable, cola sin conexión y caché aislada por cuenta.
- Eliminación automática de mensajes tras 1, 3, 7 o 30 días.
- Solicitudes de amistad, alias, etiquetas, bloqueo e invitaciones QR a grupos.
- 2FA TOTP, notificaciones ntfy opcionales en Android y endpoints Bark configurados por el usuario en iOS. El servidor solo permite `api.day.app` por defecto; los hosts Bark propios se autorizan mediante `BARK_ALLOWED_HOSTS`.
- Archivos de hasta 500 MB en el volumen persistente del servidor; los avatares de usuarios/amigos y grupos son permanentes, los adjuntos de chat caducan según la configuración del servidor y las cargas antiguas se migran automáticamente.
- Ocho idiomas de interfaz.

## Verificación de identidad, firmas y recuperación sin conexión

Estos cambios del protocolo **aún no están publicados**; la versión sigue siendo 3.1.0. Consulte el [registro](changelog.md).

Acuerde la contraseña de apariencia del texto fuera de Internet, configúrela localmente y desbloquéela. **Esta contraseña permite comprobar si la otra persona es de confianza. Compártala solo fuera de Internet; nunca la envíe en línea de ninguna forma.** La verificación cifra exactamente `paperphone.app` con AES-GCM, vincula ambas identidades, el número de seguridad existente de 12 grupos y desafíos aleatorios, y firma el sobre cifrado con Ed25519. El perfil muestra un ✅ verde solo cuando ambas partes validan las pruebas y confirman la recepción de los desafíos. Compare los números por un canal fiable fuera de Internet.

Abrir un perfil privado actualiza las claves, comprueba la confianza local y recupera confirmaciones del historial. La contraseña desbloqueada inicia o reanuda automáticamente la verificación. Las aperturas repetidas conservan el desafío y limitan los reintentos. Un cambio de clave revoca la confianza y exige verificar manualmente tras comprobarla fuera de Internet. Recuperar el foco o la visibilidad también actualiza el estado.

Todos los mensajes nuevos de grupo requieren firma y cifrado Sender Key. Los grupos existentes se migran y el cifrado no se puede desactivar. Verifique a cada participante en privado antes de distribuir claves. La lista de miembros actualiza claves y confianza sin iniciar verificaciones privadas masivas. La firma protege grupo, remitente, generación de clave, ID, tipo, claves de identidad, nombre, nonce y texto cifrado. Una firma válida de una clave fiable produce un ✅ verde; el contenido permanece bloqueado mientras falte la clave de descifrado.

Entrar, salir o expulsar a un miembro incrementa la generación de claves. Los miembros conectados renuevan inmediatamente sus propias claves de envío tras el aviso; los demás, al reconectarse y antes de enviar. El servidor rechaza generaciones anteriores. Quien sale no puede enviar ni obtener nuevos mensajes o claves; volver a entrar no permite consultar el período de ausencia. Los destinatarios restantes conservan las claves históricas. No se pueden borrar a distancia claves o textos que un antiguo miembro ya posea.

La recuperación guarda `server_seq` por cuenta, recorre todas las páginas y reintenta con versiones históricas de las claves. El historial de grupo usa secuencias inmutables sin un límite total fijo. La caché local sigue limitada a 2.000 mensajes por conversación y se mantienen los plazos de borrado automático. Las claves ya perdidas no se pueden recrear.

Se utilizan X25519, XSalsa20-Poly1305 y Ed25519. El campo antiguo `kem_pub` contiene una clave de firma, no un registro poscuántico ML-KEM funcional. El servidor controla miembros y orden, sin consenso MLS. Los metadatos y los propios archivos adjuntos subidos no pasan a tener cifrado de extremo a extremo gracias a las firmas.

Actualice conjuntamente el servidor y los clientes de todas las plataformas: los antiguos no entienden los sobres nuevos. Los cambios del código no implican aplicaciones o imágenes Docker publicadas. Consulte [migración](DEPLOY_EN.md) y [pruebas](client/tests/README.md).

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
