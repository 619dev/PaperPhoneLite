# PaperPhoneLite 3.0.15

[简体中文](README.md) · [English](README_EN.md) · [日本語](README_JA.md) · [한국어](README_KO.md) · [Français](README_FR.md) · [Deutsch](README_DE.md) · [Русский](README_RU.md) · [Español](README_ES.md)

[更新履歴](changelog.md)

PaperPhoneLite は、公開ソーシャル機能を持たない軽量なエンドツーエンド暗号化メッセンジャーです。

<details>
<summary>アプリのスクリーンショット</summary>

![PaperPhoneLite アプリのスクリーンショット 1](screenshot/ui1.png)
![PaperPhoneLite アプリのスクリーンショット 2](screenshot/ui2.png)
![PaperPhoneLite アプリのスクリーンショット 3](screenshot/ui3.png)

</details>

## 機能

- X25519 + ML-KEM-768 ハイブリッド鍵共有、XSalsa20-Poly1305、安全番号検証。
- 個人チャットと最大 2,000 人の Sender Key 暗号化グループ。
- テキスト、画像、動画、文書、音声、絵文字、Telegram ステッカー。
- 信頼性の高い WebSocket 同期、オフライン送信キュー、アカウント別キャッシュ。
- 1、3、7、30 日後のメッセージ自動削除。
- 友達申請、メモ、タグ、ブロック、QR グループ招待。
- TOTP 2FA と、Android で任意に利用できる ntfy バックグラウンド通知。iOS クライアントは APNs を使用せず、現在はシステムのバックグラウンドリモート通知を提供しません。
- 最大 500 MB のファイルをサーバー永続ボリュームのみに保存。添付ファイルは Rust サーバー経由でダウンロードされ、外部ブラウザでオブジェクトストレージや `.onion` URL を開かず、スマートフォンのシステム画面から保存できます。
- 8 言語の UI。

## Tor は必須です

Android、iOS、Windows、macOS の本番クライアントは Tor を内蔵し、ログイン前に Tor の bootstrap 完了を待ち、`.onion` 通信を分離 SOCKS5 回線に限定して clearnet へフォールバックしてはいけません。`client/` は共有 UI ソースコードのみです。ブラウザと PWA は本番クライアントとしてサポートされません。

Moments、Timeline、すべての通話、LiveKit、Cloudflare R2、通報、モデレーション画面は削除済みです。本番は Docker Compose と Tor v3 onion service のみを使用します。[DEPLOY_EN.md](DEPLOY_EN.md) を参照してください。

Docker Compose の起動後、Tor コンテナは完全な `http://...onion` アドレスをホストの `logs/onion-address.log` に自動保存します。`docker compose logs tor` でも確認できます。

iOS アプリは Apple App Store で配布予定です。Android APK は Android クライアントリポジトリの [GitHub Releases](https://github.com/619dev/ppl-android/releases) のみで公開し、Google Play には公開せず、Google のプッシュ／サービスフレームワークも組み込みません。

## クライアントリポジトリ

| プラットフォーム | ソースコードとリリース |
|------------------|--------------------------|
| Android | [619dev/ppl-android](https://github.com/619dev/ppl-android) |
| iOS | [619dev/ppl-ios](https://github.com/619dev/ppl-ios) |
| macOS | [619dev/ppl-mac](https://github.com/619dev/ppl-mac) |
| Windows | [619dev/ppl-win](https://github.com/619dev/ppl-win) |

このプロジェクトが役に立ったら、コーラを一缶おごってください。

<img width="30%" height="30%" src="请我喝可乐.jpg" alt="寄付 QR コード">

Telegram グループ: https://t.me/+vHJtvWJY_gEyMTUx

ライセンス: AGPL-3.0。
