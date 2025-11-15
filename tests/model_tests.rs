// 生产级模型测试
use serde_json::{self, json};
use uuid::Uuid;
use chrono::Utc;

// 测试用户模型
#[actix_web::test]
async fn test_signup_request_model() {
    let signup_request = orpheus::models::user::SignupRequest {
        username: "test_user".to_string(),
        email: "test@example.com".to_string(),
        password: "secure_password_123".to_string(),
    };
    
    // 序列化为JSON
    let json_str = serde_json::to_string(&signup_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    
    assert_eq!(parsed["username"], "test_user");
    assert_eq!(parsed["email"], "test@example.com");
    assert_eq!(parsed["password"], "secure_password_123");
    
    // 从JSON反序列化
    let deserialized: orpheus::models::user::SignupRequest = serde_json::from_value(parsed).unwrap();
    assert_eq!(deserialized.username, "test_user");
    assert_eq!(deserialized.email, "test@example.com");
    assert_eq!(deserialized.password, "secure_password_123");
}

#[actix_web::test]
async fn test_login_request_model() {
    let login_request = orpheus::models::user::LoginRequest {
        email: "test@example.com".to_string(),
        password: "secure_password_123".to_string(),
    };
    
    // 序列化为JSON
    let json_str = serde_json::to_string(&login_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    
    assert_eq!(parsed["email"], "test@example.com");
    assert_eq!(parsed["password"], "secure_password_123");
    
    // 从JSON反序列化
    let deserialized: orpheus::models::user::LoginRequest = serde_json::from_value(parsed).unwrap();
    assert_eq!(deserialized.email, "test@example.com");
    assert_eq!(deserialized.password, "secure_password_123");
}

#[actix_web::test]
async fn test_change_password_request_model() {
    let change_password_request = orpheus::models::user::ChangePasswordRequest {
        old_password: "old_password_123".to_string(),
        new_password: "new_password_456".to_string(),
    };
    
    // 序列化为JSON
    let json_str = serde_json::to_string(&change_password_request).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    
    assert_eq!(parsed["old_password"], "old_password_123");
    assert_eq!(parsed["new_password"], "new_password_456");
    
    // 从JSON反序列化
    let deserialized: orpheus::models::user::ChangePasswordRequest = serde_json::from_value(parsed).unwrap();
    assert_eq!(deserialized.old_password, "old_password_123");
    assert_eq!(deserialized.new_password, "new_password_456");
}

// 测试会话模型
#[actix_web::test]
async fn test_session_response_model() {
    let user_id = Uuid::new_v4();
    let session_id = "user_12345678-1234-1234-1234-123456789abc".to_string();
    let expires_at = Utc::now().timestamp() + 3600;
    
    let session_response = orpheus::models::session::SessionResponse {
        session_id: session_id.clone(),
        user_id,
        expires_at,
    };
    
    // 序列化为JSON
    let json_str = serde_json::to_string(&session_response).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    
    assert_eq!(parsed["session_id"], session_id);
    assert_eq!(parsed["user_id"], user_id.to_string());
    assert_eq!(parsed["expires_at"], expires_at);
    
    // 从JSON反序列化
    let deserialized: orpheus::models::session::SessionResponse = serde_json::from_value(parsed).unwrap();
    assert_eq!(deserialized.session_id, session_id);
    assert_eq!(deserialized.user_id, user_id);
    assert_eq!(deserialized.expires_at, expires_at);
}

// 测试API响应模型
#[actix_web::test]
async fn test_api_response_model_with_data() {
    let data = "test_data".to_string();
    let api_response = orpheus::models::response::ApiResponse {
        code: 200,
        success: true,
        message: Some("Success".to_string()),
        data: Some(data.clone()),
    };
    
    // 序列化为JSON
    let json_str = serde_json::to_string(&api_response).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    
    assert_eq!(parsed["code"], 200);
    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["message"], "Success");
    assert_eq!(parsed["data"], data);
    
    // 从JSON反序列化
    let deserialized: orpheus::models::response::ApiResponse<String> = serde_json::from_value(parsed).unwrap();
    assert_eq!(deserialized.code, 200);
    assert_eq!(deserialized.success, true);
    assert_eq!(deserialized.message, Some("Success".to_string()));
    assert_eq!(deserialized.data, Some(data));
}

#[actix_web::test]
async fn test_api_response_model_without_data() {
    let api_response = orpheus::models::response::ApiResponse::<()> {
        code: 500,
        success: false,
        message: Some("Error".to_string()),
        data: None,
    };
    
    // 序列化为JSON
    let json_str = serde_json::to_string(&api_response).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    
    assert_eq!(parsed["code"], 500);
    assert_eq!(parsed["success"], false);
    assert_eq!(parsed["message"], "Error");
    assert!(parsed["data"].is_null());
    
    // 从JSON反序列化
    let deserialized: orpheus::models::response::ApiResponse<()> = serde_json::from_value(parsed).unwrap();
    assert_eq!(deserialized.code, 500);
    assert_eq!(deserialized.success, false);
    assert_eq!(deserialized.message, Some("Error".to_string()));
    assert!(deserialized.data.is_none());
}

#[actix_web::test]
async fn test_api_response_model_without_message() {
    let api_response = orpheus::models::response::ApiResponse::<()> {
        code: 200,
        success: true,
        message: None,
        data: None,
    };
    
    // 序列化为JSON
    let json_str = serde_json::to_string(&api_response).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    
    assert_eq!(parsed["code"], 200);
    assert_eq!(parsed["success"], true);
    assert!(parsed["message"].is_null());
    assert!(parsed["data"].is_null());
    
    // 从JSON反序列化
    let deserialized: orpheus::models::response::ApiResponse<()> = serde_json::from_value(parsed).unwrap();
    assert_eq!(deserialized.code, 200);
    assert_eq!(deserialized.success, true);
    assert!(deserialized.message.is_none());
    assert!(deserialized.data.is_none());
}

// 测试复杂数据类型的API响应
#[actix_web::test]
async fn test_api_response_model_with_complex_data() {
    use serde_json::json;
    
    let complex_data = json!({
        "user_id": Uuid::new_v4().to_string(),
        "profile": {
            "name": "John Doe",
            "email": "john@example.com",
            "age": 30
        },
        "settings": {
            "theme": "dark",
            "notifications": true
        }
    });
    
    let api_response = orpheus::models::response::ApiResponse {
        code: 200,
        success: true,
        message: Some("Profile retrieved".to_string()),
        data: Some(complex_data.clone()),
    };
    
    // 序列化为JSON
    let json_str = serde_json::to_string(&api_response).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    
    assert_eq!(parsed["code"], 200);
    assert_eq!(parsed["success"], true);
    assert_eq!(parsed["message"], "Profile retrieved");
    assert_eq!(parsed["data"], complex_data);
    
    // 从JSON反序列化
    let deserialized: orpheus::models::response::ApiResponse<serde_json::Value> = serde_json::from_value(parsed).unwrap();
    assert_eq!(deserialized.code, 200);
    assert_eq!(deserialized.success, true);
    assert_eq!(deserialized.message, Some("Profile retrieved".to_string()));
    assert_eq!(deserialized.data, Some(complex_data));
}

// 测试模型边界情况
#[actix_web::test]
async fn test_user_model_with_empty_strings() {
    let signup_request = orpheus::models::user::SignupRequest {
        username: "".to_string(),
        email: "".to_string(),
        password: "".to_string(),
    };
    
    // 序列化和反序列化空字符串应该正常工作
    let json_str = serde_json::to_string(&signup_request).unwrap();
    let deserialized: orpheus::models::user::SignupRequest = serde_json::from_str(&json_str).unwrap();
    
    assert_eq!(deserialized.username, "");
    assert_eq!(deserialized.email, "");
    assert_eq!(deserialized.password, "");
}

#[actix_web::test]
async fn test_user_model_with_very_long_strings() {
    let long_string = "a".repeat(10000);
    
    let signup_request = orpheus::models::user::SignupRequest {
        username: long_string.clone(),
        email: format!("{}@example.com", long_string),
        password: long_string.clone(),
    };
    
    // 序列化和反序列化超长字符串应该正常工作
    let json_str = serde_json::to_string(&signup_request).unwrap();
    let deserialized: orpheus::models::user::SignupRequest = serde_json::from_str(&json_str).unwrap();
    
    assert_eq!(deserialized.username, long_string);
    assert_eq!(deserialized.email, format!("{}@example.com", long_string));
    assert_eq!(deserialized.password, long_string);
}

#[actix_web::test]
async fn test_user_model_with_special_characters() {
    let special_chars = "!@#$%^&*()_+-=[]{}|;':\",./<>?`~\n\t\r";
    
    let signup_request = orpheus::models::user::SignupRequest {
        username: special_chars.to_string(),
        email: format!("{}@example.com", special_chars),
        password: special_chars.to_string(),
    };
    
    // 序列化和反序列化特殊字符应该正常工作
    let json_str = serde_json::to_string(&signup_request).unwrap();
    let deserialized: orpheus::models::user::SignupRequest = serde_json::from_str(&json_str).unwrap();
    
    assert_eq!(deserialized.username, special_chars);
    assert_eq!(deserialized.email, format!("{}@example.com", special_chars));
    assert_eq!(deserialized.password, special_chars);
}

#[actix_web::test]
async fn test_user_model_with_unicode_characters() {
    let unicode_chars = "中文测试 🚀 ñáéíóú кириллицا العربية";
    
    let signup_request = orpheus::models::user::SignupRequest {
        username: unicode_chars.to_string(),
        email: format!("{}@example.com", unicode_chars),
        password: unicode_chars.to_string(),
    };
    
    // 序列化和反序列化Unicode字符应该正常工作
    let json_str = serde_json::to_string(&signup_request).unwrap();
    let deserialized: orpheus::models::user::SignupRequest = serde_json::from_str(&json_str).unwrap();
    
    assert_eq!(deserialized.username, unicode_chars);
    assert_eq!(deserialized.email, format!("{}@example.com", unicode_chars));
    assert_eq!(deserialized.password, unicode_chars);
}

#[actix_web::test]
async fn test_api_response_model_with_different_codes() {
    let test_cases = vec![
        (200, true, "OK"),
        (201, true, "Created"),
        (400, false, "Bad Request"),
        (401, false, "Unauthorized"),
        (403, false, "Forbidden"),
        (404, false, "Not Found"),
        (500, false, "Internal Server Error"),
    ];
    
    for (code, success, message) in test_cases {
        let api_response = orpheus::models::response::ApiResponse::<()> {
            code,
            success,
            message: Some(message.to_string()),
            data: None,
        };
        
        // 序列化和反序列化
        let json_str = serde_json::to_string(&api_response).unwrap();
        let deserialized: orpheus::models::response::ApiResponse<()> = serde_json::from_str(&json_str).unwrap();
        
        assert_eq!(deserialized.code, code);
        assert_eq!(deserialized.success, success);
        assert_eq!(deserialized.message, Some(message.to_string()));
        assert!(deserialized.data.is_none());
    }
}

#[actix_web::test]
async fn test_session_response_model_with_boundary_timestamps() {
    let user_id = Uuid::new_v4();
    let session_id = "user_12345678-1234-1234-1234-123456789abc".to_string();
    
    let test_cases = vec![
        i64::MIN,      // 最小时间戳
        -86400,        // 昨天的时间戳
        0,             // Unix纪元
        86400,         // 明天的时间戳
        i64::MAX,      // 最大时间戳
    ];
    
    for expires_at in test_cases {
        let session_response = orpheus::models::session::SessionResponse {
            session_id: session_id.clone(),
            user_id,
            expires_at,
        };
        
        // 序列化和反序列化
        let json_str = serde_json::to_string(&session_response).unwrap();
        let deserialized: orpheus::models::session::SessionResponse = serde_json::from_str(&json_str).unwrap();
        
        assert_eq!(deserialized.session_id, session_id);
        assert_eq!(deserialized.user_id, user_id);
        assert_eq!(deserialized.expires_at, expires_at);
    }
}

#[actix_web::test]
async fn test_model_serialization_performance() {
    use std::time::Instant;
    
    let signup_request = orpheus::models::user::SignupRequest {
        username: "test_user".to_string(),
        email: "test@example.com".to_string(),
        password: "secure_password_123".to_string(),
    };
    
    // 测试序列化性能
    let start = Instant::now();
    for _ in 0..1000 {
        let _json_str = serde_json::to_string(&signup_request).unwrap();
    }
    let serialization_duration = start.elapsed();
    
    // 测试反序列化性能
    let json_str = serde_json::to_string(&signup_request).unwrap();
    let start = Instant::now();
    for _ in 0..1000 {
        let _: orpheus::models::user::SignupRequest = serde_json::from_str(&json_str).unwrap();
    }
    let deserialization_duration = start.elapsed();
    
    // 性能应该合理（这些阈值可能需要根据实际情况调整）
    assert!(serialization_duration.as_millis() < 100, "Serialization too slow: {}ms", serialization_duration.as_millis());
    assert!(deserialization_duration.as_millis() < 100, "Deserialization too slow: {}ms", deserialization_duration.as_millis());
}

#[actix_web::test]
async fn test_model_memory_usage() {
    use std::mem;
    
    let signup_request = orpheus::models::user::SignupRequest {
        username: "test_user".to_string(),
        email: "test@example.com".to_string(),
        password: "secure_password_123".to_string(),
    };
    
    let size = mem::size_of_val(&signup_request);
    
    // 检查模型大小是否合理（这些阈值可能需要根据实际情况调整）
    assert!(size < 1000, "Model size too large: {} bytes", size);
}

#[actix_web::test]
async fn test_model_with_null_values() {
    // 测试JSON中的null值处理
    let json_with_nulls = json!({
        "username": "test_user",
        "email": null,
        "password": "test_password"
    });
    
    // 尝试反序列化包含null值的JSON
    let result: Result<orpheus::models::user::SignupRequest, _> = serde_json::from_value(json_with_nulls);
    
    // 应该失败，因为email字段不能为null
    assert!(result.is_err());
}