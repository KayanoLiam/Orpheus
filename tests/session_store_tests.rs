// 简化的会话存储测试
use uuid::Uuid;
use orpheus::auth::session_store::SessionStore;
use std::env;

// 测试配置
struct TestConfig {
    #[allow(dead_code)]
    database_url: String,
    redis_url: String,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            database_url: env::var("TEST_DATABASE_URL")
                .unwrap_or_else(|_| "postgres://orpheus_user:secret@localhost:5432/orpheus_db".to_string()),
            redis_url: env::var("TEST_REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
        }
    }
}

// 创建测试 Redis 客户端
fn create_test_redis_client() -> redis::Client {
    let config = TestConfig::default();
    redis::Client::open(config.redis_url).expect("Failed to create test Redis client")
}

// 创建测试会话
#[allow(dead_code)]
async fn create_test_session(
    redis_client: &redis::Client,
    user_id: Uuid,
) -> String {
    let session_store = SessionStore::new(redis_client.clone());
    session_store
        .create_session(user_id)
        .await
        .expect("Failed to create test session")
}

#[tokio::test]
async fn test_session_store_new() {
    let redis_client = create_test_redis_client();
    let session_store = SessionStore::new(redis_client.clone());
    
    // 验证 SessionStore 创建成功
    let test_user_id = Uuid::new_v4();
    let result = session_store.create_session(test_user_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_session_success() {
    let redis_client = create_test_redis_client();
    let session_store = SessionStore::new(redis_client);
    
    let test_user_id = Uuid::new_v4();
    
    // 创建会话
    let session_id = session_store.create_session(test_user_id).await.unwrap();
    
    // 验证会话 ID 不为空
    assert!(!session_id.is_empty());
    
    // 验证会话 ID 是有效的 UUID 格式
    assert!(Uuid::parse_str(&session_id).is_ok());
}

#[tokio::test]
async fn test_get_user_id_existing_session() {
    let redis_client = create_test_redis_client();
    let session_store = SessionStore::new(redis_client);
    
    let test_user_id = Uuid::new_v4();
    
    // 创建会话
    let session_id = session_store.create_session(test_user_id).await.unwrap();
    
    // 获取用户 ID
    let retrieved_user_id = session_store.get_user_id(&session_id).await.unwrap();
    
    // 验证获取的用户 ID 正确
    assert_eq!(retrieved_user_id, Some(test_user_id));
}

#[tokio::test]
async fn test_destroy_session_existing() {
    let redis_client = create_test_redis_client();
    let session_store = SessionStore::new(redis_client);
    
    let test_user_id = Uuid::new_v4();
    
    // 创建会话
    let session_id = session_store.create_session(test_user_id).await.unwrap();
    
    // 验证会话存在
    let retrieved_user_id = session_store.get_user_id(&session_id).await.unwrap();
    assert_eq!(retrieved_user_id, Some(test_user_id));
    
    // 销毁会话
    let destroy_result = session_store.destroy_session(&session_id).await;
    assert!(destroy_result.is_ok());
    
    // 验证会话已被销毁
    let retrieved_user_id = session_store.get_user_id(&session_id).await.unwrap();
    assert_eq!(retrieved_user_id, None);
}

#[tokio::test]
async fn test_refresh_session_existing() {
    let redis_client = create_test_redis_client();
    let session_store = SessionStore::new(redis_client);
    
    let test_user_id = Uuid::new_v4();
    
    // 创建会话
    let session_id = session_store.create_session(test_user_id).await.unwrap();
    
    // 刷新会话
    let refresh_result = session_store.refresh_session(&session_id).await;
    assert!(refresh_result.is_ok());
    
    // 验证会话仍然存在
    let retrieved_user_id = session_store.get_user_id(&session_id).await.unwrap();
    assert_eq!(retrieved_user_id, Some(test_user_id));
}