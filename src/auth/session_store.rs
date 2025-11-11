// 导入 Redis 异步命令 trait
use redis::AsyncCommands;
// 导入 UUID 生成和解析
use uuid::Uuid;
// 导入会话相关配置常量
use crate::config::{SESSION_KEY_PREFIX, SESSION_EXPIRY_SECONDS};

/// Session 管理器
/// 
/// 负责用户会话的创建、验证、刷新和销毁操作
/// 使用 Redis 作为会话存储后端，支持分布式部署
pub struct SessionStore {
    redis: redis::Client, // Redis 客户端连接
}

impl SessionStore {
    /// 创建新的 SessionStore 实例
    /// 
    /// # 参数
    /// - `redis`: Redis 客户端连接
    /// 
    /// # 返回
    /// SessionStore 实例
    pub fn new(redis: redis::Client) -> Self {
        Self { redis }
    }

    /// 创建新的用户会话
    /// 
    /// 为指定用户创建一个新的会话，并存储到 Redis 中
    /// 
    /// # 参数
    /// - `user_id`: 用户的 UUID 标识
    /// 
    /// # 返回
    /// - `Ok(String)`: 新创建的会话 ID
    /// - `Err(anyhow::Error)`: Redis 操作失败时的错误信息
    pub async fn create_session(&self, user_id: Uuid) -> anyhow::Result<String> {
        // 建立 Redis 异步连接
        let mut redis_conn = self.redis.get_async_connection().await?;
        // 生成唯一的会话 ID
        let session_id = Uuid::new_v4().to_string();
        // 构造 Redis 存储键名
        let key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
        
        // 在 Redis 中存储会话信息，设置过期时间
        redis_conn
            .set_ex::<String, String, ()>(key, user_id.to_string(), SESSION_EXPIRY_SECONDS)
            .await?;
        Ok(session_id)
    }

    /// 根据会话 ID 获取用户 ID
    /// 
    /// 从 Redis 中查找指定会话 ID 对应的用户 ID
    /// 
    /// # 参数
    /// - `session_id`: 会话 ID 字符串
    /// 
    /// # 返回
    /// - `Ok(Some(Uuid))`: 找到对应的用户 ID
    /// - `Ok(None)`: 会话不存在或已过期
    /// - `Err(anyhow::Error)`: Redis 操作失败时的错误信息
    pub async fn get_user_id(&self, session_id: &str) -> anyhow::Result<Option<Uuid>> {
        // 建立 Redis 异步连接
        let mut conn = self.redis.get_async_connection().await?;
        // 构造 Redis 存储键名
        let key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
        
        // 从 Redis 获取用户 ID 字符串
        let user_id_opt: Option<String> = conn.get(key).await?;
        // 尝试将字符串转换为 UUID，失败时返回 None
        let user_id_opt = user_id_opt.and_then(|id_str| Uuid::parse_str(&id_str).ok());
        
        Ok(user_id_opt)
    }
    /// 销毁指定的用户会话
    /// 
    /// 从 Redis 中删除指定会话 ID 的记录，实现用户登出
    /// 
    /// # 参数
    /// - `session_id`: 要销毁的会话 ID 字符串
    /// 
    /// # 返回
    /// - `Ok(())`: 会话成功销毁
    /// - `Err(anyhow::Error)`: Redis 操作失败时的错误信息
    pub async fn destroy_session(&self, session_id: &str) -> anyhow::Result<()> {
        // 建立 Redis 异步连接
        let mut conn = self.redis.get_async_connection().await?;
        // 构造 Redis 存储键名
        let key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
        
        // 从 Redis 删除会话记录
        conn.del::<_, ()>(key).await?;
        Ok(())
    }
    
    /// 刷新会话过期时间
    /// 
    /// 为指定会话延长过期时间，实现会话的滑动过期机制
    /// 
    /// # 参数
    /// - `session_id`: 要刷新的会话 ID 字符串
    /// 
    /// # 返回
    /// - `Ok(())`: 会话过期时间成功刷新
    /// - `Err(anyhow::Error)`: Redis 操作失败时的错误信息
    /// 
    /// # 注意
    /// 此方法目前标记为 `#[allow(dead_code)]`，暂未在业务逻辑中使用
    /// 可用于实现"记住我"功能或会话自动延期
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
