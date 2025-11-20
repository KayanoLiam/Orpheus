use actix_web::{get, HttpResponse, Responder};

#[get("/auth/status")]
pub async fn auth_status(
    req: actix_web::HttpRequest,
    redis_client: actix_web::web::Data<redis::Client>,
) -> impl Responder {
    //　REDIS を使った　SESSIONSTORE を初期化
    let session_store =
        crate::auth::session_store::SessionStore::new(redis_client.get_ref().clone());
    // =====================================================================
    // 1. Authorization ヘッダから session_id を取得する
    // =====================================================================
    // フロントは毎回:
    //   Authorization: <session_id>
    // をつけて送ってくる仕様
    // =====================================================================
    if let Some(session_id_header) = req.headers().get("Authorization") {
        if let Ok(session_id) = session_id_header.to_str() {
            // =================================================================
            // 2. session_id を使って Redis から user_id を取得する
            // =================================================================
            match session_store.get_user_id(session_id).await {
                Ok(Some(user_id)) => {
                    // 認証成功
                    return actix_web::HttpResponse::Ok().json(
                        crate::models::response::ApiResponse::<serde_json::Value> {
                            code: 200,
                            success: true,
                            message: Some("Authenticated".to_string()),
                            data: Some(serde_json::json!({ "user_id": user_id })),
                        },
                    );
                }
                Ok(None) => {
                    // セッションが無効または期限切れ
                    return actix_web::HttpResponse::Unauthorized().json(
                        crate::models::response::ApiResponse::<()> {
                            code: 401,
                            success: false,
                            message: Some("Invalid or expired session".to_string()),
                            data: None,
                        },
                    );
                }
                Err(e) => {
                    println!("Error accessing Redis: {:?}", e);
                    return actix_web::HttpResponse::InternalServerError().json(
                        crate::models::response::ApiResponse::<()> {
                            code: 500,
                            success: false,
                            message: Some("Internal server error".to_string()),
                            data: None,
                        },
                    );
                }
            }
        }
    }
    // Authorization ヘッダ無し → 未ログイン扱い → 401
    HttpResponse::Unauthorized().json(
        crate::models::response::ApiResponse::<()> {
            code: 401,
            success: false,
            message: Some("Missing session".into()),
            data: None,
        },
    )
}
