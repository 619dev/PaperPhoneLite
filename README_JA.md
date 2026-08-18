# PaperPhoneLite 3.0.0

PaperPhoneLite は、公開ソーシャル機能を持たない軽量なエンドツーエンド暗号化メッセンジャーです。

## 機能

- X25519 + ML-KEM-768 ハイブリッド鍵共有、XSalsa20-Poly1305、安全番号検証。
- 個人チャットと最大 2,000 人の Sender Key 暗号化グループ。
- テキスト、画像、動画、文書、音声、絵文字、Telegram ステッカー。
- 信頼性の高い WebSocket 同期、オフライン送信キュー、アカウント別キャッシュ。
- 1、3、7、30 日後のメッセージ自動削除。
- 友達申請、メモ、タグ、ブロック、QR グループ招待。
- TOTP 2FA と Web Push、FCM、OneSignal、ntfy、APNS 通知。
- 最大 500 MB のファイルをサーバー永続ボリュームのみに保存。
- 8 言語の UI。

## Tor は必須です

Android、iOS、Windows、macOS の本番クライアントは Tor を内蔵し、ログイン前に Tor の bootstrap 完了を待ち、`.onion` 通信を分離 SOCKS5 回線に限定して clearnet へフォールバックしてはいけません。`client/` は共有 UI ソースコードのみです。ブラウザと PWA は本番クライアントとしてサポートされません。

Moments、Timeline、すべての通話、LiveKit、Cloudflare R2、通報、モデレーション画面は削除済みです。本番は Docker Compose と Tor v3 onion service を使用します。Zeabur テンプレートは開発・移行専用です。[DEPLOY_EN.md](DEPLOY_EN.md) を参照してください。

このプロジェクトが役に立ったら、コーラを一缶おごってください。

<img width="30%" height="30%" src="请我喝可乐.jpg" alt="寄付 QR コード">

Telegram グループ: https://t.me/+vHJtvWJY_gEyMTUx

ライセンス: AGPL-3.0。
