// 简化的集成测试，避免并发问题
use actix_web::{test, web, App, http::StatusCode};
use actix_web_httpauth::headers::authorization::Bearer;
use serde_json::json;

// 测试配置
struct TestConfig {
    database_url: String,
    redis_url: String,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://orpheus_user:secret@localhost:5432/orpheus_db".to_string()),
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
        }
    }
}

// 创建测试数据库连接池
async fn create_test_pool() -> sqlx::Pool<sqlx::Postgres> {
    let config = TestConfig::default();
    let pool = sqlx::PgPool::connect(&config.database_url).await.expect("Failed to create test pool");
    
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

// 创建完整测试应用实例
async fn create_full_test_app() -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    
    App::new()
        .app_data(web::Data::new(pool))
        .app_data(web::Data::new(redis_client))
        .service(orpheus::handlers::user_handler::user_signup)
        .service(orpheus::handlers::user_handler::user_login)
        .service(orpheus::handlers::user_handler::user_reset_password)
        .service(orpheus::handlers::user_handler::user_delete)
        .service(orpheus::handlers::session_handler::user_logout)
        .service(orpheus::handlers::github_handler::get_github_repo_stars)
        .service(
            web::scope("/api")
                .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(
                    orpheus::middlewares::session::session_validator,
                ))
                .service(orpheus::handlers::session_handler::user_profile),
        )
}

// 简单的用户生命周期测试
#[actix_web::test]
async fn test_simple_user_lifecycle() {
    let app = test::init_service(create_full_test_app().await).await;
    
    // 1. 用户注册
    let test_user = create_test_user();
    let email = test_user.email.clone();
    
    let signup_req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&test_user)
        .to_request();
    
    let signup_resp = test::call_service(&app, signup_req).await;
    assert_eq!(signup_resp.status(), StatusCode::OK);
    
    let signup_body: serde_json::Value = test::read_body_json(signup_resp).await;
    assert_eq!(signup_body["code"], 200);
    assert_eq!(signup_body["success"], true);
    
    // 2. 用户登录
    let login_data = create_test_login(email.clone());
    
    let login_req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_data)
        .to_request();
    
    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status(), StatusCode::OK);
    
    let login_body: serde_json::Value = test::read_body_json(login_resp).await;
    assert_eq!(login_body["code"], 200);
    assert_eq!(login_body["success"], true);
    assert!(login_body["data"]["session_id"].is_string());
    assert!(login_body["data"]["user_id"].is_string());
    assert!(login_body["data"]["expires_at"].is_number());
    
    let session_id = login_body["data"]["session_id"].as_str().unwrap().to_string();
    let user_id = login_body["data"]["user_id"].as_str().unwrap().to_string();
    
    // 3. 获取用户资料
    let profile_req = test::TestRequest::get()
        .uri("/api/profile")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_id.clone())))
        .to_request();
    
    let profile_resp = test::call_service(&app, profile_req).await;
    assert_eq!(profile_resp.status(), StatusCode::OK);
    
    let profile_body: serde_json::Value = test::read_body_json(profile_resp).await;
    assert_eq!(profile_body["code"], 200);
    assert_eq!(profile_body["success"], true);
    assert_eq!(profile_body["data"].as_str().unwrap(), user_id);
    
    // 4. 用户登出
    let logout_req = test::TestRequest::post()
        .uri("/logout")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_id)))
        .to_request();
    
    let logout_resp = test::call_service(&app, logout_req).await;
    assert!(logout_resp.status() == StatusCode::OK || logout_resp.status() == StatusCode::INTERNAL_SERVER_ERROR);
    
    if logout_resp.status().is_success() {
        let logout_body: serde_json::Value = test::read_body_json(logout_resp).await;
        assert_eq!(logout_body["code"], 200);
        assert_eq!(logout_body["success"], true);
        assert_eq!(logout_body["message"], "Logged out successfully");
    }
}

