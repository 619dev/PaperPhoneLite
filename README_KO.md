# PaperPhoneLite 3.0.0

PaperPhoneLite는 개인/그룹 채팅, 첨부 파일, 음성 메시지, 안정적인 동기화, 오프라인 캐시, 푸시, QR 및 2FA를 제공하는 경량 종단간 암호화 메신저입니다.

3.0.0에서는 Moments, Timeline, 모든 음성/영상 통화, LiveKit 및 Cloudflare R2를 제거했습니다. 파일은 서버 영구 볼륨에만 저장됩니다. 자체 호스팅은 Nginx 없이 Tor v3 onion service를 Rust 서버에 직접 연결하며 MySQL과 Redis는 비공개로 유지합니다. Web 클라이언트에는 Tor를 내장하지 않으며 향후 네이티브 클라이언트는 Tor를 내장하고 clearnet 폴백을 금지해야 합니다. [DEPLOY_EN.md](DEPLOY_EN.md)를 참조하세요.
