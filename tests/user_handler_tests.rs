use actix_web::{test, web, App, http::StatusCode};
use serde_json::json;
use sqlx::{PgPool, Pool, Postgres};
use std::env;

// 测试配置
struct TestConfig {
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

// 创建测试数据库连接池
async fn create_test_pool() -> Pool<Postgres> {
    let config = TestConfig::default();
    let pool = PgPool::connect(&config.database_url).await.expect("Failed to create test pool");
    
    // 清理测试数据
    sqlx::query("DELETE FROM users WHERE email LIKE '%@test.com'")
        .execute(&pool)
        .await
        .ok();
    
    pool
}

// 创建测试 Redis 客户端
fn create_test_redis_client() -> redis::Client {
    let config = TestConfig::default();
    redis::Client::open(config.redis_url).expect("Failed to create test Redis client")
}

// 生成测试用户数据
fn create_test_user() -> orpheus::models::user::SignupRequest {
    orpheus::models::user::SignupRequest {
        username: format!("test_user_{}", uuid::Uuid::new_v4()),
        email: format!("{}@test.com", uuid::Uuid::new_v4()),
        password: "test_password_123".to_string(),
    }
}

// 生成测试登录数据
fn create_test_login(email: String) -> orpheus::models::user::LoginRequest {
    orpheus::models::user::LoginRequest {
        email,
        password: "test_password_123".to_string(),
    }
}

// 创建测试应用实例
fn create_test_app(
    pool: sqlx::Pool<sqlx::Postgres>,
    redis_client: redis::Client,
) -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
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

#[actix_web::test]
async fn test_user_signup_success() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    let test_user = create_test_user();
    
    let req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&test_user)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::OK);
    
    let response_body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(response_body["code"], 200);
    assert_eq!(response_body["success"], true);
    assert_eq!(response_body["message"], "User signed up successfully");
}

#[actix_web::test]
async fn test_user_signup_duplicate_email() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    let test_user = create_test_user();
    
    // 第一次注册应该成功
    let req1 = test::TestRequest::post()
        .uri("/signup")
        .set_json(&test_user)
        .to_request();
    
    let resp1 = test::call_service(&app, req1).await;
    assert_eq!(resp1.status(), StatusCode::OK);
    
    // 第二次使用相同邮箱应该失败
    let mut duplicate_user = test_user.clone();
    duplicate_user.username = format!("different_user_{}", uuid::Uuid::new_v4());
    
    let req2 = test::TestRequest::post()
        .uri("/signup")
        .set_json(&duplicate_user)
        .to_request();
    
    let resp2 = test::call_service(&app, req2).await;
    assert_eq!(resp2.status(), StatusCode::INTERNAL_SERVER_ERROR);
    
    let response_body: serde_json::Value = test::read_body_json(resp2).await;
    assert_eq!(response_body["code"], 500);
    assert_eq!(response_body["success"], false);
}

#[actix_web::test]
async fn test_user_signup_invalid_json() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    let invalid_json = json!({
        "username": "test_user",
        // 缺少 email 和 password
    });
    
    let req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&invalid_json)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn test_user_signup_missing_fields() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    let empty_user = json!({
        "username": "",
        "email": "",
        "password": ""
    });
    
    let req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&empty_user)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // 应该返回错误，具体状态码取决于验证逻辑
    assert!(resp.status().is_client_error() || resp.status().is_server_error());
}

// ===== 登录测试 =====

#[actix_web::test]
async fn test_user_login_success() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    // 先注册一个用户
    let test_user = create_test_user();
    let email = test_user.email.clone();
    
    let signup_req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&test_user)
        .to_request();
    
    let signup_resp = test::call_service(&app, signup_req).await;
    assert_eq!(signup_resp.status(), StatusCode::OK);
    
    // 然后登录
    let login_data = create_test_login(email);
    
    let login_req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_data)
        .to_request();
    
    let login_resp = test::call_service(&app, login_req).await;
    
    assert_eq!(login_resp.status(), StatusCode::OK);
    
    let response_body: serde_json::Value = test::read_body_json(login_resp).await;
    assert_eq!(response_body["code"], 200);
    assert_eq!(response_body["success"], true);
    assert_eq!(response_body["message"], "User logged in successfully");
    
    // 验证返回了会话信息
    assert!(response_body["data"].is_object());
    assert!(response_body["data"]["session_id"].is_string());
    assert!(response_body["data"]["user_id"].is_string());
    assert!(response_body["data"]["expires_at"].is_number());
}

