use actix_web::{test, web, App};
use sqlx::{PgPool, Pool, Postgres};
use std::env;
use uuid::Uuid;

/// 测试数据库配置
pub struct TestConfig {
    pub database_url: String,
    pub redis_url: String,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            database_url: env::var("TEST_DATABASE_URL")
                .unwrap_or_else(|_| "postgres://kayano:121381@localhost:5432/test_postgres".to_string()),
            redis_url: env::var("TEST_REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
        }
    }
}

/// 创建测试数据库连接池
pub async fn create_test_pool() -> Pool<Postgres> {
    let config = TestConfig::default();
    let pool = PgPool::connect(&config.database_url).await.expect("Failed to create test pool");
    
    // 清理测试数据
    cleanup_test_data(&pool).await;
    
    pool
}

/// 创建测试 Redis 客户端
pub fn create_test_redis_client() -> redis::Client {
    let config = TestConfig::default();
    redis::Client::open(config.redis_url).expect("Failed to create test Redis client")
}

/// 清理测试数据
async fn cleanup_test_data(pool: &Pool<Postgres>) {
    sqlx::query("DELETE FROM users WHERE email LIKE '%@test.com'")
        .execute(pool)
        .await
        .ok();
}

/// 创建测试应用实例
pub async fn create_test_app() -> impl actix_web::dev::ServiceFactory<
    actix_web::dev::ServiceRequest,
    Config = (),
    Response = actix_web::dev::ServiceResponse,
    Error = actix_web::Error,
    InitError = (),
> {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    
    App::new()
        .app_data(web::Data::new(pool))
        .app_data(web::Data::new(redis_client))
        .service(orpheus::handlers::user_handler::user_signup)
        .service(orpheus::handlers::user_handler::user_login)
        .service(orpheus::handlers::session_handler::user_logout)
        .service(
            web::scope("/api")
                .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(
                    orpheus::middlewares::session::session_validator,
                ))
                .service(orpheus::handlers::session_handler::user_profile),
        )
}

/// 生成测试用户数据
pub fn create_test_user() -> orpheus::models::user::SignupRequest {
    orpheus::models::user::SignupRequest {
        username: format!("test_user_{}", uuid::Uuid::new_v4()),
        email: format!("{}@test.com", uuid::Uuid::new_v4()),
        password: "test_password_123".to_string(),
    }
}

/// 生成测试登录数据
pub fn create_test_login(email: String) -> orpheus::models::user::LoginRequest {
    orpheus::models::user::LoginRequest {
        email,
        password: "test_password_123".to_string(),
    }
}

/// 创建测试会话
pub async fn create_test_session(
    redis_client: &redis::Client,
    user_id: Uuid,
) -> String {
    let session_store = orpheus::auth::session_store::SessionStore::new(redis_client.clone());
    session_store
        .create_session(user_id)
        .await
        .expect("Failed to create test session")
}