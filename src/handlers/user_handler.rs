use crate::auth::session_store::SessionStore;
use crate::models::response::ApiResponse;
use crate::models::session::SessionResponse;
use crate::models::user::{ChangePasswordRequest, LoginRequest, SignupRequest};
use actix_web::{delete, post, web, HttpResponse, Responder};
use argon2::password_hash::rand_core::OsRng;
use argon2::{PasswordHasher, PasswordVerifier};
use sqlx::PgPool;
use tracing::{error, info, warn};

/// 用户注册处理器
/// 自令和7年11.17之后，不再提供中文注释
/// ユーザー登録ハンドラー
///
/// 接收 POST /signup 请求，创建新用户账号并存储到数据库
/// POST /signupリクエストを受け取り、新しいユーザーアカウントを作成してデータベースに保存します
///
/// # 参数
/// # 引数
/// - `pool`: PostgreSQL 数据库连接池
/// - `pool`: PostgreSQLデータベース接続プール
/// - `body`: 包含用户注册信息的 JSON 请求体（用户名、邮箱、密码）
/// - `body`: ユーザー登録情報を含むJSONリクエストボディ（ユーザー名、メール、パスワード）
///
/// # 返回
/// # 戻り値
/// 成功时返回 200 状态码和注册成功消息
/// 失败时返回 500 状态码和错误信息
/// 成功時は200ステータスコードと登録成功メッセージを返します
/// 失敗時は500ステータスコードとエラー情報を返します
///
/// # 安全特性
/// # セキュリティ特性
/// - 使用 Argon2 算法对密码进行安全哈希
/// - Argon2アルゴリズムを使用してパスワードを安全にハッシュ化
/// - 生成随机 salt 增强密码安全性
/// - ランダムなソルトを生成し、パスワードの安全性を強化
/// - 统一错误响应避免信息泄露
/// - 統一されたエラーレスポンスで情報漏洩を防止
#[post("/signup")]
pub async fn user_signup(
    pool: web::Data<PgPool>,
    body: web::Json<SignupRequest>,
) -> impl Responder {
    //读取用户输入
    // ユーザー入力を読み取り
    let username = &body.username;
    let email = &body.email;
    let password = &body.password;

    //生成安全的salt + hashed password
    // 安全なソルト+ハッシュ化パスワードを生成
    let salt = argon2::password_hash::SaltString::generate(&mut OsRng);
    let argon2 = argon2::Argon2::default();

    let hashed_password = match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        // Err(_) => return HttpResponse::InternalServerError().body("Something went wrong"),
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                success: false,
                message: Option::from("Something went wrong".to_string()),
                data: None,
            });
        }
    };
    //将用户信息存储到数据库
    // ユーザー情報をデータベースに保存
    let result = sqlx::query!(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3)",
        username,
        email,
        hashed_password
    )
    .execute(pool.get_ref())
    .await;

    match result {
        // Ok(_) => HttpResponse::Ok().body("User signed up successfully"),
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()> {
            code: 200,
            success: true,
            message: Option::from("User signed up successfully".to_string()),
            data: None,
        }),
        Err(e) => {
            println!("Failed to sign up user: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                success: false,
                message: Option::from("Failed to sign up user".to_string()),
                data: None,
            })
        }
    }
}

