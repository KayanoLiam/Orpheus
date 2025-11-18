# Orpheus

[English](./README.en.md) | [简体中文](./README.zh.md)

## プロジェクト概要

OrpheusはRustエコシステムに基づいて構築されたSupabaseのような全栈アプリケーションシステムで、Webアプリケーションにすぐに使える認証、セッション管理、データアクセス機能を提供することを目的としています。プロジェクトはActix-WebをコアWebフレームワークとして使用し、PostgreSQLをメインデータストア、Redisをセッションストアとキャッシュ加速に使用しています。プロジェクトは厳格なコードセキュリティポリシーを採用し、完全なテストスイートを含んでいます。フロントエンドはNext.js 16 + React 19 + TypeScriptで構築されています。

### 技術スタック

#### バックエンド
- **フレームワーク**: Actix-Web 4
- **認証**: Actix-Web HTTPAuth (Bearer Token)
- **データベース**: PostgreSQL (SQLx 0.8.6経由)
- **キャッシュ/セッション**: Redis 0.23.1
- **パスワードハッシュ**: Argon2 0.5
- **非同期ランタイム**: Tokio (full features)
- **シリアライゼーション**: Serde + JSON
- **乱数生成**: rand_core 0.10.0-rc-2
- **ロギング**: tracing + tracing-subscriber
- **HTTPクライアント**: reqwest 0.12.24
- **CORS**: actix-cors 0.6

#### フロントエンド
- **フレームワーク**: Next.js 16.0.3
- **UIライブラリ**: React 19.2.0
- **言語**: TypeScript 5
- **スタイル**: Tailwind CSS 4
- **コンポーネントライブラリ**: Radix UI
- **アイコン**: Lucide React
- **HTTPクライアント**: Axios 1.13.2
- **ビルドツール**: ESLint 9
- **アニメーション**: tw-animate-css 1.4.0

### アーキテクチャの特徴
- 厳格なコードセキュリティポリシー（unwrap、expect、panicなどの無効化）
- セッションベースの認証システム
- RESTful API設計
- 非同期データベース操作
- 安全なパスワードハッシュ保存
- 完全なテストカバレッジ（単体テストと統合テスト）
- モジュラーコード構造
- フロントエンドとバックエンド分離アーキテクチャ
- コンテナ化デプロイサポート

## プロジェクト構造

