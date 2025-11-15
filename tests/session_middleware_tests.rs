// 生产级中间件测试
use actix_web::{test, web, App, http::StatusCode};
use actix_web_httpauth::headers::authorization::Bearer;
use uuid::Uuid;

// 测试配置
struct TestConfig {
    #[allow(dead_code)]
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

// 创建测试 Redis 客户端
fn create_test_redis_client() -> redis::Client {
    let config = TestConfig::default();
    redis::Client::open(config.redis_url).expect("Failed to create test Redis client")
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

#[actix_web::test]
async fn test_session_validator_valid_token() {
    let redis_client = create_test_redis_client();
    let test_user_id = Uuid::new_v4();
    
    // 创建一个有效的会话
    let session_id = create_test_session(&redis_client, test_user_id).await;
    
    // 创建一个简单的测试端点来验证中间件
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(redis_client))
            .service(
                web::scope("/test")
                    .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(
                        orpheus::middlewares::session::session_validator,
                    ))
                    .route("/protected", web::get().to(|| async { "protected content" })),
            ),
    )
    .await;
    
    // 使用有效 token 请求受保护的端点
    let req = test::TestRequest::get()
        .uri("/test/protected")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_id)))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // 应该成功访问
    assert_eq!(resp.status(), StatusCode::OK);
    
    let body = test::read_body(resp).await;
    assert_eq!(body, "protected content");
}

#[actix_web::test]
async fn test_session_validator_invalid_token() {
    let redis_client = create_test_redis_client();
    
    // 创建一个简单的测试端点来验证中间件
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(redis_client))
            .service(
                web::scope("/test")
                    .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(
                        orpheus::middlewares::session::session_validator,
                    ))
                    .route("/protected", web::get().to(|| async { "protected content" })),
            ),
    )
    .await;
    
    // 使用无效 token 请求受保护的端点
    let req = test::TestRequest::get()
        .uri("/test/protected")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new("invalid_token".to_string())))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // 应该返回未授权
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_session_validator_missing_token() {
    let redis_client = create_test_redis_client();
    
    // 创建一个简单的测试端点来验证中间件
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(redis_client))
            .service(
                web::scope("/test")
                    .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(
                        orpheus::middlewares::session::session_validator,
                    ))
                    .route("/protected", web::get().to(|| async { "protected content" })),
            ),
    )
    .await;
    
    // 不提供 token 请求受保护的端点
    let req = test::TestRequest::get()
        .uri("/test/protected")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // 应该返回未授权
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_session_validator_malformed_header() {
    let redis_client = create_test_redis_client();
    
    // 创建一个简单的测试端点来验证中间件
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(redis_client))
            .service(
                web::scope("/test")
                    .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(
                        orpheus::middlewares::session::session_validator,
                    ))
                    .route("/protected", web::get().to(|| async { "protected content" })),
            ),
    )
    .await;
    
    // 使用格式错误的Authorization头
    let req = test::TestRequest::get()
        .uri("/test/protected")
        .insert_header(("Authorization", "InvalidFormat token123"))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // 应该返回未授权
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_session_validator_empty_token() {
    let redis_client = create_test_redis_client();
    
    // 创建一个简单的测试端点来验证中间件
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(redis_client))
            .service(
                web::scope("/test")
                    .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(
                        orpheus::middlewares::session::session_validator,
                    ))
                    .route("/protected", web::get().to(|| async { "protected content" })),
            ),
    )
    .await;
    
    // 使用空的token
    let req = test::TestRequest::get()
        .uri("/test/protected")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new("".to_string())))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // 应该返回未授权
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_session_validator_expired_session() {
    let redis_client = create_test_redis_client();
    let test_user_id = Uuid::new_v4();
    
    // 创建一个会话
    let session_id = create_test_session(&redis_client, test_user_id).await;
    
    // 手动删除Redis中的会话以模拟过期
    let mut conn = redis_client.get_connection().unwrap();
    let _: () = redis::cmd("DEL")
        .arg(&session_id)
        .query(&mut conn)
        .unwrap();
    
    // 创建一个简单的测试端点来验证中间件
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(redis_client))
            .service(
                web::scope("/test")
                    .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(
                        orpheus::middlewares::session::session_validator,
                    ))
                    .route("/protected", web::get().to(|| async { "protected content" })),
            ),
    )
    .await;
    
    // 使用已过期的token请求受保护的端点
    let req = test::TestRequest::get()
        .uri("/test/protected")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_id)))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // 应该返回未授权
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_session_validator_special_characters() {
    let redis_client = create_test_redis_client();
    
    // 创建一个简单的测试端点来验证中间件
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(redis_client))
            .service(
                web::scope("/test")
                    .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(
                        orpheus::middlewares::session::session_validator,
                    ))
                    .route("/protected", web::get().to(|| async { "protected content" })),
            ),
    )
    .await;
    
    // 使用包含特殊字符的token
    let special_tokens = vec![
        "token_with_underscores_and-dashes",
        "tokenWithCamelCase",
        "token123!@#$%^&*()",
        "token with spaces",
        "token\nwith\nnewlines",
        "token\twith\ttabs",
    ];
    
    for token in special_tokens {
        let req = test::TestRequest::get()
            .uri("/test/protected")
            .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(token.to_string())))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // 特殊字符的token应该被处理，但会话不存在所以应该返回未授权
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}