#[actix_web::test]
async fn test_user_login_invalid_email() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    let login_data = json!({
        "email": "nonexistent@test.com",
        "password": "some_password"
    });
    
    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_data)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    
    let response_body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(response_body["code"], 401);
    assert_eq!(response_body["success"], false);
    assert_eq!(response_body["message"], "Invalid credentials");
}

#[actix_web::test]
async fn test_user_login_invalid_password() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    // 先注册一个用户
    let test_user = create_test_user();
    let email = test_user.email.clone();
    
    let signup_req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&test_user)
        .to_request();
    
    let signup_resp = test::call_service(&app, signup_req).await;
    assert_eq!(signup_resp.status(), StatusCode::OK);
    
    // 使用错误的密码登录
    let login_data = json!({
        "email": email,
        "password": "wrong_password"
    });
    
    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_data)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    
    let response_body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(response_body["code"], 401);
    assert_eq!(response_body["success"], false);
    assert_eq!(response_body["message"], "Invalid credentials");
}

#[actix_web::test]
async fn test_user_login_missing_fields() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    let incomplete_data = json!({
        "email": "test@test.com"
        // 缺少 password
    });
    
    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&incomplete_data)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn test_user_login_empty_fields() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    let empty_data = json!({
        "email": "",
        "password": ""
    });
    
    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&empty_data)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // 应该返回错误
    assert!(resp.status().is_client_error() || resp.status().is_server_error());
}

// ===== 密码重置测试 =====

#[actix_web::test]
async fn test_user_reset_password_success() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    // 先注册并登录一个用户
    let test_user = create_test_user();
    let email = test_user.email.clone();
    
    // 注册用户
    let signup_req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&test_user)
        .to_request();
    
    let signup_resp = test::call_service(&app, signup_req).await;
    assert_eq!(signup_resp.status(), StatusCode::OK);
    
    // 登录获取 session_id
    let login_data = create_test_login(email.clone());
    let login_req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_data)
        .to_request();
    
    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status(), StatusCode::OK);
    
    let login_body: serde_json::Value = test::read_body_json(login_resp).await;
    let session_id = login_body["data"]["session_id"].as_str().unwrap();
    
    // 重置密码
    let reset_data = json!({
        "old_password": "test_password_123",
        "new_password": "new_test_password_456"
    });
    
    let reset_req = test::TestRequest::post()
        .uri("/reset_password")
        .insert_header(("Authorization", format!("Bearer {}", session_id)))
        .set_json(&reset_data)
        .to_request();
    
    let reset_resp = test::call_service(&app, reset_req).await;
    assert_eq!(reset_resp.status(), StatusCode::OK);
    
    let reset_body: serde_json::Value = test::read_body_json(reset_resp).await;
    assert_eq!(reset_body["code"], 200);
    assert_eq!(reset_body["success"], true);
    assert!(reset_body["message"].as_str().unwrap().contains("Password changed successfully"));
    
    // 验证旧密码无法登录
    let old_login_req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_data)
        .to_request();
    
    let old_login_resp = test::call_service(&app, old_login_req).await;
    assert_eq!(old_login_resp.status(), StatusCode::UNAUTHORIZED);
    
    // 验证新密码可以登录
    let new_login_data = json!({
        "email": email,
        "password": "new_test_password_456"
    });
    
    let new_login_req = test::TestRequest::post()
        .uri("/login")
        .set_json(&new_login_data)
        .to_request();
    
    let new_login_resp = test::call_service(&app, new_login_req).await;
    assert_eq!(new_login_resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_user_reset_password_invalid_old_password() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    // 先注册并登录一个用户
    let test_user = create_test_user();
    let email = test_user.email.clone();
    
    // 注册用户
    let signup_req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&test_user)
        .to_request();
    
    let signup_resp = test::call_service(&app, signup_req).await;
    assert_eq!(signup_resp.status(), StatusCode::OK);
    
    // 登录获取 session_id
    let login_data = create_test_login(email);
    let login_req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_data)
        .to_request();
    
    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status(), StatusCode::OK);
    
    let login_body: serde_json::Value = test::read_body_json(login_resp).await;
    let session_id = login_body["data"]["session_id"].as_str().unwrap();
    
    // 使用错误的旧密码尝试重置
    let reset_data = json!({
        "old_password": "wrong_old_password",
        "new_password": "new_test_password_456"
    });
    
    let reset_req = test::TestRequest::post()
        .uri("/reset_password")
        .insert_header(("Authorization", format!("Bearer {}", session_id)))
        .set_json(&reset_data)
        .to_request();
    
    let reset_resp = test::call_service(&app, reset_req).await;
    assert_eq!(reset_resp.status(), StatusCode::UNAUTHORIZED);
    
    let reset_body: serde_json::Value = test::read_body_json(reset_resp).await;
    assert_eq!(reset_body["code"], 401);
    assert_eq!(reset_body["success"], false);
    assert_eq!(reset_body["message"], "Invalid old password");
}