/// 用户登录处理器
/// 自令和7年11.17之后，不再提供中文注释
/// ユーザーログインハンドラー
///
/// 接收 POST /login 请求，验证用户凭据并创建会话
/// POST /loginリクエストを受け取り、ユーザー認証情報を検証してセッションを作成します
///
/// # 参数
/// # 引数
/// - `pool`: PostgreSQL 数据库连接池
/// - `pool`: PostgreSQLデータベース接続プール
/// - `body`: 包含用户登录信息的 JSON 请求体（邮箱、密码）
/// - `body`: ユーザーログイン情報を含むJSONリクエストボディ（メール、パスワード）
/// - `redis_client`: Redis 客户端连接，用于会话存储
/// - `redis_client`: セッションストレージ用のRedisクライアント接続
///
/// # 返回
/// # 戻り値
/// 成功时返回 200 状态码和会话信息（session_id、user_id、过期时间）
/// 认证失败时返回 401 状态码
/// 服务器错误时返回 500 状态码
/// 成功時は200ステータスコードとセッション情報（session_id、user_id、有効期限）を返します
/// 認証失敗時は401ステータスコードを返します
/// サーバーエラー時は500ステータスコードを返します
///
/// # 安全特性
/// # セキュリティ特性
/// - 使用 Argon2 算法验证密码哈希
/// - Argon2アルゴリズムを使用してパスワードハッシュを検証
/// - 统一错误响应避免用户枚举攻击
/// - 統一されたエラーレスポンスでユーザー列挙攻撃を防止
/// - 会话存储在 Redis 中，支持过期管理
/// - セッションはRedisに保存され、有効期限管理をサポート
/// - 返回的 session_id 用作 Bearer Token
/// - 返されたsession_idはBearer Tokenとして使用されます
#[post("/login")]
pub async fn user_login(
    pool: web::Data<PgPool>,
    body: web::Json<LoginRequest>,
    redis_client: web::Data<redis::Client>,
) -> impl Responder {
    let session_store = SessionStore::new(redis_client.get_ref().clone());
    //获取用户输入
    // ユーザー入力を取得
    let email = &body.email;
    let password = &body.password;
    //准备argon2实例
    // argon2インスタンスを準備
    let argon2 = argon2::Argon2::default();
    //查询数据库验证用户
    // データベースをクエリしてユーザーを検証
    let result = sqlx::query!("SELECT id,password_hash FROM users WHERE email = $1", email)
        .fetch_one(pool.get_ref())
        .await;
    match result {
        //将数据库中查询到的密码hash进行验证
        // データベースからクエリしたパスワードハッシュを検証
        Ok(record) => {
            let user_id = record.id;
            let parsed_hash = match argon2::password_hash::PasswordHash::new(&record.password_hash)
            {
                Ok(hash) => hash,
                Err(_) => {
                    return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                        code: 401,
                        success: false,
                        message: Option::from("Invalid credentials".to_string()),
                        data: None,
                    });
                }
            };
            //检验密码是否正确
            // パスワードが正しいかを検証
            match argon2.verify_password(password.as_bytes(), &parsed_hash) {
                Ok(_) => match session_store.create_session(record.id).await {
                    Ok(session_id) => {
                        let expires_at = chrono::Utc::now().timestamp()
                            + crate::config::SESSION_EXPIRY_SECONDS as i64;
                        let session_response = SessionResponse {
                            session_id,
                            user_id,
                            expires_at,
                        };
                        HttpResponse::Ok().json(ApiResponse::<SessionResponse> {
                            code: 200,
                            success: true,
                            message: Option::from("User logged in successfully".to_string()),
                            data: Some(session_response),
                        })
                    }
                    Err(e) => {
                        println!("Failed to create session: {:?}", e);
                        HttpResponse::InternalServerError().json(ApiResponse::<()> {
                            code: 500,
                            success: false,
                            message: Option::from("Failed to create session".to_string()),
                            data: None,
                        })
                    }
                },
                //密码错误，统一返回未授权，避免泄露用户是否存在的信息和暴力探测账号
                // パスワードが間違っている場合、認証失敗を統一返答し、ユーザーの存在情報漏洩やアカウント総当たり攻撃を防止
                Err(_) => HttpResponse::Unauthorized().json(ApiResponse::<()> {
                    code: 401,
                    success: false,
                    message: Option::from("Invalid credentials".to_string()),
                    data: None,
                }),
            }
        }
        //这里也返回未授权，避免泄露用户是否存在的信息和暴力探测账号
        // ここでも認証失敗を返答し、ユーザーの存在情報漏洩やアカウント総当たり攻撃を防止
        Err(sqlx::Error::RowNotFound) => HttpResponse::Unauthorized().json(ApiResponse::<()> {
            code: 401,
            success: false,
            message: Option::from("Invalid credentials".to_string()),
            data: None,
        }),
        //其他数据库错误
        // その他のデータベースエラー
        Err(e) => {
            println!("Failed to log in user: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                success: false,
                message: Option::from("Failed to log in user".to_string()),
                data: None,
            })
        }
    }
}

