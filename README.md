# Orpheus

[English](./README.en.md) | [简体中文](./README.zh.md)

## プロジェクト概要

**Orpheus は Backend-as-a-Service (BaaS) プラットフォームです**。Supabase のように、開発者が PostgreSQL データベースを用意するだけで、すぐに使える REST API、リアルタイムデータ購読、オブジェクトストレージなどの機能を提供することを目指しています。

### 🎯 コアミッション

開発者は以下のステップだけで完全なバックエンドを手に入れることができます：

1. PostgreSQL データベーステーブルを作成
2. (オプション) Row Level Security (RLS) ポリシーを設定
3. Orpheus を起動

すると自動的に以下が利用可能になります：
- ✅ 完全な RESTful API（全テーブルに対して）
- ✅ リアルタイムデータ購読（WebSocket）
- ✅ S3 互換のオブジェクトストレージ
- ✅ データベース管理 API
- ✅ (オプション) GraphQL API
- ✅ (オプション) Serverless Functions

**バックエンドコードを書く必要はありません！**

---

## 🏗️ アーキテクチャ

```
┌─────────────────────────────────────────────────────────────┐
│                   API Gateway (Actix-Web)                    │
│                 (ルーティング、認証、レート制限、CORS)           │
└──────────┬─────────────┬──────────────┬─────────────┬────────┘
           │             │              │             │
     ┌─────▼─────┐ ┌────▼─────┐  ┌─────▼──────┐ ┌───▼────────┐
     │   REST    │ │ Realtime │  │  Storage   │ │   Meta     │
     │    API    │ │  Server  │  │    API     │ │    API     │
     │(自動CRUD) │ │(WebSocket)│  │ (S3互換)   │ │(DB管理)    │
     └─────┬─────┘ └────┬─────┘  └─────┬──────┘ └───┬────────┘
           │             │              │             │
           └─────────────┴──────────────┴─────────────┘
                              │
                    ┌─────────▼──────────┐
                    │   PostgreSQL DB    │
                    │   (データストア)      │
                    └────────────────────┘
```

---

## 🚀 技術スタック

### バックエンド
- **フレームワーク**: Actix-Web 4 (Rust 2021)
- **データベース**: PostgreSQL (SQLx 0.8.6 経由)
- **キャッシュ**: Redis 0.23.1
- **非同期ランタイム**: Tokio
- **HTTP クライアント**: Reqwest

### セキュリティポリシー
- 厳格なコードセキュリティ（unwrap、expect、panic を禁止）
- 型安全なデータベース操作
- 完全なエラーハンドリング

---

## 📦 現在の状態

### ✅ 実装済み
- PostgreSQL データベース接続
- Redis キャッシュシステム
- 基本的な HTTP サーバー
- GitHub API 統合（サンプル）

### 🚧 開発中（優先順位順）

#### 第一段階：Auto REST API ⭐⭐⭐
- [ ] データベース Schema インスペクター
- [ ] 動的 CRUD エンドポイント生成
- [ ] クエリビルダー（フィルター、ソート、ページネーション）
- [ ] Row Level Security (RLS) 統合

#### 第二段階：Realtime ⭐⭐⭐
- [ ] PostgreSQL 論理レプリケーションリスナー
- [ ] WebSocket サーバー
- [ ] データベース変更通知
- [ ] Broadcast と Presence

#### 第三段階：Storage ⭐⭐
- [ ] S3 互換ストレージバックエンド
- [ ] ファイルアップロード/ダウンロード API
- [ ] Bucket 管理
- [ ] 画像最適化

#### 第四段階：Meta API ⭐⭐
- [ ] テーブル管理 API
- [ ] カラム/インデックス/制約管理
- [ ] SQL クエリエグゼキューター

#### 第五段階：拡張機能 ⭐
- [ ] Edge Functions (Serverless)
- [ ] GraphQL API
- [ ] パフォーマンス最適化
- [ ] モニタリングとメトリクス

---

## 🔧 ビルドと実行

### 環境要件
- Rust 2021 Edition
- PostgreSQL
- Redis
- Node.js 20+ (フロントエンド開発の場合)

### 環境設定

`.env` ファイルを作成：

```bash
DATABASE_URL=postgres://username:password@localhost:5432/database
REDIS_URL=redis://localhost:6379
```

### 実行

```bash
# プロジェクトのビルドと実行
cargo run

# テスト実行
cargo test

# コードフォーマット
cargo fmt

# Clippy チェック
cargo clippy
```

サーバーは `http://127.0.0.1:8080` で起動します。

---

## 📚 サンプルとアーカイブ

