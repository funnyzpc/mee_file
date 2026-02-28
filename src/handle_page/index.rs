use actix_web::{HttpResponse};

/// 路由业务处理模块

const INDEX_HTML: &str = include_str!("../html/index.html");


// 主页
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(INDEX_HTML)
}