/// 用户密法重置处理器
/// 自令和7年11.17之后，不再提供中文注释
/// ユーザーパスワードリセットハンドラー
///
/// 从请求头读取Authorization: Bearer <session_id>
/// リクエストヘッダーからAuthorization: Bearer <session_id>を読み取り
/// 验证session_id有效后，获取user_id
/// session_idが有効なことを検証した後、user_idを取得
/// 从数据库查询旧密码哈希，验证旧密码正确性
/// データベースから古いパスワードハッシュをクエリし、古いパスワードの正しさを検証
/// 使用argon2对新密码进行哈希处理
/// argon2を使用して新しいパスワードをハッシュ化
/// 更新数据库中的密码哈希值
/// データベース内のパスワードハッシュ値を更新
/// 立即销毁session，要求用户重新登录
/// セッションを直ちに破棄し、ユーザーに再ログインを要求
/// 返回成功或失败响应
/// 成功または失敗のレスポンスを返す
///
/// # 参数
/// # 引数
/// - `pool`: PostgreSQL 数据库连接池
/// - `pool`: PostgreSQLデータベース接続プール
/// - `redis_client`: Redis 客户端连接，用于会话存储
/// - `redis_client`: セッションストレージ用のRedisクライアント接続
/// - `body`: 包含密码变更信息的 JSON 请求体（旧密码、新密码）
/// - `body`: パスワード変更情報を含むJSONリクエストボディ（旧パスワード、新パスワード）
/// - `req`: HTTP 请求对象，用于读取Authorization头
/// - `req`: Authorizationヘッダーを読み取るためのHTTPリクエストオブジェクト
///
/// # 返回
/// # 戻り値
/// 成功时返回 200 状态码和成功消息
/// 认证失败时返回 401 状态码
/// 服务器错误时返回 500 状态码
/// 成功時は200ステータスコードと成功メッセージを返します
/// 認証失敗時は401ステータスコードを返します
/// サーバーエラー時は500ステータスコードを返します
/// # 安全特性
/// # セキュリティ特性
/// - 使用 Argon2 算法验证和哈希密码
/// - Argon2アルゴリズムを使用してパスワードを検証・ハッシュ化
/// - 统一错误响应避免信息泄露
/// - 統一されたエラーレスポンスで情報漏洩を防止
/// - 会话存储在 Redis 中，支持过期管理
/// - セッションはRedisに保存され、有効期限管理をサポート
/// - 密码更改后立即销毁会话，要求重新登录
/// - パスワード変更後、直ちにセッションを破棄し、再ログインを要求
#[post("/reset_password")]
pub async fn user_reset_password(
    pool: web::Data<PgPool>,
    redis_client: web::Data<redis::Client>,
    body: web::Json<ChangePasswordRequest>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    // 使用已封装的SessionStore操作Redis会话，避免重复代码
    // カプセル化されたSessionStoreを使用してRedisセッションを操作し、コードの重複を回避
    let session_store = SessionStore::new(redis_client.get_ref().clone());
    // 1.从请求体中获取session_id
    // 1. リクエストヘッダーからsession_idを取得
    let auth_header = req.headers().get("Authorization");
    let session_id = match auth_header
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
    {
        Some(token) => token,
        _ => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                code: 401,
                success: false,
                message: Some("Missing or invalid Authorization header".to_string()),
                data: None,
            });
        }
    };
    // 2.验证session_id有效性，获取user_id
    // 2. session_idの有効性を検証し、user_idを取得
    let user_id = match session_store.get_user_id(session_id).await {
        Ok(Some(uid)) => uid,
        Ok(None) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                code: 401,
                success: false,
                message: Some("Invalid or expired session".to_string()),
                data: None,
            });
        }
        Err(e) => {
            println!("Failed to validate session: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                success: false,
                message: Some("Failed to validate session".to_string()),
                data: None,
            });
        }
    };
    // 3.从数据库获取旧密码哈希
    // 3. データベースから古いパスワードハッシュを取得
    let result = sqlx::query!("SELECT password_hash FROM users WHERE id = $1", user_id)
        .fetch_one(pool.get_ref())
        .await;
    let record = match result {
        Ok(rec) => rec,
        Err(e) => {
            println!("Failed to fetch user record: {:?}", e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                success: false,
                message: Some("Failed to fetch user record".to_string()),
                data: None,
            });
        }
    };
    // 4.验证旧密码正确性
    // 4. 古いパスワードの正しさを検証
    let argon2 = argon2::Argon2::default();
    let parsed_hash = match argon2::password_hash::PasswordHash::new(&record.password_hash) {
        Ok(hash) => hash,
        Err(_) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                code: 401,
                success: false,
                message: Some("Invalid old password".to_string()),
                data: None,
            });
        }
    };
    // 检验旧密码是否正确
    // 古いパスワードが正しいかを検証
    match argon2.verify_password(body.old_password.as_bytes(), &parsed_hash) {
        Ok(_) => {
            // 5.对新密码进行哈希处理
            // 5. 新しいパスワードをハッシュ化
            let salt = argon2::password_hash::SaltString::generate(&mut OsRng);
            let new_hashed_password =
                match argon2.hash_password(body.new_password.as_bytes(), &salt) {
                    Ok(hash) => hash.to_string(),
                    Err(_) => {
                        return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                            code: 500,
                            success: false,
                            message: Some("Something went wrong".to_string()),
                            data: None,
                        });
                    }
                };
            // 6.更新数据库中的密码哈希值
            // 6. データベース内のパスワードハッシュ値を更新
            let update_result = sqlx::query!(
                "UPDATE users SET password_hash = $1 WHERE id = $2",
                new_hashed_password,
                user_id
            )
            .execute(pool.get_ref())
            .await;
            match update_result {
                Ok(_) => {
                    // 7.销毁session，要求重新登录
                    // 7. セッションを破棄し、再ログインを要求
                    if let Err(e) = session_store.destroy_session(session_id).await {
                        println!("Failed to destroy session: {:?}", e);
                    }
                    HttpResponse::Ok().json(ApiResponse::<()> {
                        code: 200,
                        success: true,
                        message: Some(
                            "Password changed successfully. Please log in again.".to_string(),
                        ),
                        data: None,
                    })
                }
                Err(e) => {
                    println!("Failed to update password: {:?}", e);
                    HttpResponse::InternalServerError().json(ApiResponse::<()> {
                        code: 500,
                        success: false,
                        message: Some("Failed to update password".to_string()),
                        data: None,
                    })
                }
            }
        }
        //旧密码错误
        // 古いパスワードが間違っている場合
        Err(_) => HttpResponse::Unauthorized().json(ApiResponse::<()> {
            code: 401,
            success: false,
            message: Some("Invalid old password".to_string()),
            data: None,
        }),
    }
}

