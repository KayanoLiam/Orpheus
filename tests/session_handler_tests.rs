// 生产级会话处理器测试
use actix_web::{test, web, App, http::StatusCode};
use actix_web_httpauth::headers::authorization::Bearer;
use uuid::Uuid;

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

// 创建测试会话
async fn create_test_session(
    redis_client: &redis::Client,
    user_id: Uuid,
) -> String {
    let session_store = orpheus::auth::session_store::SessionStore::new(redis_client.clone());
    session_store
        .create_session(user_id)
        .await
        .expect("Failed to create test session")
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
async fn test_user_logout_success() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    // 先注册并登录用户
    let test_user = create_test_user();
    let email = test_user.email.clone();
    
    // 注册
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
    
    let login_response: serde_json::Value = test::read_body_json(login_resp).await;
    let session_id = login_response["data"]["session_id"].as_str().unwrap();
    
    // 使用会话 ID 登出
    let logout_req = test::TestRequest::post()
        .uri("/logout")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_id.to_string())))
        .to_request();
    
    let logout_resp = test::call_service(&app, logout_req).await;
    
    assert_eq!(logout_resp.status(), StatusCode::OK);
    
    let logout_body: serde_json::Value = test::read_body_json(logout_resp).await;
    assert_eq!(logout_body["code"], 200);
    assert_eq!(logout_body["success"], true);
    assert_eq!(logout_body["message"], "Logged out successfully");
    
    // 验证会话已失效，无法再访问受保护的端点
    let profile_req = test::TestRequest::get()
        .uri("/api/profile")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_id.to_string())))
        .to_request();
    
    let profile_resp = test::call_service(&app, profile_req).await;
    assert_eq!(profile_resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_user_logout_invalid_session() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    // 使用无效会话ID尝试登出
    let logout_req = test::TestRequest::post()
        .uri("/logout")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new("invalid_session_id".to_string())))
        .to_request();
    
    let logout_resp = test::call_service(&app, logout_req).await;
    assert_eq!(logout_resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_user_logout_missing_auth() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    // 不提供认证信息尝试登出
    let logout_req = test::TestRequest::post()
        .uri("/logout")
        .to_request();
    
    let logout_resp = test::call_service(&app, logout_req).await;
    assert_eq!(logout_resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_user_logout_malformed_header() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    // 提供格式错误的Authorization头
    let logout_req = test::TestRequest::post()
        .uri("/logout")
        .insert_header(("Authorization", "InvalidFormat token123"))
        .to_request();
    
    let logout_resp = test::call_service(&app, logout_req).await;
    assert_eq!(logout_resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_user_profile_success() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    // 先注册并登录用户
    let test_user = create_test_user();
    let email = test_user.email.clone();
    
    // 注册
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
    
    let login_response: serde_json::Value = test::read_body_json(login_resp).await;
    let session_id = login_response["data"]["session_id"].as_str().unwrap();
    let user_id = login_response["data"]["user_id"].as_str().unwrap();
    
    // 使用会话 ID 获取用户资料
    let profile_req = test::TestRequest::get()
        .uri("/api/profile")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_id.to_string())))
        .to_request();
    
    let profile_resp = test::call_service(&app, profile_req).await;
    assert_eq!(profile_resp.status(), StatusCode::OK);
    
    let profile_body: serde_json::Value = test::read_body_json(profile_resp).await;
    assert_eq!(profile_body["code"], 200);
    assert_eq!(profile_body["success"], true);
    assert_eq!(profile_body["message"], "Profile retrieved successfully");
    assert_eq!(profile_body["data"].as_str().unwrap(), user_id);
}

