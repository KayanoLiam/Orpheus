// 导入 Actix-Web 相关组件，用于 HTTP 请求处理
// HTTPリクエスト処理用のActix-Web関連コンポーネントをインポート
use actix_web::{web, HttpResponse, Responder, post, get};
use uuid::Uuid;
// 导入会话存储管理器
// セッションストレージマネージャーをインポート
use crate::auth::session_store::SessionStore;
// 导入统一的 API 响应模型
// 統一APIレスポンスモデルをインポート
use crate::models::response::ApiResponse;

/// 用户登出处理器
/// 自令和7年11.17之后，不再提供中文注释
/// ユーザーログアウトハンドラー
/// 
/// 接收 POST /logout 请求，销毁用户会话并返回登出结果
/// POST /logoutリクエストを受け取り、ユーザーセッションを破棄してログアウト結果を返します
/// 
/// # 参数
/// # 引数
/// - `user_id`: 从认证中间件获取的用户 ID
/// - `user_id`: 認証ミドルウェアから取得したユーザーID
/// - `redis_client`: Redis 客户端连接
/// - `redis_client`: Redisクライアント接続
/// 
/// # 返回
/// # 戻り値
/// 成功时返回 200 状态码和登出成功消息
/// 失败时返回 500 状态码和错误信息
/// 成功時は200ステータスコードとログアウト成功メッセージを返します
/// 失敗時は500ステータスコードとエラー情報を返します
#[post("/logout")]
pub async fn user_logout(
    user_id: web::ReqData<Uuid>,
    redis_client: web::Data<redis::Client>,
) -> impl Responder {
    // 构造会话 ID，使用用户 ID 加前缀
    // ユーザーIDにプレフィックスを付けてセッションIDを構築
    let session_id = format!("user_{}", user_id.into_inner());
    // 创建会话存储管理器实例
    // セッションストレージマネージャーインスタンスを作成
    let session_store = SessionStore::new(redis_client.get_ref().clone());
    
    // 尝试销毁用户会话
    // ユーザーセッションの破棄を試行
    match session_store.destroy_session(&session_id).await {
        // 会话销毁成功
        // セッション破棄成功
        Ok(_) => {
            HttpResponse::Ok().json(ApiResponse::<()> {
                code: 200,
                success: true,
                message: Option::from("Logged out successfully".to_string()),
                data: None,
            })
        }
        // 会话销毁失败，记录错误并返回服务器错误
        // セッション破棄失敗、エラーを記録しサーバーエラーを返す
        Err(e) => {
            println!("Failed to destroy session: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                success: false,
                message: Option::from("Failed to logout".to_string()),
                data: None,
            })
        }
    }
}

/// 用户资料获取处理器
/// 自令和7年11.17之后，不再提供中文注释
/// ユーザープロフィール取得ハンドラー
/// 
/// 接收 GET /profile 请求，返回当前认证用户的基本信息
/// GET /profileリクエストを受け取り、現在認証されているユーザーの基本情報を返します
/// 
/// # 参数
/// # 引数
/// - `user_id`: 从认证中间件获取的用户 ID
/// - `user_id`: 認証ミドルウェアから取得したユーザーID
/// 
/// # 返回
/// # 戻り値
/// 返回包含用户 ID 的成功响应
/// 这是一个简单的示例实现，实际应用中可以返回更详细的用户信息
// 这个接口的测试貌似有点问题，需要先登陆，登陆时返回的session_id作为bearer token放在header里才能通过认证中间件
// 有时间需要修改
// ユーザーIDを含む成功レスポンスを返します
// これは簡単なサンプル実装で、実際のアプリケーションではより詳細なユーザー情報を返すことができます
// このインターフェースのテストには少し問題があるようです。まずログインする必要があり、ログイン時に返されるsession_idをベアラートークンとしてヘッダーに含めることで認証ミドルウェアを通過できます
// 時間があるときに修正する必要があります
#[get("/profile")]
pub async fn user_profile(
    user_id: web::ReqData<Uuid>,
) -> impl Responder {
    // 提取用户 ID
    // ユーザーIDを抽出
    let user_id = user_id.into_inner();
    // 返回用户 ID 作为用户资料信息
    // ユーザーIDをユーザープロフィール情報として返す
    HttpResponse::Ok().json(ApiResponse::<Uuid> {
        code: 200,
        success: true,
        message: Option::from("Profile retrieved successfully".to_string()),
        data: Some(user_id),
    })
}