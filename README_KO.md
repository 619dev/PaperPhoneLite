# PaperPhoneLite 3.0.12

[简体中文](README.md) · [English](README_EN.md) · [日本語](README_JA.md) · [한국어](README_KO.md) · [Français](README_FR.md) · [Deutsch](README_DE.md) · [Русский](README_RU.md) · [Español](README_ES.md)

[변경 기록](changelog.md)

PaperPhoneLite는 공개 소셜 기능이 없는 경량 종단간 암호화 메신저입니다.

<details>
<summary>앱 스크린샷</summary>

![PaperPhoneLite 앱 스크린샷 1](screenshot/ui1.png)
![PaperPhoneLite 앱 스크린샷 2](screenshot/ui2.png)
![PaperPhoneLite 앱 스크린샷 3](screenshot/ui3.png)

</details>

## 기능

- X25519 + ML-KEM-768 하이브리드 키 합의, XSalsa20-Poly1305 및 안전 번호 검증.
- 개인 채팅과 최대 2,000명의 Sender Key 암호화 그룹.
- 텍스트, 이미지, 동영상, 문서, 음성, 이모지 및 Telegram 스티커.
- 안정적인 WebSocket 동기화, 오프라인 전송 큐 및 계정별 캐시.
- 1일, 3일, 7일 또는 30일 후 메시지 자동 삭제.
- 친구 요청, 메모, 태그, 차단 및 QR 그룹 초대.
- TOTP 2FA와 Android에서 선택적으로 사용할 수 있는 ntfy 백그라운드 알림. iOS 클라이언트는 APNs를 사용하지 않으며 현재 시스템 백그라운드 원격 알림을 제공하지 않습니다.
- 최대 500MB 파일을 서버 영구 볼륨에만 저장하며, 첨부 파일은 외부 브라우저에서 객체 스토리지 또는 `.onion` URL을 열지 않고 Rust 서버를 통해 다운로드되어 휴대전화 시스템 패널에서 저장할 수 있습니다.
- 8개 UI 언어.

## Tor 필수

Android, iOS, Windows 및 macOS 운영 클라이언트는 Tor를 내장하고 로그인 전에 Tor bootstrap 완료를 기다려야 하며, `.onion` 연결을 격리된 SOCKS5 회로로만 전송하고 clearnet으로 폴백해서는 안 됩니다. `client/`는 공유 UI 소스 코드만 포함합니다. 브라우저와 PWA는 운영 클라이언트로 지원되지 않습니다.

Moments, Timeline, 모든 통화, LiveKit, Cloudflare R2, 신고 및 검토 대시보드는 제거되었습니다. 운영 환경은 Docker Compose와 Tor v3 onion service만 사용합니다. [DEPLOY_EN.md](DEPLOY_EN.md)를 참조하세요.

Docker Compose가 시작되면 Tor 컨테이너가 전체 `http://...onion` 주소를 호스트의 `logs/onion-address.log`에 자동으로 기록하며, `docker compose logs tor`에서도 확인할 수 있습니다.

iOS 앱은 Apple App Store 배포를 목표로 합니다. Android APK는 Android 클라이언트 저장소의 [GitHub Releases](https://github.com/619dev/ppl-android/releases)에서만 배포하고 Google Play에는 게시하지 않으며 Google 푸시 또는 서비스 프레임워크를 포함하지 않습니다.

## 클라이언트 저장소

| 플랫폼 | 소스 코드 및 릴리스 |
|--------|---------------------|
| Android | [619dev/ppl-android](https://github.com/619dev/ppl-android) |
| iOS | [619dev/ppl-ios](https://github.com/619dev/ppl-ios) |
| macOS | [619dev/ppl-mac](https://github.com/619dev/ppl-mac) |
| Windows | [619dev/ppl-win](https://github.com/619dev/ppl-win) |

프로젝트가 유용하다면 콜라 한 캔을 사 주세요.

<img width="30%" height="30%" src="请我喝可乐.jpg" alt="후원 QR 코드">

Telegram 그룹: https://t.me/+vHJtvWJY_gEyMTUx

라이선스: AGPL-3.0.