### サンプルプロジェクト

`examples/` ディレクトリには、Orpheus を使用したサンプルプロジェクトがあります：

#### `examples/authentication/`
完全なユーザー認証システムの例：
- ユーザー登録/ログイン
- セッション管理
- パスワードリセット
- Bearer Token 認証

詳細は [examples/authentication/README.md](./examples/authentication/README.md) を参照してください。

### アーカイブプロジェクト

`archived_projects/` ディレクトリには、以前開発された関連プロジェクトがあります：

#### `archived_projects/orpheus-admin-panel/`
Next.js + React で構築された管理画面 UI：
- Awwwards レベルのデザイン
- 3D 背景アニメーション
- ダッシュボード UI

詳細は [archived_projects/orpheus-admin-panel/README.md](./archived_projects/orpheus-admin-panel/README.md) を参照してください。

---

## 🎯 API エンドポイント（計画中）

### REST API (開発中)

```
GET    /rest/v1/{table}              - レコード検索
POST   /rest/v1/{table}              - レコード作成
PATCH  /rest/v1/{table}?{filter}     - レコード更新
DELETE /rest/v1/{table}?{filter}     - レコード削除
```

#### クエリ例

```
# 基本的な検索
GET /rest/v1/users

# フィルター
GET /rest/v1/users?age=gte.18&status=eq.active

# ソート
GET /rest/v1/users?order=created_at.desc

# ページネーション
GET /rest/v1/users?limit=10&offset=20

# 関連テーブルの結合
GET /rest/v1/posts?select=*,author(name,email)
```

### Realtime API (開発中)

```javascript
// WebSocket 接続
const ws = new WebSocket('ws://localhost:8080/realtime/v1');

// テーブルの変更を購読
ws.send(JSON.stringify({
  topic: 'realtime:public:users',
  event: '*',  // INSERT | UPDATE | DELETE
}));

// 変更通知を受信
ws.onmessage = (event) => {
  console.log('Database change:', JSON.parse(event.data));
};
```

### Storage API (開発中)

```
POST   /storage/v1/object/{bucket}/{path}     - ファイルアップロード
GET    /storage/v1/object/{bucket}/{path}     - ファイルダウンロード
DELETE /storage/v1/object/{bucket}/{path}     - ファイル削除
POST   /storage/v1/object/sign/{bucket}/{path} - 署名付き URL 生成
```

### Meta API (開発中)

```
GET    /meta/v1/tables                  - テーブル一覧
POST   /meta/v1/tables                  - テーブル作成
PATCH  /meta/v1/tables/{id}             - テーブル構造変更
DELETE /meta/v1/tables/{id}             - テーブル削除
```

---

## 🔐 セキュリティとアクセス制御

### Row Level Security (RLS)

PostgreSQL の RLS ポリシーを使用してデータアクセスを制御：

```sql
-- RLS を有効化
ALTER TABLE posts ENABLE ROW LEVEL SECURITY;

-- ポリシー作成：ユーザーは自分の投稿のみ閲覧可能
CREATE POLICY "Users can view own posts"
ON posts FOR SELECT
USING (auth.uid() = user_id);
```

---

## 📖 ドキュメント

- [Supabase Core Roadmap](./SUPABASE_CORE_ROADMAP.md) - 詳細な開発ロードマップ
- [Project Progress](./PROJECT_PROGRESS.md) - プロジェクト進捗状況
- [Test Issues Analysis](./TEST_ISSUES_ANALYSIS.md) - テスト問題分析
- [Task Notes](./TASK.md) - 開発タスクと注意事項

---

## 🤝 コントリビューション

Orpheus はオープンソースプロジェクトです。貢献を歓迎します！

### 開発の優先順位

1. **最高優先度**: Auto REST API の実装
2. **第二優先度**: Realtime 購読システム
3. **第三優先度**: Storage サービス

詳細は [SUPABASE_CORE_ROADMAP.md](./SUPABASE_CORE_ROADMAP.md) を参照してください。

---

## 📝 ライセンス

MIT License

---

## 🌟 インスピレーション

Orpheus は以下のプロジェクトからインスピレーションを得ています：

- [Supabase](https://supabase.com/) - オープンソース Firebase 代替
- [PostgREST](https://postgrest.org/) - PostgreSQL の REST API 生成
- [Realtime](https://github.com/supabase/realtime) - PostgreSQL リアルタイム購読

---

**ドキュメント更新日**: 2025-11-20  
**プロジェクトバージョン**: 0.2.0  
**ステータス**: 🏗️ BaaS コア機能開発中（アーキテクチャ再編成完了）