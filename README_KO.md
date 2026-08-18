# PaperPhoneLite 3.0.0

PaperPhoneLite는 공개 소셜 기능이 없는 경량 종단간 암호화 메신저입니다.

## 기능

- X25519 + ML-KEM-768 하이브리드 키 합의, XSalsa20-Poly1305 및 안전 번호 검증.
- 개인 채팅과 최대 2,000명의 Sender Key 암호화 그룹.
- 텍스트, 이미지, 동영상, 문서, 음성, 이모지 및 Telegram 스티커.
- 안정적인 WebSocket 동기화, 오프라인 전송 큐 및 계정별 캐시.
- 1일, 3일, 7일 또는 30일 후 메시지 자동 삭제.
- 친구 요청, 메모, 태그, 차단 및 QR 그룹 초대.
- TOTP 2FA와 Android의 ntfy 및 iOS의 APNs만 사용하는 알림.
- 최대 500MB 파일을 서버 영구 볼륨에만 저장.
- 8개 UI 언어.

## Tor 필수

Android, iOS, Windows 및 macOS 운영 클라이언트는 Tor를 내장하고 로그인 전에 Tor bootstrap 완료를 기다려야 하며, `.onion` 연결을 격리된 SOCKS5 회로로만 전송하고 clearnet으로 폴백해서는 안 됩니다. `client/`는 공유 UI 소스 코드만 포함합니다. 브라우저와 PWA는 운영 클라이언트로 지원되지 않습니다.

Moments, Timeline, 모든 통화, LiveKit, Cloudflare R2, 신고 및 검토 대시보드는 제거되었습니다. 운영 환경은 Docker Compose와 Tor v3 onion service를 사용합니다. Zeabur 템플릿은 개발 또는 마이그레이션 전용입니다. [DEPLOY_EN.md](DEPLOY_EN.md)를 참조하세요.

iOS 앱은 Apple App Store 배포를 목표로 합니다. Android APK는 [GitHub Releases](https://github.com/619dev/PaperPhoneLite/releases)에서만 배포하고 Google Play에는 게시하지 않으며 Google 푸시 또는 서비스 프레임워크를 포함하지 않습니다.

프로젝트가 유용하다면 콜라 한 캔을 사 주세요.

<img width="30%" height="30%" src="请我喝可乐.jpg" alt="후원 QR 코드">

Telegram 그룹: https://t.me/+vHJtvWJY_gEyMTUx

라이선스: AGPL-3.0.
