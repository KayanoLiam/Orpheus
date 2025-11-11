// 简单的单元测试，不依赖外部服务
use orpheus::auth::session_store::SessionStore;
use uuid::Uuid;

#[tokio::test]
async fn test_session_store_creation() {
    // 创建一个 Redis 客户端（即使连接失败，我们只是测试结构体创建）
    let redis_url = "redis://localhost:6379";
    let redis_client = redis::Client::open(redis_url).expect("Failed to create Redis client");
    
    // 测试 SessionStore 创建
    let _session_store = SessionStore::new(redis_client);
    
    // 如果没有 panic，说明创建成功
    assert!(true);
}

#[test]
fn test_uuid_generation() {
    // 测试 UUID 生成
    let uuid1 = Uuid::new_v4();
    let uuid2 = Uuid::new_v4();
    
    // 验证两个 UUID 不同
    assert_ne!(uuid1, uuid2);
    
    // 验证 UUID 可以转换为字符串
    let uuid_str = uuid1.to_string();
    assert!(!uuid_str.is_empty());
    
    // 验证 UUID 字符串可以解析回来
    let parsed_uuid = Uuid::parse_str(&uuid_str).unwrap();
    assert_eq!(uuid1, parsed_uuid);
}

#[test]
fn test_user_model_creation() {
    // 测试用户模型创建
    let signup_request = orpheus::models::user::SignupRequest {
        username: "test_user".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };
    
    assert_eq!(signup_request.username, "test_user");
    assert_eq!(signup_request.email, "test@example.com");
    assert_eq!(signup_request.password, "password123");
    
    let login_request = orpheus::models::user::LoginRequest {
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };
    
    assert_eq!(login_request.email, "test@example.com");
    assert_eq!(login_request.password, "password123");
}