```
Orpheus/
├───src/                    # バックエンドソースコード
│   ├───main.rs             # アプリケーションエントリとサーバー設定
│   ├───config.rs           # 設定定数（セッション関連）
│   ├───lib.rs              # ライブラリファイルエントリ
│   ├───auth.rs             # 認証モジュールエクスポート
│   ├───handlers.rs         # ハンドラモジュールエクスポート
│   ├───middlewares.rs      # ミドルウェアモジュールエクスポート
│   ├───models.rs           # モデルモジュールエクスポート
│   ├───auth/               # 認証モジュール
│   │   ├───session_store.rs # Redisセッションストア管理
│   │   └───status.rs       # 認証状態チェック
│   ├───handlers/           # HTTPリクエストハンドラ
│   │   ├───user_handler.rs # ユーザー登録/ログイン/削除/パスワードリセット処理
│   │   ├───session_handler.rs # セッション管理処理
│   │   └───github_handler.rs # GitHub API統合処理
│   ├───middlewares/        # ミドルウェア
│   │   └───session.rs      # セッション認証ミドルウェア
│   └───models/             # データモデル
│       ├───response.rs     # APIレスポンスモデル
│       ├───session.rs      # セッション関連モデル
│       └───user.rs         # ユーザー関連モデル
├───frontend/               # フロントエンドソースコード
│   ├───app/                # Next.jsアプリケーションディレクトリ
│   │   ├───layout.tsx      # アプリケーションレイアウト
│   │   ├───page.tsx        # メインページ
│   │   ├───globals.css     # グローバルスタイル
│   │   ├───dashboard/      # ダッシュボードページ
│   │   │   └───page.tsx    # ユーザーダッシュボード
│   │   ├───login/          # ログインページ
│   │   │   └───page.tsx    # ログインインターフェース
│   │   ├───terms/          # 利用規約ページ
│   │   │   ├───page.tsx    # 利用規約内容
│   │   │   └───layout.tsx  # 利用規約レイアウト
│   │   └───privacy/        # プライバシーポリシーページ
│   │       ├───page.tsx    # プライバシーポリー内容
│   │       └───layout.tsx  # プライバシポリシーレイアウト
│   ├───components/         # Reactコンポーネント
│   │   └───ui/             # UIコンポーネント
│   │       ├───Navbar.tsx  # ナビゲーションバーコンポーネント
│   │       ├───main.tsx    # メインコンテンツコンポーネント
│   │       ├───card.tsx    # カードコンポーネント
│   │       ├───button.tsx  # ボタンコンポーネント
│   │       ├───input.tsx   # 入力フィールドコンポーネント
│   │       ├───label.tsx   # ラベルコンポーネント
│   │       ├───alert-dialog.tsx # ダイアログコンポーネント
│   │       └───LayoutWrapper.tsx # レイアウトラッパーコンポーネント
│   ├───lib/                # ツールライブラリ
│   │   └───utils.ts        # 汎用ツール関数
│   ├───public/             # 静的リソース
│   ├───package.json        # フロントエンド依存設定
│   ├───tsconfig.json       # TypeScript設定
│   ├───tailwind.config.js  # Tailwind CSS設定
│   └───next.config.ts      # Next.js設定
├───tests/                  # テストディレクトリ
│   ├───mod.rs              # テストモジュールエントリ
│   ├───common.rs           # テスト共通ツール
│   ├───basic_tests.rs      # 基本テスト
│   ├───integration_tests.rs # 統合テスト
│   ├───model_tests.rs      # モデルテスト
│   ├───user_handler_tests.rs # ユーザーハンドラテスト
│   ├───user_handler_tests_simple.rs # 簡略化ユーザーハンドラテスト
│   ├───session_handler_tests.rs # セッションハンドラテスト
│   ├───session_handler_tests_simple.rs # 簡略化セッションハンドラテスト
│   ├───session_store_tests.rs # セッションストアテスト
│   ├───session_middleware_tests.rs # ミドルウェアテスト
│   ├───session_middleware_tests_simple.rs # 簡略化ミドルウェアテスト
│   ├───github_handler_tests.rs # GitHubハンドラテスト
│   ├───simple_integration_tests.rs # 簡略化統合テスト
│   └───README.md           # テスト説明ドキュメント
├───docker-compose.yml      # Dockerコンテナオーケストレーション
├───Dockerfile              # バックエンドDockerイメージビルド
├───docker.env              # Docker環境変数
├───.dockerignore           # Docker無視ファイル
├───Cargo.toml              # Rustプロジェクト設定
├───package.json            # ルートディレクトリ依存設定
├───LICENSE                 # プロジェクトライセンス
├───README.en.md            # 英語README
├───README.zh.md            # 中国語README
├───PROJECT_PROGRESS.md     # プロジェクト進捗レポート
├───TASK.md                 # 開発タスクと注意事項
└───TEST_ISSUES_ANALYSIS.md # テスト問題分析レポート
```

## ビルドと実行

### 環境要件
- Rust 2021 Edition
- PostgreSQLデータベース
- Redisサーバー
- Node.js 20+ (フロントエンド開発)
- Docker & Docker Compose (オプション、コンテナ化デプロイ用)

### 環境設定
プロジェクトは`.env`ファイル内の以下の環境変数に依存します：
```bash
DATABASE_URL=postgres://ユーザー名:パスワード@localhost:5432/データベース名
REDIS_URL=redis://localhost:6379
```

### ビルドコマンド

#### バックエンド
```bash
# プロジェクトビルド
cargo build

# プロジェクト実行（開発モード）
cargo run

# 本番ビルド
cargo build --release

# コードチェック（ビルドなし）
cargo check

# コードフォーマット
cargo fmt

# Clippyチェック実行
cargo clippy

# テスト実行
cargo test
```

#### フロントエンド
```bash
# フロントエンドディレクトリに移動
cd frontend

# 依存インストール
npm install
# またはpnpmを使用
pnpm install

# 開発サーバー実行
npm run dev
# または
pnpm dev

# 本番バージョンビルド
npm run build
# または
pnpm build

# 本番サーバー起動
npm run start
# または
pnpm start

# コードチェック
npm run lint
# または
pnpm lint
```