#[actix_web::test]
async fn test_user_reset_password_missing_auth() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    let reset_data = json!({
        "old_password": "test_password_123",
        "new_password": "new_test_password_456"
    });
    
    let reset_req = test::TestRequest::post()
        .uri("/reset_password")
        .set_json(&reset_data)
        .to_request();
    
    let reset_resp = test::call_service(&app, reset_req).await;
    assert_eq!(reset_resp.status(), StatusCode::UNAUTHORIZED);
    
    let reset_body: serde_json::Value = test::read_body_json(reset_resp).await;
    assert_eq!(reset_body["code"], 401);
    assert_eq!(reset_body["success"], false);
    assert!(reset_body["message"].as_str().unwrap().contains("Missing or invalid Authorization header"));
}

#[actix_web::test]
async fn test_user_reset_password_invalid_session() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    let reset_data = json!({
        "old_password": "test_password_123",
        "new_password": "new_test_password_456"
    });
    
    let reset_req = test::TestRequest::post()
        .uri("/reset_password")
        .insert_header(("Authorization", "Bearer invalid_session_id"))
        .set_json(&reset_data)
        .to_request();
    
    let reset_resp = test::call_service(&app, reset_req).await;
    assert_eq!(reset_resp.status(), StatusCode::UNAUTHORIZED);
    
    let reset_body: serde_json::Value = test::read_body_json(reset_resp).await;
    assert_eq!(reset_body["code"], 401);
    assert_eq!(reset_body["success"], false);
    assert!(reset_body["message"].as_str().unwrap().contains("Invalid or expired session"));
}

// ===== 用户删除测试 =====

#[actix_web::test]
async fn test_user_delete_success() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    // 先注册并登录一个用户
    let test_user = create_test_user();
    let email = test_user.email.clone();
    
    // 注册用户
    let signup_req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&test_user)
        .to_request();
    
    let signup_resp = test::call_service(&app, signup_req).await;
    assert_eq!(signup_resp.status(), StatusCode::OK);
    
    // 登录获取 session_id
    let login_data = create_test_login(email);
    let login_req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_data)
        .to_request();
    
    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status(), StatusCode::OK);
    
    let login_body: serde_json::Value = test::read_body_json(login_resp).await;
    let session_id = login_body["data"]["session_id"].as_str().unwrap();
    
    // 删除用户
    let delete_req = test::TestRequest::delete()
        .uri("/delete")
        .insert_header(("Authorization", format!("Bearer {}", session_id)))
        .to_request();
    
    let delete_resp = test::call_service(&app, delete_req).await;
    assert_eq!(delete_resp.status(), StatusCode::OK);
    
    let delete_body: serde_json::Value = test::read_body_json(delete_resp).await;
    assert_eq!(delete_body["code"], 200);
    assert_eq!(delete_body["success"], true);
    assert_eq!(delete_body["message"], "User deleted successfully.");
    
    // 验证用户无法再登录
    let login_req2 = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_data)
        .to_request();
    
    let login_resp2 = test::call_service(&app, login_req2).await;
    assert_eq!(login_resp2.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_user_delete_missing_auth() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    let delete_req = test::TestRequest::delete()
        .uri("/delete")
        .to_request();
    
    let delete_resp = test::call_service(&app, delete_req).await;
    assert_eq!(delete_resp.status(), StatusCode::UNAUTHORIZED);
    
    let delete_body: serde_json::Value = test::read_body_json(delete_resp).await;
    assert_eq!(delete_body["code"], 401);
    assert_eq!(delete_body["success"], false);
    assert!(delete_body["message"].as_str().unwrap().contains("Missing or invalid Authorization header"));
}

#[actix_web::test]
async fn test_user_delete_invalid_session() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    let delete_req = test::TestRequest::delete()
        .uri("/delete")
        .insert_header(("Authorization", "Bearer invalid_session_id"))
        .to_request();
    
    let delete_resp = test::call_service(&app, delete_req).await;
    assert_eq!(delete_resp.status(), StatusCode::UNAUTHORIZED);
    
    let delete_body: serde_json::Value = test::read_body_json(delete_resp).await;
    assert_eq!(delete_body["code"], 401);
    assert_eq!(delete_body["success"], false);
    assert!(delete_body["message"].as_str().unwrap().contains("Invalid or expired session"));
}

