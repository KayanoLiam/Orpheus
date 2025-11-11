use actix_web::{test, web, App, http::StatusCode, HttpMessage};
use serde_json::json;
use uuid::Uuid;
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
                .unwrap_or_else(|_| "postgres://kayano:121381@localhost:5432/test_postgres".to_string()),
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
async fn create_test_app() -> impl actix_web::dev::ServiceFactory<
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

#[actix_web::test]
async fn test_user_signup_success() {
    let app = test::init_service(create_test_app().await).await;
    
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
    let app = test::init_service(create_test_app().await).await;
    
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
    let app = test::init_service(create_test_app().await);
    
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
async fn test_user_signup_empty_fields() {
    let app = test::init_service(create_test_app().await);
    
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
    let app = test::init_service(create_test_app().await).await;
    
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
    let app = test::init_service(create_test_app().await);
    
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
    let app = test::init_service(create_test_app().await).await;
    
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
    let app = test::init_service(create_test_app().await);
    
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
    let app = test::init_service(create_test_app().await);
    
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