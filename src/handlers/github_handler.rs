use actix_web::{get, Responder};
use serde_json::Value;
use crate::models::response::ApiResponse;

/// 获取指定 GitHub 仓库的星标数量
/// 路由: GET /github/stars/{owner}/{repo}
/// 参数:
/// - owner: 仓库所有者用户名
/// - repo: 仓库名称
/// 返回值:
/// - JSON 格式的响应，包含仓库星标数量或错误信息
/// # 行为说明
/// - 该处理器函数将调用 GitHub API 获取指定仓库的星标数量
/// - 如果请求成功，返回星标数量
/// - 如果请求失败，返回相应的错误信息

#[get("/github/stars/{owner}/{repo}")]
pub async fn get_github_repo_stars(path: actix_web::web::Path<(String, String)>) -> impl Responder {
    // 从路径参数中提取仓库所有者和名称
    let (owner, repo) = path.into_inner();
    // 构造 GitHub API 请求 URL
    let url = format!("https://api.github.com/repos/{}/{}", owner, repo);
    // 创建 HTTP 客户端
    let client = reqwest::Client::new();
    // 发送 GET 请求到 GitHub API
    // 设置 User-Agent 头以符合 GitHub API 要求
    let response = client
        .get(&url)
        .header("User-Agent", "Actix-web-App")
        .send()
        .await;
    let response = match response {
        Ok(resp) => resp,
        Err(_) => return actix_web::HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 500,
            success: false,
            message: Some("Failed to connect to GitHub API".to_string()),
            data: None,
        }),
    };
    // 解析响应 JSON
    let json_result: Result<Value, _> = response.json().await;
    let json = match json_result {
        Ok(v) => v,
        Err(_) => return actix_web::HttpResponse::InternalServerError().json(ApiResponse::<()> {
            code: 500,
            success: false,
            message: Some("Failed to parse GitHub API response".to_string()),
            data: None,
        }),
    };
    // 提取星标数量
    let stars = json.get("stargazers_count")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    // 返回星标数量的响应
    actix_web::HttpResponse::Ok().json(ApiResponse::<Value> {
        code: 200,
        success: true,
        message: Some("Repository stars fetched successfully".to_string()),
        data: Some(serde_json::json!({ "stars": stars })),
    })
}