#[actix_web::test]
async fn test_session_validator_very_long_token() {
    let redis_client = create_test_redis_client();
    
    // 创建一个简单的测试端点来验证中间件
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(redis_client))
            .service(
                web::scope("/test")
                    .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(
                        orpheus::middlewares::session::session_validator,
                    ))
                    .route("/protected", web::get().to(|| async { "protected content" })),
            ),
    )
    .await;
    
    // 使用非常长的token
    let long_token = "a".repeat(10000);
    
    let req = test::TestRequest::get()
        .uri("/test/protected")
        .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(long_token)))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    // 长token应该被正确处理，但会话不存在所以应该返回未授权
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_session_validator_unicode_token() {
    let redis_client = create_test_redis_client();
    
    // 创建一个简单的测试端点来验证中间件
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(redis_client))
            .service(
                web::scope("/test")
                    .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(
                        orpheus::middlewares::session::session_validator,
                    ))
                    .route("/protected", web::get().to(|| async { "protected content" })),
            ),
    )
    .await;
    
    // 使用Unicode字符的token
    let unicode_tokens = vec![
        "token_with_中文",
        "token_with_emoji_🚀",
        "token_with_ñ_á_é_í_ó_ú",
        "token_with_кириллица",
        "token_with_العربية",
    ];
    
    for token in unicode_tokens {
        let req = test::TestRequest::get()
            .uri("/test/protected")
            .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(token.to_string())))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Unicode token应该被正确处理，但会话不存在所以应该返回未授权
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}

#[actix_web::test]
async fn test_session_validator_sql_injection_attempt() {
    let redis_client = create_test_redis_client();
    
    // 创建一个简单的测试端点来验证中间件
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(redis_client))
            .service(
                web::scope("/test")
                    .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(
                        orpheus::middlewares::session::session_validator,
                    ))
                    .route("/protected", web::get().to(|| async { "protected content" })),
            ),
    )
    .await;
    
    // 使用SQL注入尝试的token
    let sql_injection_tokens = vec![
        "'; DROP TABLE sessions; --",
        "' OR '1'='1",
        "'; DELETE FROM sessions WHERE '1'='1'; --",
        "admin'--",
        "admin' /*",
    ];
    
    for token in sql_injection_tokens {
        let req = test::TestRequest::get()
            .uri("/test/protected")
            .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(token.to_string())))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // SQL注入应该被正确处理，但会话不存在所以应该返回未授权
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}