/// 用户删除处理器
/// 自令和7年11.17之后，不再提供中文注释
/// ユーザー削除ハンドラー
///
/// 先从请求头读取Authorization: Bearer <session_id>
/// まずリクエストヘッダーからAuthorization: Bearer <session_id>を読み取り
/// 验证session_id有效后，获取user_id
/// session_idが有効なことを検証した後、user_idを取得
/// 从数据库删除用户记录
/// データベースからユーザーレコードを削除
/// 销毁session，要求用户重新注册登录
/// セッションを破棄し、ユーザーに再登録・ログインを要求
/// 返回成功或失败响应
/// 成功または失敗のレスポンスを返す
/// # 参数
/// # 引数
/// - `pool`: PostgreSQL 数据库连接池
/// - `pool`: PostgreSQLデータベース接続プール
/// - `redis_client`: Redis 客户端连接，用于会话存储
/// - `redis_client`: セッションストレージ用のRedisクライアント接続
/// - `req`: HTTP 请求对象，用于读取Authorization头
/// - `req`: Authorizationヘッダーを読み取るためのHTTPリクエストオブジェクト
///
/// # 返回
/// # 戻り値
/// 成功时返回 200 状态码和成功消息
/// 认证失败时返回 401 状态码
/// 服务器错误时返回 500 状态码
/// 成功時は200ステータスコードと成功メッセージを返します
/// 認証失敗時は401ステータスコードを返します
/// サーバーエラー時は500ステータスコードを返します
#[delete("/delete")]
pub async fn user_delete(
    pool: web::Data<PgPool>,
    redis_client: web::Data<redis::Client>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    let session_store = SessionStore::new(redis_client.get_ref().clone());
    // 1.从请求体中获取session_id
    // 1. リクエストヘッダーからsession_idを取得
    let auth_header = req.headers().get("Authorization");
    let session_id = match auth_header
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
    {
        Some(token) => token,
        _ => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                code: 401,
                success: false,
                message: Some("Missing or invalid Authorization header".to_string()),
                data: None,
            });
        }
    };
    tracing::Span::current().record("session_id", &session_id);
    // 2.验证session_id有效性，获取user_id
    // 2. session_idの有効性を検証し、user_idを取得
    let user_id = match session_store.get_user_id(session_id).await {
        Ok(Some(uid)) => uid,
        Ok(None) => {
            warn!("Invalid or expired session for session_id: {}", session_id);
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                code: 401,
                success: false,
                message: Some("Invalid or expired session".to_string()),
                data: None,
            });
        }
        Err(e) => {
            // println!("Failed to validate session: {:?}", e);
            error!("Failed to validate session for session_id {}: {:?}", session_id, e);
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                success: false,
                message: Some("Failed to validate session".to_string()),
                data: None,
            });
        }
    };
    // 3.从数据库删除用户记录
    // 3. データベースからユーザーレコードを削除
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(pool.get_ref())
        .await;
    match result {
        Ok(_) => {
            info!("User with id {} deleted successfully", user_id);
            // 4.销毁session，要求重新注册登录
            // 4. セッションを破棄し、再登録・ログインを要求
            if let Err(e) = session_store.destroy_session(session_id).await {
                // println!("Failed to destroy session: {:?}", e);
                error!("Failed to destroy session for session_id {}: {:?}", session_id, e);
            }
            HttpResponse::Ok().json(ApiResponse::<()> {
                code: 200,
                success: true,
                message: Some("User deleted successfully.".to_string()),
                data: None,
            })
        }
        Err(e) => {
            // println!("Failed to delete user: {:?}", e);
            error!("Failed to delete user with id {}: {:?}", user_id, e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                code: 500,
                success: false,
                message: Some("Failed to delete user".to_string()),
                data: None,
            })
        }
    }
}
