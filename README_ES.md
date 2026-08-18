# PaperPhoneLite 3.0.0

PaperPhoneLite es un mensajero ligero con cifrado de extremo a extremo, chats privados y grupales, archivos, mensajes de voz, sincronización fiable, caché sin conexión, notificaciones, QR y 2FA.

La versión 3.0.0 elimina Moments, Timeline, todas las llamadas de voz/vídeo, LiveKit y Cloudflare R2. Los archivos se guardan únicamente en un volumen persistente del servidor. El autohospedaje usa un servicio onion Tor v3, sin Nginx, conectado directamente al servidor Rust; MySQL y Redis permanecen privados. El cliente Web no integra Tor; los futuros clientes nativos deberán integrarlo y prohibir la salida a clearnet. Consulte [DEPLOY_EN.md](DEPLOY_EN.md).
