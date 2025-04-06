# Open WebUI 👋

![GitHub stars](https://img.shields.io/github/stars/open-webui/open-webui?style=social)
![GitHub forks](https://img.shields.io/github/forks/open-webui/open-webui?style=social)
![GitHub watchers](https://img.shields.io/github/watchers/open-webui/open-webui?style=social)
![GitHub repo size](https://img.shields.io/github/repo-size/open-webui/open-webui)
![GitHub language count](https://img.shields.io/github/languages/count/open-webui/open-webui)
![GitHub top language](https://img.shields.io/github/languages/top/open-webui/open-webui)
![GitHub last commit](https://img.shields.io/github/last-commit/open-webui/open-webui?color=red)
![Hits](https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Follama-webui%2Follama-wbui&count_bg=%2379C83D&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=hits&edge_flat=false)
[![Discord](https://img.shields.io/badge/Discord-Open_WebUI-blue?logo=discord&logoColor=white)](https://discord.gg/5rJgQTnV4s)
[![](https://img.shields.io/static/v1?label=Sponsor&message=%E2%9D%A4&logo=GitHub&color=%23fe8e86)](https://github.com/sponsors/tjbck)

**Open WebUIは、[拡張可能](https://docs.openwebui.com/features/plugin/)で機能豊富、かつユーザーフレンドリーなセルフホスト型AIプラットフォームで、完全にオフラインで動作するように設計されています。** **Ollama**や**OpenAI互換API**などの様々なLLMランナーをサポートし、RAG用の**組み込み推論エンジン**を備えており、**強力なAIデプロイメントソリューション**となっています。

![Open WebUI Demo](./demo.gif)

> [!TIP]  
> **[エンタープライズプラン](https://docs.openwebui.com/enterprise)をお探しですか？** – **[今すぐ営業チームにお問い合わせください！](mailto:sales@openwebui.com)**
>
> **カスタムテーマとブランディング**、**サービスレベル契約（SLA）サポート**、**長期サポート（LTS）バージョン**など、**拡張機能**をご利用いただけます！

詳細については、[Open WebUIドキュメント](https://docs.openwebui.com/)をご確認ください。

## Open WebUIの主な機能 ⭐

- 🚀 **簡単セットアップ**: DockerやKubernetes（kubectl、kustomize、helm）を使用して、`:ollama`と`:cuda`タグ付きイメージの両方をサポートする手間のないインストールが可能です。

- 🤝 **Ollama/OpenAI API統合**: Ollamaモデルと並行して、OpenAI互換APIを簡単に統合し、多様な会話を実現します。OpenAI APIのURLをカスタマイズして、**LMStudio、GroqCloud、Mistral、OpenRouterなど**と連携できます。

- 🛡️ **詳細な権限とユーザーグループ**: 管理者が詳細なユーザーロールと権限を作成できるようにすることで、安全なユーザー環境を確保します。この詳細な設定はセキュリティを強化するだけでなく、カスタマイズされたユーザーエクスペリエンスを可能にし、ユーザー間の所有権と責任感を育みます。

- 📱 **レスポンシブデザイン**: デスクトップPC、ノートパソコン、モバイルデバイスで一貫したエクスペリエンスを提供します。

- 📱 **モバイル向けプログレッシブウェブアプリ（PWA）**: PWAによりモバイルデバイスでネイティブアプリのような体験を提供し、localhostでのオフラインアクセスとシームレスなユーザーインターフェースを実現します。

- ✒️🔢 **完全なMarkdownとLaTeXサポート**: 包括的なMarkdownとLaTeX機能により、LLMとの豊かなインタラクションを実現します。

- 🎤📹 **ハンズフリー音声/ビデオ通話**: 統合されたハンズフリー音声およびビデオ通話機能により、よりダイナミックでインタラクティブなチャット環境を実現します。

- 🛠️ **モデルビルダー**: WebUIを通じてOllamaモデルを簡単に作成できます。カスタムキャラクター/エージェントの作成、チャット要素のカスタマイズ、[Open WebUIコミュニティ](https://openwebui.com/)統合を通じたモデルの簡単なインポートが可能です。

- 🐍 **ネイティブPython関数呼び出しツール**: ツールワークスペースに組み込みのコードエディタサポートでLLMを強化します。純粋なPython関数を追加するだけで独自の関数（BYOF）を持ち込み、LLMとのシームレスな統合を実現します。

- 📚 **ローカルRAG統合**: 画期的な検索拡張生成（RAG）サポートにより、チャットインタラクションの未来を体験できます。この機能により、ドキュメントとのインタラクションがチャット体験にシームレスに統合されます。ドキュメントを直接チャットにロードしたり、ドキュメントライブラリにファイルを追加したりして、クエリの前に`#`コマンドを使用して簡単にアクセスできます。

- 🔍 **RAG用ウェブ検索**: `SearXNG`、`Google PSE`、`Brave Search`、`serpstack`、`serper`、`Serply`、`DuckDuckGo`、`TavilySearch`、`SearchApi`、`Bing`などのプロバイダーを使用してウェブ検索を実行し、結果を直接チャット体験に組み込むことができます。

- 🌐 **ウェブブラウジング機能**: URLの前に`#`コマンドを使用して、ウェブサイトをチャット体験にシームレスに統合できます。この機能により、ウェブコンテンツを会話に直接取り込み、インタラクションの豊かさと深さを高めることができます。

- 🎨 **画像生成統合**: AUTOMATIC1111 APIやComfyUI（ローカル）、OpenAIのDALL-E（外部）などのオプションを使用して画像生成機能をシームレスに組み込み、ダイナミックなビジュアルコンテンツでチャット体験を豊かにします。

- ⚙️ **複数モデル会話**: 様々なモデルと同時に簡単に対話し、それぞれの強みを活かして最適な応答を得ることができます。並行して多様なモデルセットを活用することで、体験を向上させます。

- 🔐 **ロールベースのアクセス制御（RBAC）**: 制限された権限で安全なアクセスを確保します。権限のある個人のみがOllamaにアクセスでき、モデルの作成/取得権限は管理者専用です。

- 🌐🌍 **多言語サポート**: 国際化（i18n）サポートにより、お好みの言語でOpen WebUIを体験できます。サポート言語の拡大にご協力ください！積極的に貢献者を募集しています！

- 🧩 **パイプライン、Open WebUIプラグインサポート**: [パイプラインプラグインフレームワーク](https://github.com/open-webui/pipelines)を使用して、カスタムロジックとPythonライブラリをOpen WebUIにシームレスに統合できます。パイプラインインスタンスを起動し、OpenAI URLをパイプラインURLに設定して、無限の可能性を探索しましょう。[例](https://github.com/open-webui/pipelines/tree/main/examples)には、**関数呼び出し**、アクセスを制御するユーザー**レート制限**、Langfuseなどのツールを使用した**使用状況モニタリング**、多言語サポート用の**LibreTranslateによるライブ翻訳**、**有害メッセージフィルタリング**などが含まれます。

- 🌟 **継続的な更新**: 定期的な更新、修正、新機能の追加によりOpen WebUIの改善に取り組んでいます。

Open WebUIの機能についてもっと知りたい場合は、[Open WebUIドキュメント](https://docs.openwebui.com/features)で包括的な概要をご確認ください！

## 🔗 Open WebUIコミュニティもチェックしてください！

姉妹プロジェクトの[Open WebUIコミュニティ](https://openwebui.com/)も忘れずにチェックしてください。カスタマイズされたModelfilesの発見、ダウンロード、探索ができます。Open WebUIコミュニティは、Open WebUIとのチャットインタラクションを強化するための幅広い可能性を提供しています！🚀

## インストール方法 🚀

### Python pipによるインストール 🐍

Open WebUIはPythonパッケージインストーラーのpipを使用してインストールできます。互換性の問題を避けるため、**Python 3.11**を使用していることを確認してください。

1. **Open WebUIのインストール**:
   ターミナルを開き、次のコマンドを実行してOpen WebUIをインストールします：

   ```bash
   pip install open-webui
   ```

2. **Open WebUIの実行**:
   インストール後、次のコマンドを実行してOpen WebUIを起動できます：

   ```bash
   open-webui serve
   ```

これによりOpen WebUIサーバーが起動し、[http://localhost:8080](http://localhost:8080)でアクセスできます。

### Dockerでのクイックスタート 🐳

> [!NOTE]  
> 特定のDocker環境では、追加の設定が必要な場合があります。接続の問題が発生した場合は、[Open WebUIドキュメント](https://docs.openwebui.com/)の詳細ガイドがお役に立ちます。

> [!WARNING]
> DockerでOpen WebUIをインストールする際は、Dockerコマンドに`-v open-webui:/app/backend/data`を含めるようにしてください。これはデータベースが適切にマウントされ、データの損失を防ぐために重要なステップです。

> [!TIP]  
> Open WebUIをOllamaを含めて使用したい場合や、CUDA高速化を利用したい場合は、`:cuda`または`:ollama`でタグ付けされた公式イメージの使用をお勧めします。CUDAを有効にするには、Linux/WSLシステムに[Nvidia CUDA container toolkit](https://docs.nvidia.com/dgx/nvidia-container-runtime-upgrade/)をインストールする必要があります。

### デフォルト設定でのインストール

- **Ollamaがあなたのコンピュータにある場合**、このコマンドを使用します：

  ```bash
  docker run -d -p 3000:8080 --add-host=host.docker.internal:host-gateway -v open-webui:/app/backend/data --name open-webui --restart always ghcr.io/open-webui/open-webui:main
  ```

- **Ollamaが別のサーバーにある場合**、このコマンドを使用します：

  別のサーバーのOllamaに接続するには、`OLLAMA_BASE_URL`をサーバーのURLに変更します：

  ```bash
  docker run -d -p 3000:8080 -e OLLAMA_BASE_URL=https://example.com -v open-webui:/app/backend/data --name open-webui --restart always ghcr.io/open-webui/open-webui:main
  ```

- **Nvidia GPUサポートでOpen WebUIを実行するには**、このコマンドを使用します：

  ```bash
  docker run -d -p 3000:8080 --gpus all --add-host=host.docker.internal:host-gateway -v open-webui:/app/backend/data --name open-webui --restart always ghcr.io/open-webui/open-webui:cuda
  ```

### OpenAI APIのみの使用のためのインストール

- **OpenAI APIのみを使用している場合**、このコマンドを使用します：

  ```bash
  docker run -d -p 3000:8080 -e OPENAI_API_KEY=your_secret_key -v open-webui:/app/backend/data --name open-webui --restart always ghcr.io/open-webui/open-webui:main
  ```

### バンドルされたOllamaサポート付きOpen WebUIのインストール

このインストール方法では、Open WebUIとOllamaをバンドルした単一のコンテナイメージを使用し、単一のコマンドによる合理化されたセットアップを可能にします。ハードウェア設定に応じて適切なコマンドを選択してください：

- **GPUサポート付き**:
  次のコマンドを実行してGPUリソースを活用します：

  ```bash
  docker run -d -p 3000:8080 --gpus=all -v ollama:/root/.ollama -v open-webui:/app/backend/data --name open-webui --restart always ghcr.io/open-webui/open-webui:ollama
  ```

- **CPUのみ**:
  GPUを使用していない場合は、代わりにこのコマンドを使用します：

  ```bash
  docker run -d -p 3000:8080 -v ollama:/root/.ollama -v open-webui:/app/backend/data --name open-webui --restart always ghcr.io/open-webui/open-webui:ollama
  ```

両方のコマンドにより、Open WebUIとOllamaの両方が組み込まれた手間のないインストールが可能になり、すべてを迅速に起動して実行できます。

インストール後、[http://localhost:3000](http://localhost:3000)でOpen WebUIにアクセスできます。お楽しみください！😄

### その他のインストール方法

Docker以外のネイティブインストール方法、Docker Compose、Kustomize、Helmなど、様々なインストール方法を提供しています。詳細なガイダンスについては、[Open WebUIドキュメント](https://docs.openwebui.com/getting-started/)をご覧いただくか、[Discordコミュニティ](https://discord.gg/5rJgQTnV4s)にご参加ください。

### トラブルシューティング

接続の問題が発生していますか？[Open WebUIドキュメント](https://docs.openwebui.com/troubleshooting/)がお役に立ちます。さらなるサポートや活気あるコミュニティへの参加については、[Open WebUI Discord](https://discord.gg/5rJgQTnV4s)をご覧ください。

#### Open WebUI: サーバー接続エラー

接続の問題が発生している場合、多くの場合、WebUI dockerコンテナがコンテナ内の127.0.0.1:11434（host.docker.internal:11434）でOllamaサーバーに到達できないことが原因です。dockerコマンドに`--network=host`フラグを使用してこれを解決します。ポートが3000から8080に変更され、リンクは`http://localhost:8080`になることに注意してください。

**Dockerコマンド例**:

```bash
docker run -d --network=host -v open-webui:/app/backend/data -e OLLAMA_BASE_URL=http://127.0.0.1:11434 --name open-webui --restart always ghcr.io/open-webui/open-webui:main
```

### Dockerインストールを最新の状態に保つ

ローカルのDockerインストールを最新バージョンに更新したい場合は、[Watchtower](https://containrrr.dev/watchtower/)を使用して行うことができます：

```bash
docker run --rm --volume /var/run/docker.sock:/var/run/docker.sock containrrr/watchtower --run-once open-webui
```

コマンドの最後の部分で、コンテナ名が異なる場合は`open-webui`をあなたのコンテナ名に置き換えてください。

[Open WebUIドキュメント](https://docs.openwebui.com/getting-started/updating)で利用可能な更新ガイドをご確認ください。

### 開発ブランチの使用 🌙

> [!WARNING]
> `:dev`ブランチには最新の不安定な機能と変更が含まれています。バグや不完全な機能がある可能性があるため、自己責任で使用してください。

最新の最先端機能を試してみたい場合で、時折の不安定さを許容できる場合は、次のように`:dev`タグを使用できます：

```bash
docker run -d -p 3000:8080 -v open-webui:/app/backend/data --name open-webui --add-host=host.docker.internal:host-gateway --restart always ghcr.io/open-webui/open-webui:dev
```

### オフラインモード

オフライン環境でOpen WebUIを実行している場合、インターネットからモデルをダウンロードしようとするのを防ぐために、`HF_HUB_OFFLINE`環境変数を`1`に設定できます。

```bash
export HF_HUB_OFFLINE=1
```

## 次は何ですか？🌟

[Open WebUIドキュメント](https://docs.openwebui.com/roadmap/)のロードマップで今後の機能をご確認ください。

## ライセンス 📜

このプロジェクトは[BSD-3-Clause License](LICENSE)の下でライセンスされています - 詳細は[LICENSE](LICENSE)ファイルをご覧ください。📄

## サポート 💬

質問、提案、またはサポートが必要な場合は、issueを開くか、
[Open WebUI Discordコミュニティ](https://discord.gg/5rJgQTnV4s)に参加して私たちとつながりましょう！🤝

## スター履歴

<a href="https://star-history.com/#open-webui/open-webui&Date">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=open-webui/open-webui&type=Date&theme=dark" />
    <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=open-webui/open-webui&type=Date" />
    <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=open-webui/open-webui&type=Date" />
  </picture>
</a>

---

作成者: [Timothy Jaeryang Baek](https://github.com/tjbck) - 一緒にOpen WebUIをさらに素晴らしいものにしましょう！💪