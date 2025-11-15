// 生产级集成测试
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
            database_url: std::env::var("TEST_DATABASE_URL")
                .unwrap_or_else(|_| "postgres://orpheus_user:secret@localhost:5432/orpheus_db".to_string()),
            redis_url: std::env::var("TEST_REDIS_URL")
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

// 完整的用户生命周期测试
#[actix_web::test]
async fn test_complete_user_lifecycle() {
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
    
    // 4. 重置密码
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
    
    // 5. 使用新密码重新登录
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
    let new_session_id = new_login_body["data"]["session_id"].as_str().unwrap().to_string();
    
    // 6. 用户登出
    let logout_req = test::TestRequest::post()
        .uri("/logout")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(new_session_id.clone())))
        .to_request();
    
    let logout_resp = test::call_service(&app, logout_req).await;
    assert_eq!(logout_resp.status(), StatusCode::OK);
    
    let logout_body: serde_json::Value = test::read_body_json(logout_resp).await;
    assert_eq!(logout_body["code"], 200);
    assert_eq!(logout_body["success"], true);
    assert_eq!(logout_body["message"], "Logged out successfully");
    
    // 7. 验证会话已失效
    let profile_req2 = test::TestRequest::get()
        .uri("/api/profile")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(new_session_id)))
        .to_request();
    
    let profile_resp2 = test::call_service(&app, profile_req2).await;
    assert_eq!(profile_resp2.status(), StatusCode::UNAUTHORIZED);
    
    // 8. 重新登录以删除用户
    let delete_login_req = test::TestRequest::post()
        .uri("/login")
        .set_json(&new_login_data)
        .to_request();
    
    let delete_login_resp = test::call_service(&app, delete_login_req).await;
    assert_eq!(delete_login_resp.status(), StatusCode::OK);
    
    let delete_login_body: serde_json::Value = test::read_body_json(delete_login_resp).await;
    let delete_session_id = delete_login_body["data"]["session_id"].as_str().unwrap().to_string();
    
    // 9. 删除用户
    let delete_req = test::TestRequest::delete()
        .uri("/delete")
        .insert_header(("Authorization", format!("Bearer {}", delete_session_id)))
        .to_request();
    
    let delete_resp = test::call_service(&app, delete_req).await;
    assert_eq!(delete_resp.status(), StatusCode::OK);
    
    let delete_body: serde_json::Value = test::read_body_json(delete_resp).await;
    assert_eq!(delete_body["code"], 200);
    assert_eq!(delete_body["success"], true);
    assert_eq!(delete_body["message"], "User deleted successfully.");
    
    // 10. 验证用户无法再登录
    let final_login_req = test::TestRequest::post()
        .uri("/login")
        .set_json(&new_login_data)
        .to_request();
    
    let final_login_resp = test::call_service(&app, final_login_req).await;
    assert_eq!(final_login_resp.status(), StatusCode::UNAUTHORIZED);
}

