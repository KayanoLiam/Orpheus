// 导入 Actix-Web 相关组件，用于 HTTP 请求处理和中间件开发
// HTTPリクエスト処理とミドルウェア開発用のActix-Web関連コンポーネントをインポート
use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web::web::Data;
// 导入 Bearer Token 认证提取器
// Bearer Token認証エクストラクターをインポート
use actix_web_httpauth::extractors::bearer::BearerAuth;
// 导入会话存储管理器
// セッションストレージマネージャーをインポート
use crate::auth::session_store::SessionStore;

/// 会话验证中间件
/// 自令和7年11.17之后，不再提供中文注释
/// セッション検証ミドルウェア
/// 
/// 该函数作为 HTTP 认证中间件，验证请求中的 Bearer Token 是否有效
/// 通过检查 Redis 中的会话信息来确认用户身份
/// この関数はHTTP認証ミドルウェアとして機能し、リクエスト内のBearer Tokenが有効かどうかを検証します
/// Redis内のセッション情報をチェックしてユーザー身份を確認します
/// 
/// # 参数
/// # 引数
/// - `req`: HTTP 请求对象，包含应用数据和请求信息
/// - `req`: アプリケーションデータとリクエスト情報を含むHTTPリクエストオブジェクト
/// - `credentials`: 从 Authorization 头中提取的 Bearer Token
/// - `credentials`: Authorizationヘッダーから抽出されたBearer Token
/// 
/// # 返回值
/// # 戻り値
/// - `Ok(ServiceRequest)`: 验证成功，继续处理请求
/// - `Ok(ServiceRequest)`: 検証成功、リクエスト処理を継続
/// - `Err((Error, ServiceRequest))`: 验证失败，返回错误信息和原始请求
/// - `Err((Error, ServiceRequest))`: 検証失敗、エラー情報と元のリクエストを返す
/// 
/// # 验证流程
/// # 検証フロー
/// 1. 从 Bearer Token 中提取 session_id
/// 1. Bearer Tokenからsession_idを抽出
/// 2. 从应用数据中获取 Redis 客户端
/// 2. アプリケーションデータからRedisクライアントを取得
/// 3. 使用 session_id 查询 Redis 获取用户 ID
/// 3. session_idを使用してRedisをクエリしユーザーIDを取得
/// 4. 如果验证成功，将用户 ID 添加到请求扩展中
/// 4. 検証成功の場合、ユーザーIDをリクエスト拡張に追加
/// 5. 如果验证失败，返回相应的错误响应
/// 5. 検証失敗の場合、対応するエラーレスポンスを返す
pub async fn session_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // 从 Bearer Token 中提取会话 ID
    // Bearer TokenからセッションIDを抽出
    let session_id = credentials.token();
    
    // 从应用数据中获取 Redis 客户端实例
    // 如果没有找到 Redis 客户端，说明服务器配置有问题
    // アプリケーションデータからRedisクライアントインスタンスを取得
    // Redisクライアントが見つからない場合、サーバー設定に問題があることを示します
    let redis_client = match req.app_data::<Data<redis::Client>>() {
        Some(client) => client,
        None => {
            // 返回内部服务器错误，表示 Redis 客户端不可用
            // Redisクライアントが利用できないことを示す内部サーバーエラーを返す
            let error = actix_web::error::ErrorInternalServerError("Redis client not available");
            return Err((error, req));
        }
    };
    
    // 创建会话存储管理器实例，用于与 Redis 交互
    // Redisとの対話用にセッションストレージマネージャーインスタンスを作成
    let session_store = SessionStore::new(redis_client.get_ref().clone());
    
    // 尝试从 Redis 中获取会话对应的用户 ID
    // Redisからセッションに対応するユーザーIDの取得を試行
    match session_store.get_user_id(session_id).await {
        // 成功获取到用户 ID，说明会话有效
        // ユーザーIDの取得に成功し、セッションが有効であることを示します
        Ok(Some(user_id)) => {
            // 将用户 ID 添加到请求扩展中，供后续处理器使用
            // 後続ハンドラーが使用できるように、ユーザーIDをリクエスト拡張に追加
            req.extensions_mut().insert(user_id);
            // 验证成功，继续处理请求
            // 検証成功、リクエスト処理を継続
            Ok(req)
        }
        // 没有找到对应的用户 ID，说明会话无效或已过期
        // 対応するユーザーIDが見つからず、セッションが無効または期限切れであることを示します
        Ok(None) => {
            // 返回未授权错误，提示客户端会话无效
            // セッションが無効であることを示す認証失敗エラーを返す
            let error = actix_web::error::ErrorUnauthorized("Invalid or expired session");
            Err((error, req))
        }
        // 查询过程中发生错误（如 Redis 连接问题）
        // クエリ中にエラーが発生（例：Redis接続問題）
        Err(_) => {
            // 返回内部服务器错误，表示会话验证失败
            // セッション検証失敗を示す内部サーバーエラーを返す
            let error = actix_web::error::ErrorInternalServerError("Failed to validate session");
            Err((error, req))
        }
    }
}