### サービス実行

#### 方法1：ローカル実行（推奨）
```bash
# バックエンドサーバー起動（デフォルトポート8080）
cargo run

# フロントエンドサーバー起動（デフォルトポート3000）
cd frontend && npm run dev
```

#### 方法2：Docker実行（問題あり）
```bash
# Docker Composeですべてのサービス起動
docker-compose up -d

# サービス状態確認
docker-compose ps

# サービス停止
docker-compose down
```

**注意**: 現在のDockerデプロイにはビルド時間の長さ、プラットフォーム互換性、ネットワーク設定、データベーストランザクションなどの問題があります。`cargo run`でプロジェクトを直接起動することを推奨します。

サーバーは`http://127.0.0.1:8080`（バックエンド）、フロントエンドは`http://localhost:3000`でアクセスできます。

## APIエンドポイント

### パブリックエンドポイント
- `POST /signup` - ユーザー登録
  - リクエストボディ：`{"username": "string", "email": "string", "password": "string"}`
  - レスポンス：登録成功メッセージ

- `POST /login` - ユーザーログイン
  - リクエストボディ：`{"email": "string", "password": "string"}`
  - レスポンス：ログイン成功メッセージとセッション情報

- `POST /reset-password` - パスワードリセット
  - リクエストボディ：`{"email": "string", "new_password": "string"}`
  - レスポンス：パスワードリセット成功メッセージ

- `DELETE /delete-user` - ユーザー削除
  - リクエストボディ：`{"email": "string", "password": "string"}`
  - レスポンス：ユーザー削除成功メッセージ

- `GET /github/stars/{owner}/{repo}` - GitHubリポジトリスター数取得
  - パラメータ：owner（リポジトリ所有者）、repo（リポジトリ名）
  - レスポンス：スター数

### 認証エンドポイント（Bearer Token必須）
- `POST /logout` - ユーザーログアウト
  - ヘッダー: `Authorization: Bearer <token>`
  - レスポンス：ログアウト成功メッセージ

- `GET /api/profile` - ユーザー情報取得
  - ヘッダー: `Authorization: Bearer <token>`
  - レスポンス：ユーザープロフィール情報

- `GET /auth/status` - 認証状態チェック
  - ヘッダー: `Authorization: <session_id>`
  - レスポンス：ユーザーIDまたは認証失敗情報

## 開発規約

### コードスタイル
- 厳格なセキュアコーディング実践（panicを引き起こす可能性のある操作の無効化）
- エラーハンドリングに`anyhow`を使用
- 非同期関数を優先
- 明確な型注釈
- モジュラー設計、明確な責任分離
- 構造化ログ記録に`tracing`を使用

### セッション管理
- セッションはRedisに保存され、キープレフィックスは`session:`
- セッション有効期限は1時間（3600秒）
- UUIDをセッションIDとして使用
- セッションID形式：`user_<uuid>`
- フロントエンドはlocalStorageにsession_idを保存

### セキュリティ実践
- パスワードはArgon2ハッシュアルゴリズム、ランダムsalt付き
- 統一されたエラーレスポンス（情報漏洩防止）
- Bearer Token認証
- 入力検証とクリーンアップ
- 厳格なコードセキュリティポリシー（lintsで安全でない操作を無効化）
- CORS設定で許可されるオリジンとメソッドを制限

### データベース操作
- SQLxを使用したタイプセーフなデータベース操作
- コネクションプール管理
- 非同期クエリ操作
- コンパイル時SQLチェック（macros経由）

### フロントエンド開発規約
- TypeScriptを使用したタイプセーフ開発
- コンポーネント化設計、Reactベストプラクティスに準拠
- Tailwind CSSを使用したスタイル開発
- Axiosを使用したHTTPリクエスト
- レスポンシブデザイン、モバイル対応
- Radix UIコンポーネントライブラリでアクセシビリティを確保

## テスト

プロジェクトにはすべてのコア機能をカバーする完全なテストスイートが含まれています：

