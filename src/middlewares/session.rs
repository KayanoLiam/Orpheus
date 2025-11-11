// 导入 Actix-Web 相关组件，用于 HTTP 请求处理和中间件开发
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web::web::Data;
// 导入 Bearer Token 认证提取器
use actix_web_httpauth::extractors::bearer::BearerAuth;
// 导入会话存储管理器
use crate::auth::session_store::SessionStore;

/// 会话验证中间件
/// 
/// 该函数作为 HTTP 认证中间件，验证请求中的 Bearer Token 是否有效
/// 通过检查 Redis 中的会话信息来确认用户身份
/// 
/// # 参数
/// - `req`: HTTP 请求对象，包含应用数据和请求信息
/// - `credentials`: 从 Authorization 头中提取的 Bearer Token
/// 
/// # 返回值
/// - `Ok(ServiceRequest)`: 验证成功，继续处理请求
/// - `Err((Error, ServiceRequest))`: 验证失败，返回错误信息和原始请求
/// 
/// # 验证流程
/// 1. 从 Bearer Token 中提取 session_id
/// 2. 从应用数据中获取 Redis 客户端
/// 3. 使用 session_id 查询 Redis 获取用户 ID
/// 4. 如果验证成功，将用户 ID 添加到请求扩展中
/// 5. 如果验证失败，返回相应的错误响应
pub async fn session_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // 从 Bearer Token 中提取会话 ID
    let session_id = credentials.token();
    
    // 从应用数据中获取 Redis 客户端实例
    // 如果没有找到 Redis 客户端，说明服务器配置有问题
    let redis_client = match req.app_data::<Data<redis::Client>>() {
        Some(client) => client,
        None => {
            // 返回内部服务器错误，表示 Redis 客户端不可用
            let error = actix_web::error::ErrorInternalServerError("Redis client not available");
            return Err((error, req));
        }
    };
    
    // 创建会话存储管理器实例，用于与 Redis 交互
    let session_store = SessionStore::new(redis_client.get_ref().clone());
    
    // 尝试从 Redis 中获取会话对应的用户 ID
    match session_store.get_user_id(session_id).await {
        // 成功获取到用户 ID，说明会话有效
        Ok(Some(user_id)) => {
            // 将用户 ID 添加到请求扩展中，供后续处理器使用
            req.extensions_mut().insert(user_id);
            // 验证成功，继续处理请求
            Ok(req)
        }
        // 没有找到对应的用户 ID，说明会话无效或已过期
        Ok(None) => {
            // 返回未授权错误，提示客户端会话无效
            let error = actix_web::error::ErrorUnauthorized("Invalid or expired session");
            Err((error, req))
        }
        // 查询过程中发生错误（如 Redis 连接问题）
        Err(_) => {
            // 返回内部服务器错误，表示会话验证失败
            let error = actix_web::error::ErrorInternalServerError("Failed to validate session");
            Err((error, req))
        }
    }
}