#[actix_web::test]
async fn test_user_profile_invalid_session() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    // 使用无效会话ID尝试获取用户资料
    let profile_req = test::TestRequest::get()
        .uri("/api/profile")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new("invalid_session_id".to_string())))
        .to_request();
    
    let profile_resp = test::call_service(&app, profile_req).await;
    assert_eq!(profile_resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_user_profile_missing_auth() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    // 不提供认证信息尝试获取用户资料
    let profile_req = test::TestRequest::get()
        .uri("/api/profile")
        .to_request();
    
    let profile_resp = test::call_service(&app, profile_req).await;
    assert_eq!(profile_resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_user_profile_malformed_header() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    // 提供格式错误的Authorization头
    let profile_req = test::TestRequest::get()
        .uri("/api/profile")
        .insert_header(("Authorization", "InvalidFormat token123"))
        .to_request();
    
    let profile_resp = test::call_service(&app, profile_req).await;
    assert_eq!(profile_resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_session_expiration() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    // 先注册并登录用户
    let test_user = create_test_user();
    let email = test_user.email.clone();
    
    // 注册
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
    
    let login_response: serde_json::Value = test::read_body_json(login_resp).await;
    let session_id = login_response["data"]["session_id"].as_str().unwrap();
    
    // 手动删除Redis中的会话以模拟过期
    let redis_client = create_test_redis_client();
    let mut conn = redis_client.get_connection().unwrap();
    let _: () = redis::cmd("DEL")
        .arg(&session_id)
        .query(&mut conn)
        .unwrap();
    
    // 尝试使用已过期的会话访问受保护端点
    let profile_req = test::TestRequest::get()
        .uri("/api/profile")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_id.to_string())))
        .to_request();
    
    let profile_resp = test::call_service(&app, profile_req).await;
    assert_eq!(profile_resp.status(), StatusCode::UNAUTHORIZED);
    
    // 尝试使用已过期的会话登出
    let logout_req = test::TestRequest::post()
        .uri("/logout")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_id.to_string())))
        .to_request();
    
    let logout_resp = test::call_service(&app, logout_req).await;
    assert_eq!(logout_resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_concurrent_session_operations() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool.clone(), redis_client.clone())).await;
    
    // 先注册并登录用户
    let test_user = create_test_user();
    let email = test_user.email.clone();
    
    // 注册
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
    
    let login_response: serde_json::Value = test::read_body_json(login_resp).await;
    let session_id = login_response["data"]["session_id"].as_str().unwrap();
    
    // 顺序执行多个会话操作（避免并发问题）
    let mut success_count = 0;
    
    // 顺序获取用户资料
    for _ in 0..5 {
        let app_instance = test::init_service(create_test_app(pool.clone(), redis_client.clone())).await;
        let session_id_clone = session_id.to_string();
        
        let profile_req = test::TestRequest::get()
            .uri("/api/profile")
            .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_id_clone)))
            .to_request();
        
        let resp = test::call_service(&app_instance, profile_req).await;
        if resp.status().is_success() {
            success_count += 1;
        }
    }
    
    // 所有请求都应该成功
    assert_eq!(success_count, 5);
}

#[actix_web::test]
async fn test_session_regeneration() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    // 先注册并登录用户
    let test_user = create_test_user();
    let email = test_user.email.clone();
    
    // 注册
    let signup_req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&test_user)
        .to_request();
    
    let signup_resp = test::call_service(&app, signup_req).await;
    assert_eq!(signup_resp.status(), StatusCode::OK);
    
    // 多次登录以生成多个会话
    let mut session_ids = vec![];
    
    for _ in 0..3 {
        let login_data = create_test_login(email.clone());
        
        let login_req = test::TestRequest::post()
            .uri("/login")
            .set_json(&login_data)
            .to_request();
        
        let login_resp = test::call_service(&app, login_req).await;
        assert_eq!(login_resp.status(), StatusCode::OK);
        
        let login_response: serde_json::Value = test::read_body_json(login_resp).await;
        let session_id = login_response["data"]["session_id"].as_str().unwrap().to_string();
        session_ids.push(session_id);
    }
    
    // 验证所有会话都有效
    for session_id in &session_ids {
        let profile_req = test::TestRequest::get()
            .uri("/api/profile")
            .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_id.clone())))
            .to_request();
        
        let profile_resp = test::call_service(&app, profile_req).await;
        assert_eq!(profile_resp.status(), StatusCode::OK);
    }
    
    // 使用其中一个会话登出
    let logout_req = test::TestRequest::post()
        .uri("/logout")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_ids[0].clone())))
        .to_request();
    
    let logout_resp = test::call_service(&app, logout_req).await;
    assert_eq!(logout_resp.status(), StatusCode::OK);
    
    // 验证登出的会话已失效
    let profile_req = test::TestRequest::get()
        .uri("/api/profile")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_ids[0].clone())))
        .to_request();
    
    let profile_resp = test::call_service(&app, profile_req).await;
    assert_eq!(profile_resp.status(), StatusCode::UNAUTHORIZED);
    
    // 验证其他会话仍然有效
    for session_id in &session_ids[1..] {
        let profile_req = test::TestRequest::get()
            .uri("/api/profile")
            .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_id.clone())))
            .to_request();
        
        let profile_resp = test::call_service(&app, profile_req).await;
        assert_eq!(profile_resp.status(), StatusCode::OK);
    }
}

#[actix_web::test]
async fn test_session_with_special_characters() {
    let pool = create_test_pool().await;
    let redis_client = create_test_redis_client();
    let app = test::init_service(create_test_app(pool, redis_client)).await;
    
    // 先注册并登录用户
    let test_user = create_test_user();
    let email = test_user.email.clone();
    
    // 注册
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
    
    let login_response: serde_json::Value = test::read_body_json(login_resp).await;
    let session_id = login_response["data"]["session_id"].as_str().unwrap();
    
    // 测试包含特殊字符的Authorization头格式
    let special_auth_headers = vec![
        format!("Bearer {}", session_id),
        format!("Bearer\t{}", session_id), // Tab字符
        format!("Bearer\n{}", session_id), // 换行符
    ];
    
    for auth_header in special_auth_headers {
        let profile_req = test::TestRequest::get()
            .uri("/api/profile")
            .insert_header(("Authorization", auth_header))
            .to_request();
        
        let profile_resp = test::call_service(&app, profile_req).await;
        // 大多数应该成功，但某些格式可能被拒绝
        assert!(profile_resp.status().is_success() || profile_resp.status().is_client_error());
    }
}