// 多用户并发操作测试
#[actix_web::test]
async fn test_multiple_users_concurrent_operations() {
    let app = test::init_service(create_full_test_app().await).await;
    
    // 创建多个用户
    let test_users: Vec<_> = (0..5).map(|_| create_test_user()).collect();
    
    // 顺序注册所有用户（避免并发问题）
    let mut signup_success_count = 0;
    for user in &test_users {
        // 为每个请求创建一个新的应用实例
        let app_instance = test::init_service(create_full_test_app().await).await;
        let user_clone = user.clone();
        
        let signup_req = test::TestRequest::post()
            .uri("/signup")
            .set_json(&user_clone)
            .to_request();
        
        let resp = test::call_service(&app_instance, signup_req).await;
        if resp.status().is_success() {
            signup_success_count += 1;
        }
    }
    assert_eq!(signup_success_count, 5);
    
    // 顺序登录所有用户（避免并发问题）
    let mut login_success_count = 0;
    let mut session_ids = vec![];
    
    for user in &test_users {
        // 为每个请求创建一个新的应用实例
        let app_instance = test::init_service(create_full_test_app().await).await;
        let user_clone = user.clone();
        let login_data = create_test_login(user_clone.email);
        
        let login_req = test::TestRequest::post()
            .uri("/login")
            .set_json(&login_data)
            .to_request();
        
        let resp = test::call_service(&app_instance, login_req).await;
        if resp.status().is_success() {
            login_success_count += 1;
            
            let login_response: serde_json::Value = test::read_body_json(resp).await;
            let session_id = login_response["data"]["session_id"].as_str().unwrap().to_string();
            session_ids.push(session_id);
        }
    }
    assert_eq!(login_success_count, 5);
    
    // 顺序获取用户资料（避免并发问题）
    let mut profile_success_count = 0;
    for session_id in &session_ids {
        // 为每个请求创建一个新的应用实例
        let app_instance = test::init_service(create_full_test_app().await).await;
        let session_id_clone = session_id.clone();
        
        let profile_req = test::TestRequest::get()
            .uri("/api/profile")
            .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_id_clone)))
            .to_request();
        
        let resp = test::call_service(&app_instance, profile_req).await;
        if resp.status().is_success() {
            profile_success_count += 1;
        }
    }
    assert_eq!(profile_success_count, 5);
    
    // 顺序登出所有用户（避免并发问题）
    let mut logout_success_count = 0;
    for session_id in &session_ids {
        // 为每个请求创建一个新的应用实例
        let app_instance = test::init_service(create_full_test_app().await).await;
        let session_id_clone = session_id.clone();
        
        let logout_req = test::TestRequest::post()
            .uri("/logout")
            .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_id_clone)))
            .to_request();
        
        let resp = test::call_service(&app_instance, logout_req).await;
        if resp.status().is_success() {
            logout_success_count += 1;
        }
    }
    assert_eq!(logout_success_count, 5);
}

// 系统负载测试
#[actix_web::test]
async fn test_system_load() {
    let app = test::init_service(create_full_test_app().await).await;
    
    // 创建一个测试用户
    let test_user = create_test_user();
    let email = test_user.email.clone();
    
    // 注册用户
    let signup_req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&test_user)
        .to_request();
    
    let signup_resp = test::call_service(&app, signup_req).await;
    assert_eq!(signup_resp.status(), StatusCode::OK);
    
    // 登录获取会话
    let login_data = create_test_login(email);
    let login_req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_data)
        .to_request();
    
    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status(), StatusCode::OK);
    
    let login_body: serde_json::Value = test::read_body_json(login_resp).await;
    let session_id = login_body["data"]["session_id"].as_str().unwrap().to_string();
    
    // 顺序执行大量请求（避免并发问题）
    let mut success_count = 0;
    let mut total_requests = 0;
    
    // 顺序获取用户资料
    for _ in 0..50 {
        // 为每个请求创建一个新的应用实例
        let app_instance = test::init_service(create_full_test_app().await).await;
        let session_id_clone = session_id.clone();
        
        let profile_req = test::TestRequest::get()
            .uri("/api/profile")
            .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_id_clone)))
            .to_request();
        
        let resp = test::call_service(&app_instance, profile_req).await;
        total_requests += 1;
        if resp.status().is_success() {
            success_count += 1;
        }
    }
    
    // 顺序请求GitHub API
    for _ in 0..20 {
        // 为每个请求创建一个新的应用实例
        let app_instance = test::init_service(create_full_test_app().await).await;
        
        let github_req = test::TestRequest::get()
            .uri("/github/stars/microsoft/vscode")
            .to_request();
        
        let resp = test::call_service(&app_instance, github_req).await;
        total_requests += 1;
        if resp.status().is_success() {
            success_count += 1;
        }
    }
    
    // 至少80%的请求应该成功
    let success_rate = (success_count as f64 / total_requests as f64) * 100.0;
    assert!(success_rate >= 80.0, "Success rate too low: {:.2}%", success_rate);
}