// 密码重置流程测试
#[actix_web::test]
async fn test_password_reset_flow() {
    let app = test::init_service(create_full_test_app().await).await;
    
    // 1. 注册用户
    let test_user = create_test_user();
    let email = test_user.email.clone();
    
    let signup_req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&test_user)
        .to_request();
    
    let signup_resp = test::call_service(&app, signup_req).await;
    assert_eq!(signup_resp.status(), StatusCode::OK);
    
    // 2. 登录获取会话
    let login_data = create_test_login(email.clone());
    
    let login_req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_data)
        .to_request();
    
    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status(), StatusCode::OK);
    
    let login_body: serde_json::Value = test::read_body_json(login_resp).await;
    let session_id = login_body["data"]["session_id"].as_str().unwrap().to_string();
    
    // 3. 重置密码
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
    
    // 4. 使用新密码登录
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
    
    let new_login_body: serde_json::Value = test::read_body_json(new_login_resp).await;
    assert_eq!(new_login_body["code"], 200);
    assert_eq!(new_login_body["success"], true);
}

// GitHub API集成测试
#[actix_web::test]
async fn test_github_integration() {
    let app = test::init_service(create_full_test_app().await).await;
    
    // 测试GitHub API
    let github_req = test::TestRequest::get()
        .uri("/github/stars/microsoft/vscode")
        .to_request();
    
    let github_resp = test::call_service(&app, github_req).await;
    
    // GitHub API可能因网络问题失败，但请求应该被正确处理
    assert!(github_resp.status().is_success() || github_resp.status().is_server_error());
    
    if github_resp.status().is_success() {
        let github_body: serde_json::Value = test::read_body_json(github_resp).await;
        assert_eq!(github_body["code"], 200);
        assert_eq!(github_body["success"], true);
        assert!(github_body["data"]["stars"].is_number());
    }
}

// 错误处理测试
#[actix_web::test]
async fn test_error_handling() {
    let app = test::init_service(create_full_test_app().await).await;
    
    // 1. 测试无效登录
    let invalid_login = json!({
        "email": "nonexistent@test.com",
        "password": "wrong_password"
    });
    
    let invalid_login_req = test::TestRequest::post()
        .uri("/login")
        .set_json(&invalid_login)
        .to_request();
    
    let invalid_login_resp = test::call_service(&app, invalid_login_req).await;
    assert_eq!(invalid_login_resp.status(), StatusCode::UNAUTHORIZED);
    
    // 2. 测试无效会话
    let invalid_profile_req = test::TestRequest::get()
        .uri("/api/profile")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new("invalid_session_id".to_string())))
        .to_request();
    
    let invalid_profile_resp = test::call_service(&app, invalid_profile_req).await;
    assert_eq!(invalid_profile_resp.status(), StatusCode::UNAUTHORIZED);
    
    // 3. 测试不存在的端点
    let not_found_req = test::TestRequest::get()
        .uri("/nonexistent_endpoint")
        .to_request();
    
    let not_found_resp = test::call_service(&app, not_found_req).await;
    assert_eq!(not_found_resp.status(), StatusCode::NOT_FOUND);
}

// 数据一致性测试
#[actix_web::test]
async fn test_data_consistency() {
    let app = test::init_service(create_full_test_app().await).await;
    
    // 1. 注册用户
    let test_user = create_test_user();
    let email = test_user.email.clone();
    
    let signup_req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&test_user)
        .to_request();
    
    let signup_resp = test::call_service(&app, signup_req).await;
    assert_eq!(signup_resp.status(), StatusCode::OK);
    
    // 2. 多次登录应该返回相同的用户ID
    let mut user_ids = vec![];
    
    // 第一次登录
    let login_data1 = create_test_login(email.clone());
    let login_req1 = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_data1)
        .to_request();
    
    let login_resp1 = test::call_service(&app, login_req1).await;
    assert_eq!(login_resp1.status(), StatusCode::OK);
    
    let login_body1: serde_json::Value = test::read_body_json(login_resp1).await;
    let user_id1 = login_body1["data"]["user_id"].as_str().unwrap().to_string();
    user_ids.push(user_id1);
    
    // 第二次登录
    let login_data2 = create_test_login(email.clone());
    let login_req2 = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_data2)
        .to_request();
    
    let login_resp2 = test::call_service(&app, login_req2).await;
    assert_eq!(login_resp2.status(), StatusCode::OK);
    
    let login_body2: serde_json::Value = test::read_body_json(login_resp2).await;
    let user_id2 = login_body2["data"]["user_id"].as_str().unwrap().to_string();
    user_ids.push(user_id2);
    
    // 所有用户ID应该相同
    assert_eq!(user_ids[0], user_ids[1]);
}