### テスト実行
```bash
# すべてのテスト実行
cargo test

# 特定テストモジュール実行
cargo test user_handler
cargo test session_handler
cargo test session_store
cargo test session_middleware
cargo test github_handler
cargo test integration

# 単一テスト実行
cargo test test_user_signup_success

# テスト出力表示
cargo test -- --nocapture

# テスト実行（シングルスレッド）
cargo test -- --test-threads=1
```

### テスト環境変数
```bash
# テストデータベースURL（オプション、デフォルト値あり）
export TEST_DATABASE_URL="postgres://username:password@localhost:5432/test_db"

# テストRedis URL（オプション、デフォルト値あり）
export TEST_REDIS_URL="redis://localhost:6379"
```

### テストカバレッジ範囲
- **基本テスト**：システム基本機能テスト
- **ユーザーハンドラテスト**：登録、ログイン、パスワードリセット、ユーザー削除成功/失敗シナリオ
- **セッションハンドラテスト**：ログアウト、ユーザープロフィール取得
- **セッションストアテスト**：セッション作成、取得、更新、破棄
- **ミドルウェアテスト**：Token認証、期限切れ処理
- **GitHubハンドラテスト**：GitHub API統合機能
- **統合テスト**：完全なユーザーライフサイクルテスト
- **同時実行テスト**：マルチスレッド安全性検証

### テスト問題分析
現在のテストスイートにはいくつかのアプリケーションロジック問題が存在します：
- 統合テストの一部が認証ロジック問題により失敗
- セッションハンドラテストのエラーハンドリングメカニズム改善が必要
- 同時実行操作処理の最適化が必要

詳細な問題分析については`TEST_ISSUES_ANALYSIS.md`ファイルを参照してください。

## 依存関係管理

### 主要依存項目（Cargo.tomlより）
- `actix-web` 4 - Webフレームワーク
- `actix-web-httpauth` 0.8 - HTTP認証ミドルウェア
- `actix-cors` 0.6 - CORSミドルウェア
- `sqlx` 0.8.6 - データベースツールキット（PostgreSQL + UUID + macros）
- `redis` 0.23.1 - Redisクライアント（tokio-comp + connection-manager）
- `argon2` 0.5 - パスワードハッシュ
- `rand_core` 0.10.0-rc-2 - 乱数生成
- `serde` 1.0 - シリアライゼーション/デシリアライゼーション（derive特性）
- `serde_json` 1 - JSONサポート
- `tokio` 1 - 非同期ランタイム（full features）
- `chrono` 0.4 - 日付時刻処理（serdeサポート）
- `uuid` 1 - UUID生成（serde + v4特性）
- `dotenvy` 0.15.6 - 環境変数読み込み
- `anyhow` 1.0.100 - エラーハンドリング
- `tracing` 0.1 - 構造化ログ
- `tracing-subscriber` 0.3 - ログサブスクライバー
- `reqwest` 0.12.24 - HTTPクライアント（JSONサポート）

### フロントエンド主要依存項目
- `next` 16.0.3 - React全栈フレームワーク
- `react` 19.2.0 - UIライブラリ
- `react-dom` 19.2.0 - React DOMレンダリング
- `typescript` 5 - TypeScript言語サポート
- `tailwindcss` 4 - CSSフレームワーク
- `@radix-ui/react-slot` 1.2.4 - UIコンポーネントベース
- `@radix-ui/react-label` 2.1.8 - ラベルコンポーネント
- `@radix-ui/react-alert-dialog` 1.1.15 - ダイアログコンポーネント
- `lucide-react` 0.553.0 - アイコンライブラリ
- `axios` 1.13.2 - HTTPクライアント
- `class-variance-authority` 0.7.1 - クラスバリアントツール
- `clsx` 2.1.1 - クラス名ツール
- `tailwind-merge` 3.4.0 - Tailwindクラス名マージ
- `tw-animate-css` 1.4.0 - アニメーションライブラリ

## プロジェクト特性