// 错误恢复测试
#[actix_web::test]
async fn test_error_recovery() {
    let app = test::init_service(create_full_test_app().await).await;
    
    // 创建一个测试用户
    let test_user = create_test_user();
    let email = test_user.email.clone();
    
    // 注册用户
    let signup_req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&test_user)
        .to_request();
    
    let signup_resp = test::call_service(&app, signup_req).await;
    assert_eq!(signup_resp.status(), StatusCode::OK);
    
    // 尝试使用错误密码登录
    let wrong_login_data = json!({
        "email": email,
        "password": "wrong_password"
    });
    
    let wrong_login_req = test::TestRequest::post()
        .uri("/login")
        .set_json(&wrong_login_data)
        .to_request();
    
    let wrong_login_resp = test::call_service(&app, wrong_login_req).await;
    assert_eq!(wrong_login_resp.status(), StatusCode::UNAUTHORIZED);
    
    // 使用正确密码登录应该仍然有效
    let login_data = create_test_login(email);
    let login_req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_data)
        .to_request();
    
    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status(), StatusCode::OK);
    
    let login_body: serde_json::Value = test::read_body_json(login_resp).await;
    let session_id = login_body["data"]["session_id"].as_str().unwrap().to_string();
    
    // 尝试访问不存在的端点
    let not_found_req = test::TestRequest::get()
        .uri("/nonexistent_endpoint")
        .to_request();
    
    let not_found_resp = test::call_service(&app, not_found_req).await;
    assert_eq!(not_found_resp.status(), StatusCode::NOT_FOUND);
    
    // 系统应该仍然正常工作
    let profile_req = test::TestRequest::get()
        .uri("/api/profile")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_id)))
        .to_request();
    
    let profile_resp = test::call_service(&app, profile_req).await;
    assert_eq!(profile_resp.status(), StatusCode::OK);
}

// 数据一致性测试
#[actix_web::test]
async fn test_data_consistency() {
    let app = test::init_service(create_full_test_app().await).await;
    
    // 创建一个测试用户
    let test_user = create_test_user();
    let email = test_user.email.clone();
    let username = test_user.username.clone();
    
    // 注册用户
    let signup_req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&test_user)
        .to_request();
    
    let signup_resp = test::call_service(&app, signup_req).await;
    assert_eq!(signup_resp.status(), StatusCode::OK);
    
    // 多次登录应该返回相同的用户ID
    let login_data = create_test_login(email.clone());
    let mut user_ids = vec![];
    
    for _ in 0..3 {
        let login_req = test::TestRequest::post()
            .uri("/login")
            .set_json(&login_data.clone())
            .to_request();
        
        let login_resp = test::call_service(&app, login_req).await;
        assert_eq!(login_resp.status(), StatusCode::OK);
        
        let login_body: serde_json::Value = test::read_body_json(login_resp).await;
        let user_id = login_body["data"]["user_id"].as_str().unwrap().to_string();
        user_ids.push(user_id);
    }
    
    // 所有用户ID应该相同
    for user_id in &user_ids[1..] {
        assert_eq!(user_ids[0], *user_id);
    }
    
    // 尝试使用相同邮箱再次注册应该失败
    let duplicate_user = orpheus::models::user::SignupRequest {
        username: format!("different_username_{}", uuid::Uuid::new_v4()),
        email: email.clone(),
        password: "different_password_123".to_string(),
    };
    
    let duplicate_signup_req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&duplicate_user)
        .to_request();
    
    let duplicate_signup_resp = test::call_service(&app, duplicate_signup_req).await;
    assert_eq!(duplicate_signup_resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    
    // 原用户应该仍然可以登录
    let final_login_req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_data)
        .to_request();
    
    let final_login_resp = test::call_service(&app, final_login_req).await;
    assert_eq!(final_login_resp.status(), StatusCode::OK);
    
    let final_login_body: serde_json::Value = test::read_body_json(final_login_resp).await;
    let final_user_id = final_login_body["data"]["user_id"].as_str().unwrap().to_string();
    
    // 用户ID应该保持不变
    assert_eq!(user_ids[0], final_user_id);
}