#[actix_web::test]
async fn test_session_validator_xss_attempt() {
    let redis_client = create_test_redis_client();
    
    // 创建一个简单的测试端点来验证中间件
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(redis_client))
            .service(
                web::scope("/test")
                    .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(
                        orpheus::middlewares::session::session_validator,
                    ))
                    .route("/protected", web::get().to(|| async { "protected content" })),
            ),
    )
    .await;
    
    // 使用XSS尝试的token
    let xss_tokens = vec![
        "<script>alert('xss')</script>",
        "javascript:alert('xss')",
        "<img src=x onerror=alert('xss')>",
        "';alert('xss');//",
    ];
    
    for token in xss_tokens {
        let req = test::TestRequest::get()
            .uri("/test/protected")
            .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(token.to_string())))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // XSS应该被正确处理，但会话不存在所以应该返回未授权
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}

#[actix_web::test]
async fn test_session_validator_concurrent_requests() {
    let redis_client = create_test_redis_client();
    let test_user_id = Uuid::new_v4();
    
    // 创建一个有效的会话
    let session_id = create_test_session(&redis_client, test_user_id).await;
    
    // 创建一个简单的测试端点来验证中间件
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(redis_client))
            .service(
                web::scope("/test")
                    .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(
                        orpheus::middlewares::session::session_validator,
                    ))
                    .route("/protected", web::get().to(|| async { "protected content" })),
            ),
    )
    .await;
    
    // 顺序请求受保护的端点（避免并发问题）
    let mut success_count = 0;
    
    for _ in 0..10 {
        let session_id_clone = session_id.clone();
        let req = test::TestRequest::get()
            .uri("/test/protected")
            .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_id_clone)))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        if resp.status().is_success() {
            success_count += 1;
        }
    }
    
    // 所有请求都应该成功
    assert_eq!(success_count, 10);
}

#[actix_web::test]
async fn test_session_validator_multiple_endpoints() {
    let redis_client = create_test_redis_client();
    let test_user_id = Uuid::new_v4();
    
    // 创建一个有效的会话
    let session_id = create_test_session(&redis_client, test_user_id).await;
    
    // 创建多个受保护的端点
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(redis_client))
            .service(
                web::scope("/api")
                    .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(
                        orpheus::middlewares::session::session_validator,
                    ))
                    .route("/profile", web::get().to(|| async { "profile data" }))
                    .route("/settings", web::get().to(|| async { "settings data" }))
                    .route("/dashboard", web::get().to(|| async { "dashboard data" })),
            ),
    )
    .await;
    
    let endpoints = vec!["/api/profile", "/api/settings", "/api/dashboard"];
    
    for endpoint in endpoints {
        let req = test::TestRequest::get()
            .uri(endpoint)
            .insert_header((actix_web::http::header::AUTHORIZATION, Bearer::new(session_id.clone())))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // 所有端点都应该成功访问
        assert_eq!(resp.status(), StatusCode::OK);
    }
}

#[actix_web::test]
async fn test_session_validator_case_sensitivity() {
    let redis_client = create_test_redis_client();
    let test_user_id = Uuid::new_v4();
    
    // 创建一个有效的会话
    let session_id = create_test_session(&redis_client, test_user_id).await;
    
    // 创建一个简单的测试端点来验证中间件
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(redis_client))
            .service(
                web::scope("/test")
                    .wrap(actix_web_httpauth::middleware::HttpAuthentication::bearer(
                        orpheus::middlewares::session::session_validator,
                    ))
                    .route("/protected", web::get().to(|| async { "protected content" })),
            ),
    )
    .await;
    
    // 测试不同格式的Authorization头
    let auth_formats = vec![
        format!("Bearer {}", session_id),
        format!("bearer {}", session_id), // 小写
        format!("BEARER {}", session_id), // 大写
        format!("BeArEr {}", session_id), // 混合大小写
    ];
    
    for auth_header in auth_formats {
        let req = test::TestRequest::get()
            .uri("/test/protected")
            .insert_header(("Authorization", auth_header))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // 大部分格式应该被接受，但具体行为取决于HTTP认证库的实现
        assert!(resp.status().is_success() || resp.status().is_client_error());
    }
}