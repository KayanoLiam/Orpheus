// 导入 Redis 异步命令 trait
// 自令和7年11.17之后，不再提供中文注释
// Redis 非同期コマンドトレイトをインポート
use redis::AsyncCommands;
// 导入 UUID 生成和解析
// UUID 生成と解析をインポート
use uuid::Uuid;
// 导入会话相关配置常量
// セッション関連の設定定数をインポート
use crate::config::{SESSION_KEY_PREFIX, SESSION_EXPIRY_SECONDS};

/// Session 管理器
/// 
/// 负责用户会话的创建、验证、刷新和销毁操作
/// セッションの作成、検証、更新、破棄操作を担当します
/// 使用 Redis 作为会话存储后端，支持分布式部署
/// Redisをセッションストレージバックエンドとして使用し、分散デプロイをサポートします
pub struct SessionStore {
    redis: redis::Client, // Redis 客户端连接
    // Redis クライアント接続
}

impl SessionStore {
    /// 创建新的 SessionStore 实例
    /// 新しい SessionStore インスタンスを作成します
    /// 
    /// # 参数
    /// - `redis`: Redis 客户端连接
    /// - `redis`: Redis クライアント接続
    /// 
    /// # 返回
    /// SessionStore 实例
    /// SessionStore インスタンス
    pub fn new(redis: redis::Client) -> Self {
        Self { redis }
    }

    /// 创建新的用户会话
    /// 新しいユーザーセッションを作成します
    /// 
    /// 为指定用户创建一个新的会话，并存储到 Redis 中
    /// 指定されたユーザーの新しいセッションを作成し、Redisに保存します
    /// 
    /// # 参数
    /// - `user_id`: 用户的 UUID 标识
    /// - `user_id`: ユーザーのUUID識別子
    /// 
    /// # 返回
    /// - `Ok(String)`: 新创建的会话 ID
    /// - `Ok(String)`: 新しく作成されたセッションID
    /// - `Err(anyhow::Error)`: Redis 操作失败时的错误信息
    /// - `Err(anyhow::Error)`: Redis操作失敗時のエラー情報
    pub async fn create_session(&self, user_id: Uuid) -> anyhow::Result<String> {
        // 建立 Redis 异步连接
        // Redis非同期接続を確立
        let mut redis_conn = self.redis.get_async_connection().await?;
        // 生成唯一的会话 ID
        // 一意のセッションIDを生成
        let session_id = Uuid::new_v4().to_string();
        // 构造 Redis 存储键名
        // Redis保存キー名を構築
        let key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
        
        // 在 Redis 中存储会话信息，设置过期时间
        // Redisにセッション情報を保存し、有効期限を設定
        redis_conn
            .set_ex::<String, String, ()>(key, user_id.to_string(), SESSION_EXPIRY_SECONDS)
            .await?;
        Ok(session_id)
    }

    /// 根据会话 ID 获取用户 ID
    /// セッションIDに基づいてユーザーIDを取得します
    /// 
    /// 从 Redis 中查找指定会话 ID 对应的用户 ID
    /// Redisから指定されたセッションIDに対応するユーザーIDを検索します
    /// 
    /// # 参数
    /// - `session_id`: 会话 ID 字符串
    /// - `session_id`: セッションID文字列
    /// 
    /// # 返回
    /// - `Ok(Some(Uuid))`: 找到对应的用户 ID
    /// - `Ok(Some(Uuid))`: 対応するユーザーIDが見つかった場合
    /// - `Ok(None)`: 会话不存在或已过期
    /// - `Ok(None)`: セッションが存在しないか期限切れの場合
    /// - `Err(anyhow::Error)`: Redis 操作失败时的错误信息
    /// - `Err(anyhow::Error)`: Redis操作失敗時のエラー情報
    pub async fn get_user_id(&self, session_id: &str) -> anyhow::Result<Option<Uuid>> {
        // 建立 Redis 异步连接
        // Redis非同期接続を確立
        let mut conn = self.redis.get_async_connection().await?;
        // 构造 Redis 存储键名
        // Redis保存キー名を構築
        let key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
        
        // 从 Redis 获取用户 ID 字符串
        // RedisからユーザーID文字列を取得
        let user_id_opt: Option<String> = conn.get(key).await?;
        // 尝试将字符串转换为 UUID，失败时返回 None
        // 文字列をUUIDに変換を試行し、失敗時はNoneを返す
        let user_id_opt = user_id_opt.and_then(|id_str| Uuid::parse_str(&id_str).ok());
        
        Ok(user_id_opt)
    }
    /// 销毁指定的用户会话
    /// 指定されたユーザーセッションを破棄します
    /// 
    /// 从 Redis 中删除指定会话 ID 的记录，实现用户登出
    /// Redisから指定されたセッションIDのレコードを削除し、ユーザーログアウトを実現します
    /// 
    /// # 参数
    /// - `session_id`: 要销毁的会话 ID 字符串
    /// - `session_id`: 破棄するセッションID文字列
    /// 
    /// # 返回
    /// - `Ok(())`: 会话成功销毁
    /// - `Ok(())`: セッションが正常に破棄された場合
    /// - `Err(anyhow::Error)`: Redis 操作失败时的错误信息
    /// - `Err(anyhow::Error)`: Redis操作失敗時のエラー情報
    pub async fn destroy_session(&self, session_id: &str) -> anyhow::Result<()> {
        // 建立 Redis 异步连接
        // Redis非同期接続を確立
        let mut conn = self.redis.get_async_connection().await?;
        // 构造 Redis 存储键名
        // Redis保存キー名を構築
        let key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
        
        // 从 Redis 删除会话记录
        // Redisからセッションレコードを削除
        conn.del::<_, ()>(key).await?;
        Ok(())
    }
    
    /// 刷新会话过期时间
    /// 自令和7年11.17之后，不再提供中文注释
    /// セッション有効期限を更新
    /// 
    /// 为指定会话延长过期时间，实现会话的滑动过期机制
    /// 指定されたセッションの有効期限を延長し、セッションのスライド有効期限メカニズムを実現します
    /// 
    /// # 参数
    /// # 引数
    /// - `session_id`: 要刷新的会话 ID 字符串
    /// - `session_id`: 更新するセッションID文字列
    /// 
    /// # 返回
    /// # 戻り値
    /// - `Ok(())`: 会话过期时间成功刷新
    /// - `Ok(())`: セッション有効期限が正常に更新された場合
    /// - `Err(anyhow::Error)`: Redis 操作失败时的错误信息
    /// - `Err(anyhow::Error)`: Redis操作失敗時のエラー情報
    /// 
    /// # 注意
    /// # 注意
    /// 此方法目前标记为 `#[allow(dead_code)]`，暂未在业务逻辑中使用
    /// 可用于实现"记住我"功能或会话自动延期
    /// このメソッドは現在`#[allow(dead_code)]`としてマークされており、ビジネスロジックでは未使用です
    /// 「 remember me 」機能やセッションの自動延長の実装に使用できます
    #[allow(dead_code)]
    pub async fn refresh_session(&self, session_id: &str) -> anyhow::Result<()> {
        // 建立 Redis 异步连接
        let mut conn = self.redis.get_async_connection().await?;
        // 构造 Redis 存储键名
        let key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
        
        // 重新设置键的过期时间
        conn.expire::<_, ()>(key, SESSION_EXPIRY_SECONDS).await?;
        Ok(())
    }
}