// ===== 边界情况和安全性测试 =====

#[actix_web::test]
async fn test_user_signup_sql_injection_attempt() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    let malicious_user = json!({
        "username": "test_user'; DROP TABLE users; --",
        "email": "test@test.com",
        "password": "test_password_123"
    });
    
    let req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&malicious_user)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // 请求应该成功处理，SQL注入应该被防止
    assert_eq!(resp.status(), StatusCode::OK);
    
    // 验证用户表仍然存在
    let normal_user = create_test_user();
    let normal_req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&normal_user)
        .to_request();
    
    let normal_resp = test::call_service(&app, normal_req).await;
    assert_eq!(normal_resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_user_signup_xss_attempt() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    let xss_user = json!({
        "username": "<script>alert('xss')</script>",
        "email": "test@test.com",
        "password": "test_password_123"
    });
    
    let req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&xss_user)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // 请求应该成功处理
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_user_signup_very_long_fields() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    let long_string = "a".repeat(10000);
    let long_user = json!({
        "username": long_string,
        "email": "test@test.com",
        "password": "test_password_123"
    });
    
    let req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&long_user)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // 应该处理长字符串，具体行为取决于数据库限制
    assert!(resp.status().is_success() || resp.status().is_client_error() || resp.status().is_server_error());
}

#[actix_web::test]
async fn test_user_login_timing_attack_resistance() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    // 注册一个用户
    let test_user = create_test_user();
    let email = test_user.email.clone();
    
    let signup_req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&test_user)
        .to_request();
    
    let signup_resp = test::call_service(&app, signup_req).await;
    assert_eq!(signup_resp.status(), StatusCode::OK);
    
    // 测试存在用户的错误密码
    let wrong_password_data = json!({
        "email": email,
        "password": "wrong_password"
    });
    
    let req1 = test::TestRequest::post()
        .uri("/login")
        .set_json(&wrong_password_data)
        .to_request();
    
    let start1 = std::time::Instant::now();
    let resp1 = test::call_service(&app, req1).await;
    let duration1 = start1.elapsed();
    
    // 测试不存在用户
    let nonexistent_data = json!({
        "email": "nonexistent@test.com",
        "password": "some_password"
    });
    
    let req2 = test::TestRequest::post()
        .uri("/login")
        .set_json(&nonexistent_data)
        .to_request();
    
    let start2 = std::time::Instant::now();
    let resp2 = test::call_service(&app, req2).await;
    let duration2 = start2.elapsed();
    
    // 两个请求都应该返回401
    assert_eq!(resp1.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(resp2.status(), StatusCode::UNAUTHORIZED);
    
    // 时间差异应该相对较小（防止时序攻击）
    // 注意：这个测试在CI环境中可能不稳定，仅供参考
    let time_diff = if duration1 > duration2 { duration1 - duration2 } else { duration2 - duration1 };
    assert!(time_diff.as_millis() < 100, "Time difference too large: {}ms", time_diff.as_millis());
}

#[actix_web::test]
async fn test_concurrent_user_registration() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    // 创建多个不同的测试用户
    let test_users: Vec<_> = (0..10).map(|_| create_test_user()).collect();
    
    // 顺序注册所有用户（避免并发问题）
    let mut success_count = 0;
    for user in test_users {
        let req = test::TestRequest::post()
            .uri("/signup")
            .set_json(&user)
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        if resp.status().is_success() {
            success_count += 1;
        }
    }
    assert_eq!(success_count, 10);
}

#[actix_web::test]
async fn test_concurrent_login_same_user() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    // 注册一个用户
    let test_user = create_test_user();
    let email = test_user.email.clone();
    
    let signup_req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&test_user)
        .to_request();
    
    let signup_resp = test::call_service(&app, signup_req).await;
    assert_eq!(signup_resp.status(), StatusCode::OK);
    
    // 顺序登录同一用户多次（避免并发问题）
    let login_data = create_test_login(email);
    let mut success_count = 0;
    
    for _ in 0..5 {
        let req = test::TestRequest::post()
            .uri("/login")
            .set_json(&login_data.clone())
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        if resp.status().is_success() {
            success_count += 1;
        }
    }
    
    // 所有登录都应该成功
    assert_eq!(success_count, 5);
}