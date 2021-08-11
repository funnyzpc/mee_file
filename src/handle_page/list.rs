use actix_web::{HttpResponse, web};
use handlebars::Handlebars;

// 用户文件列表
pub async fn list(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    println!("mee_file => list");
    //let file_dir = std::env::var("base_dir").unwrap();
    let file_dir = "/";
    let context_path = std::env::var("CONTEXT_PATH").unwrap();
    let file_list =  json!([
    {"date":"2021-08-10","file_name":"Tmp.java","file_path":"Tmp.java","size":"400KB"},
    {"date":"2021-08-11","file_name":"文件名称002.xls","file_path":"/base/文件名称002.xls","size":"2MB"},
    ]);
    let data_model = json!({"context_path":context_path,"file_dir":file_dir,"file_list":file_list});
    let result_html = hb.render_template(LIST_HTML,&data_model).unwrap_or(String::from("<p>获取目录失败</p>"));

    // 返回
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(result_html)
}

const  LIST_HTML:&str =
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
            <h2>目录: {{file_dir}}</h2>
        </div>
        <div class="idx_list">
            <div class="list_item list_bold">
                <div class="list_first">文件</div>
                <div class="list_second">创建日期</div>
                <div class="list_third">文件大小</div>
            </div>
            <div class="list_item">
                <div class="list_first"><a href="{{../context_path}}/download?file_path={{file_path}}">上一级..</a></div>
            </div>
            {{#each file_list}}
            <div class="list_item">
                <div class="list_first">📄 <a href="{{../context_path}}/download?file_path={{file_path}}" target="_blank">{{file_name}}</a></div>
                <div class="list_second">{{date}} </div>
                <div class="list_third">{{size}} </div>
            </div>
            {{/each}}
        </div>
    </div>
    </body>
    <style>
        body{font-size:18px;}
        .main{top:0;left:0;width:80%;margin-left:10%;margin-top:2%;}
        .idx_title{color:#333;}
        .list_bold{font-weight:bold;font-size: 18px;}
        .list_item{margin:4px 8px;}
        .list_first{width:60%;display:inline-block;}
        .list_second{width:15%;display:inline-block;}
        .list_third{width:10%;display:inline-block;}
    </style>
    </html>
    "#;