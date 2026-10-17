# PaperPhoneLite 3.1.0

[简体中文](README.md) · [English](README_EN.md) · [日本語](README_JA.md) · [한국어](README_KO.md) · [Français](README_FR.md) · [Deutsch](README_DE.md) · [Русский](README_RU.md) · [Español](README_ES.md)

[更新履歴](changelog.md)

[![Rust](https://img.shields.io/badge/Rust-1.83+-orange)](#) [![Axum](https://img.shields.io/badge/Axum-0.8-black)](#) [![React](https://img.shields.io/badge/React-19-blue)](#) [![TypeScript](https://img.shields.io/badge/TypeScript-5.7-blue)](#) [![MySQL](https://img.shields.io/badge/MySQL-8.0-blue)](#) [![Redis](https://img.shields.io/badge/Redis-7.x-red)](#) [![Tor](https://img.shields.io/badge/Tor-v3-7D4698?logo=tor-project)](#) [![Version](https://img.shields.io/badge/Version-3.1.0-orange)](client/package.json) [![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](LICENSE)

PaperPhoneLite は、公開ソーシャル機能を持たない軽量なエンドツーエンド暗号化メッセンジャーです。

<details>
<summary>📸 スクリーンショット（クリックして展開）</summary>
<img width="30%" height="30%" src="screenshot/ui1.png" alt="スクリーンショット 1">
<img width="30%" height="30%" src="screenshot/ui2.png" alt="スクリーンショット 2">
<img width="30%" height="30%" src="screenshot/ui3.png" alt="スクリーンショット 3">
</details>

## 機能

- X25519 · XSalsa20-Poly1305 · Ed25519.
- 個人チャットと最大 2,000 人の Sender Key 暗号化グループ。
- テキスト、画像、動画、文書、音声、絵文字、Telegram ステッカー。
- 信頼性の高い WebSocket 同期、オフライン送信キュー、アカウント別キャッシュ。
- 1、3、7、30 日後のメッセージ自動削除。
- 友達申請、メモ、タグ、ブロック、QR グループ招待。
- TOTP 2FA、Android の任意の ntfy 通知、iOS のユーザー指定 Bark エンドポイント。サーバーは既定で `api.day.app` のみ許可し、セルフホスト Bark は `BARK_ALLOWED_HOSTS` で追加します。
- 最大 500 MB のファイルをサーバー永続ボリュームに保存。ユーザー／友だちとグループのアバターは永久保存され、チャット添付はサーバー設定の期限で削除され、旧形式のアップロードは自動移行されます。
- 8 言語の UI。

## 本人確認、署名とオフライン同期

以下は未リリースのソース変更です。バージョンは 3.1.0 のままです。[変更履歴](changelog.md)を参照してください。

新しい連絡先とは、テキスト外観パスワードをオフラインで取り決め、端末で設定してロックを解除します。**テキスト外観パスワードは相手が信頼できるか確認するための証明です。オフラインでのみ共有し、いかなる形でもオンラインに送信しないでください。** AES-GCM で固定文字列 `paperphone.app` を暗号化し、双方の識別鍵、従来の 12 組の安全番号、ランダムなチャレンジを結び付け、暗号化した封筒に Ed25519 署名を付けます。双方の証明とチャレンジの受領確認が成立すると、プロフィール名に緑の ✅ が付きます。安全番号は信頼できるオフライン経路で照合してください。

個人情報ページを開くと、公開鍵とローカルの信頼記録を再確認し、履歴から確認応答を復元します。パスワードが解除済みなら確認を自動開始・再開します。再表示では未完了のチャレンジを維持し、再試行を制限します。鍵の変更時は信頼を解除し、オフライン照合後の手動確認が必要です。フォーカスや表示状態の復帰時も更新します。

新しい群メッセージには署名と Sender Key 暗号化が必須です。既存の群は移行され、暗号化は無効にできません。鍵配布の前に各メンバーを個人チャットで確認します。メンバー一覧は鍵と信頼状態を更新しますが、一括で個人確認を開始しません。署名は群、送信者、鍵世代、メッセージ ID、種類、識別鍵、名前、nonce、暗号文を保護します。信頼済み鍵の署名が正しければ緑の ✅ を表示し、復号鍵が未着なら本文はロックされたまま再試行を待ちます。

参加・退会・除名で鍵世代が進み、オンラインの全送信者は通知後に自分の送信鍵を更新します。オフライン端末は再接続時と送信前に更新し、サーバーは古い世代の送信と鍵公開を拒否します。退会者は新しいメッセージや鍵を取得できず、再参加でも不在期間の履歴は取得できません。残る受信者の履歴鍵は保持しますが、退会者が既に保持する鍵や平文は消去できません。

`server_seq` をアカウント別に保存し、最後までページ取得して履歴鍵で復号を再試行します。群履歴は不変の番号でページ取得し、総件数の固定上限はありません。ローカルキャッシュは会話ごとに最大 2,000 件で、自動削除期限は有効です。紛失済みの鍵は復元できません。

現在は X25519、XSalsa20-Poly1305、Ed25519 を使用します。旧 API の `kem_pub` は署名公開鍵であり、ML-KEM の耐量子鍵登録ではありません。群の所属と順序はサーバー管理で、MLS 合意ではありません。メタデータやアップロードした添付ファイル自体は、この署名変更による端末間暗号化の対象ではありません。

サーバーと全プラットフォームのクライアントを同時に更新してください。旧クライアントは新しい署名形式を読めません。ソース変更はアプリや Docker イメージの公開を意味しません。[移行手順](DEPLOY_EN.md)・[テスト](client/tests/README.md)。

## Tor は必須です

Android、iOS、Windows、macOS の本番クライアントは Tor を内蔵し、ログイン前に Tor の bootstrap 完了を待ち、`.onion` 通信を分離 SOCKS5 回線に限定して clearnet へフォールバックしてはいけません。`client/` は共有 UI ソースコードのみです。ブラウザと PWA は本番クライアントとしてサポートされません。

Moments、Timeline、すべての通話、LiveKit、Cloudflare R2、通報、モデレーション画面は削除済みです。本番は Docker Compose と Tor v3 onion service のみを使用します。[DEPLOY_EN.md](DEPLOY_EN.md) を参照してください。

サーバー起動時に、旧インストールに残る廃止済み Web Push／OneSignal テーブルと古い識別子を自動削除します。

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
