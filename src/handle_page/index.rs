use actix_web::{HttpResponse};

/// 路由业务处理模块

// 主页
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(INDEX_HTML)
}

const  INDEX_HTML:&str =
    r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta content="application/xhtml+xml;charset=UTF-8" http-equiv="Content-Type">
        <meta content="no-cache,must-revalidate" http-equiv="Cache-Control">
        <meta content="no-cache" http-equiv="pragma">
        <meta content="0" http-equiv="expires">
        <meta http-equiv="Cache" content="no-cache">
        <meta content="telephone=no, address=no" name="format-detection">
        <meta content="width=device-width, initial-scale=1.0" name="viewport">
        <meta name="apple-mobile-web-app-capable" content="yes" />
        <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent" />
        <title>MEE文件中心</title>
    </head>
    <body>
    <div class="main">
        <div class="idx_title">
            <h2>MEE 文件中心</h2>
        </div>
        <div class="idx_list">
            <p></p>
            <p>📁 <a href="/mee_file/list" >文件目录中心</a></p>
            <p></p>
        </div>
    </div>
    </body>
    <style>
        body{font-size:24px;}
        .main{top:0;left:0;width:40%;margin-left:30%;margin-top:2%;}
        .idx_title{text-align:center;color:#333;text-shadow: 1px 1px 4px #747478;}
    </style>
    </html>
    "#;