- **ゼロトラストセキュリティ**：厳格なコードセキュリティポリシー、panicを引き起こす可能性のあるすべての操作を無効化
- **完全テストカバレッジ**：単体テストと統合テストを含み、コード品質を確保
- **モジュラーアーキテクチャ**：明確なモジュール分割、保守と拡張を容易に
- **非同期優先**：完全非同期設計、高い同時実行性能を提供
- **タイプセーフ**：RustタイプシステムとSQLxコンパイル時チェックを活用
- **本番対応**：完全なエラーハンドリング、ログ記録、セキュリティ対策を含む
- **フロントエンドとバックエンド分離**：独立したAPIサービスとフロントエンドアプリケーション、チームコラボレーションを容易に
- **コンテナ化サポート**：Docker設定を提供、デプロイプロセスを簡素化（現在最適化の余地あり）
- **モダンフロントエンド**：最新のReact 19とNext.js 16技術を使用
- **GitHub統合**：GitHub API統合例を提供
- **認証状態チェック**：認証状態エンドポイントを提供、フロントエンドセッション管理を容易に

## デプロイ

### Dockerデプロイ
プロジェクトはDocker設定を提供していますが、現在いくつかの問題を最適化する必要があります：

```bash
# すべてのサービスをビルドして起動
docker-compose up -d --build

# ログ確認
docker-compose logs -f

# サービス停止
docker-compose down
```

### ローカルデプロイ（推奨）
```bash
# ローカルPostgreSQLとRedisサービスが実行中であることを確認
# バックエンドサービス起動
cargo run

# フロントエンドサービス起動
cd frontend && npm run dev
```

### 環境変数設定
本番環境では以下の環境変数を設定する必要があります：
- `DATABASE_URL`: PostgreSQLデータベース接続文字列
- `REDIS_URL`: Redis接続文字列

### ポート設定
- バックエンドサービス：8080
- PostgreSQL：5432
- Redis：6379
- フロントエンド：3000（開発環境）

## 現在のステータスと問題

### 完了機能
- ✅ ユーザー登録とログインシステム
- ✅ セッション管理と認証
- ✅ Redisセッションストア
- ✅ PostgreSQLデータベース統合
- ✅ 基本フロントエンドインターフェース（ログインページ、ダッシュボード）
- ✅ GitHub API統合
- ✅ 完全なバックエンドテストスイート
- ✅ フロントエンドUIコンポーネントライブラリ
- ✅ 日本語化されたログインページと法務ページ

### 既知の問題
- Dockerデプロイにパフォーマンスと互換性の問題が存在
- 一部の統合テストがアプリケーションロジック問題により失敗
- 認証ミドルウェアのエラーハンドリング改善が必要
- フロントエンドログインページにコード重複の問題が存在

### 開発中
- ユーザープロフィール管理機能
- メール認証システム
- OAuth統合（Google、GitHub）
- APIレート制限機能

## 将来の計画

### 短期目標
- [ ] Dockerデプロイ問題の修正
- [ ] テストスイートのアプリケーションロジック問題の改善
- [ ] フロントエンドログインページコードの改善
- [ ] ユーザープロフィール更新機能の追加
- [ ] メール認証の実装
- [ ] OAuth統合の追加（Google、GitHub）
- [ ] フロントエンドページと機能の改善

### 中期目標
- [ ] ロールベースアクセス制御（RBAC）の実装
- [ ] APIレート制限機能の追加
- [ ] メールサービス統合
- [ ] ファイルストレージ機能の追加
- [ ] リアルタイム通信（WebSocket）の実装
- [ ] 監視とログ分析の追加

### 長期目標
- [ ] マルチテナントアーキテクチャのサポート
- [ ] 完全な開発者ツールチェーンの提供
- [ ] マイクロサービスアーキテクチャの実装
- [ ] GraphQLサポートの追加
- [ ] データ分析とレポート機能の実装

## 開発注意事項

- `cargo run`でプロジェクトを直接起動することを推奨し、Dockerは使用しないでください
- 開発時はPostgreSQLとRedisサービスが実行中であることを確認してください
- フロントエンド開発時はTypeScriptタイプチェックを使用してください
- コードコミット前に`cargo fmt`と`cargo clippy`を実行し、コード品質を確保してください
- `cargo test`を実行し、すべてのテストが通過することを確認してください

---

**ドキュメント更新日**: 2025-11-18  
**プロジェクトバージョン**: 0.1.0  
**ステータス**: 開発中（MVP段階完了、機能の最適化と拡張中）