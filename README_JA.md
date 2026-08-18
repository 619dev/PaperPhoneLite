# PaperPhoneLite 3.0.0

PaperPhoneLite は、個人・グループチャット、添付ファイル、音声メッセージ、信頼性の高い同期、オフラインキャッシュ、プッシュ通知、QR、2FA を備えた軽量なエンドツーエンド暗号化メッセンジャーです。

3.0.0 では Moments、Timeline、すべての音声・ビデオ通話、LiveKit、Cloudflare R2 を削除しました。ファイルはサーバーの永続ボリュームだけに保存されます。セルフホストでは Nginx を使わず、Tor v3 onion service から Rust サーバーへ直接転送し、MySQL と Redis は非公開です。Web クライアントには Tor を組み込まず、将来のネイティブクライアントに Tor と clearnet フォールバック禁止を実装します。[DEPLOY_EN.md](DEPLOY_EN.md) を参照してください。