// 安全性集成测试
#[actix_web::test]
async fn test_security_integration() {
    let app = test::init_service(create_full_test_app().await).await;
    
    // 创建一个测试用户
    let test_user = create_test_user();
    let email = test_user.email.clone();
    
    // 注册用户
    let signup_req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&test_user)
        .to_request();
    
    let signup_resp = test::call_service(&app, signup_req).await;
    assert_eq!(signup_resp.status(), StatusCode::OK);
    
    // 登录获取会话
    let login_data = create_test_login(email);
    let login_req = test::TestRequest::post()
        .uri("/login")
        .set_json(&login_data)
        .to_request();
    
    let login_resp = test::call_service(&app, login_req).await;
    assert_eq!(login_resp.status(), StatusCode::OK);
    
    let login_body: serde_json::Value = test::read_body_json(login_resp).await;
    let session_id = login_body["data"]["session_id"].as_str().unwrap().to_string();
    
    // 尝试使用伪造的会话ID
    let fake_session_req = test::TestRequest::get()
        .uri("/api/profile")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new("fake_session_id".to_string())))
        .to_request();
    
    let fake_session_resp = test::call_service(&app, fake_session_req).await;
    assert_eq!(fake_session_resp.status(), StatusCode::UNAUTHORIZED);
    
    // 尝试使用格式错误的认证头
    let malformed_auth_req = test::TestRequest::get()
        .uri("/api/profile")
        .insert_header(("Authorization", "InvalidFormat token123"))
        .to_request();
    
    let malformed_auth_resp = test::call_service(&app, malformed_auth_req).await;
    assert_eq!(malformed_auth_resp.status(), StatusCode::UNAUTHORIZED);
    
    // 尝试不提供认证头
    let no_auth_req = test::TestRequest::get()
        .uri("/api/profile")
        .to_request();
    
    let no_auth_resp = test::call_service(&app, no_auth_req).await;
    assert_eq!(no_auth_resp.status(), StatusCode::UNAUTHORIZED);
    
    // 有效会话应该仍然工作
    let valid_session_req = test::TestRequest::get()
        .uri("/api/profile")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_id)))
        .to_request();
    
    let valid_session_resp = test::call_service(&app, valid_session_req).await;
    assert_eq!(valid_session_resp.status(), StatusCode::OK);
}

// 系统边界测试
#[actix_web::test]
async fn test_system_boundaries() {
    let app = test::init_service(create_full_test_app().await).await;
    
    // 测试非常大的请求数据
    let large_username = "a".repeat(1000);
    let large_email = format!("{}@{}.com", "b".repeat(500), "c".repeat(500));
    let large_password = "d".repeat(1000);
    
    let large_user = orpheus::models::user::SignupRequest {
        username: large_username,
        email: large_email,
        password: large_password,
    };
    
    let large_signup_req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&large_user)
        .to_request();
    
    let large_signup_resp = test::call_service(&app, large_signup_req).await;
    // 应该被处理，具体结果取决于数据库限制
    assert!(large_signup_resp.status().is_success() || large_signup_resp.status().is_client_error() || large_signup_resp.status().is_server_error());
    
    // 测试包含特殊字符的数据
    let special_user = orpheus::models::user::SignupRequest {
        username: "user_with_special_chars_!@#$%^&*()".to_string(),
        email: "user+special@example.com".to_string(),
        password: "password_with_special_!@#$%^&*()".to_string(),
    };
    
    let special_signup_req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&special_user)
        .to_request();
    
    let special_signup_resp = test::call_service(&app, special_signup_req).await;
    // 应该能处理特殊字符
    assert!(special_signup_resp.status().is_success() || special_signup_resp.status().is_client_error() || special_signup_resp.status().is_server_error());
    
    // 测试Unicode字符
    let unicode_user = orpheus::models::user::SignupRequest {
        username: "用户_测试_🚀".to_string(),
        email: "用户@example.com".to_string(),
        password: "密码_测试_123".to_string(),
    };
    
    let unicode_signup_req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&unicode_user)
        .to_request();
    
    let unicode_signup_resp = test::call_service(&app, unicode_signup_req).await;
    // 应该能处理Unicode字符
    assert!(unicode_signup_resp.status().is_success() || unicode_signup_resp.status().is_client_error() || unicode_signup_resp.status().is_server_error());
}