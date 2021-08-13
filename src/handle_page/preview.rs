use actix_web::{web, HttpResponse};
use std::collections::HashMap;
use crate::structs::result_build::ResultBuild;
use std::path::Path;
use std::{fs};

// 预览文件
pub async fn preview(params:web::Query<HashMap<String,String>>) -> HttpResponse {
    println!("mee_file => preview:{:?}",params);
    let  base_dir = std::env::var("BASE_DIR").unwrap();
    // 查找参数目录
    let file_path = params.get("file_path");
    if file_path.is_none(){
        return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("参数缺失[file_path]"));
    }
    let file_path = file_path.unwrap();
    let file_full_path = format!("{}/{}",base_dir,file_path);
    let file_path_object = Path::new(&file_full_path);
    if !file_path_object.exists() || file_path_object.is_dir() {
        return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("文件不存在~"));
    }
    let file_path = file_path.to_lowercase();
    // 文本
    if file_path.ends_with(".txt")
        || file_path.ends_with(".java")
        || file_path.ends_with(".xml")
        || file_path.ends_with(".c")
        || file_path.ends_with(".go")
        || file_path.ends_with(".csv")
        || file_path.ends_with(".md")
        || file_path.ends_with(".csv")
        || file_path.ends_with(".sql")
        || file_path.ends_with(".swift"){
        let file_content = fs::read_to_string(file_path_object);
        if file_content.is_err(){
            return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("文件读取失败~"));
        }
        let dec_content = &file_content.unwrap()
            //.replace("\n","\r\n")
            .replace("	","&nbsp;&nbsp;")
            .replace(" ","&nbsp;")
            .replace("<","&lt;")
            .replace(">","&gt;")
            .replace("\\\\\"","\\\"")
            .replace("\n","<br/>")
            ;
        return HttpResponse::Ok().content_type("text/html; charset=utf-8").body(dec_content);
    }

    // 图形图像视频
    if file_path.ends_with(".png")
        || file_path.ends_with(".jpg")
        || file_path.ends_with(".jpeg")
        || file_path.ends_with(".mp3")
        || file_path.ends_with(".mp4")
        || file_path.ends_with(".mpeg")
        || file_path.ends_with(".pdf"){
        let file_data = fs::read(file_path_object);
        if file_data.is_err(){
            return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("图文读取失败~"));
        }
        let file_data = file_data.unwrap();

        // response
        if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg"){
            return HttpResponse::Ok().content_type("image/jpeg").body(file_data);
        }
        if file_path.ends_with(".png"){
            return HttpResponse::Ok().content_type("image/png").body(file_data);
        }
        if file_path.ends_with(".mp3"){
            return HttpResponse::Ok().content_type("audio/mpeg").body(file_data);
        }
        if file_path.ends_with(".mp4") || file_path.ends_with(".mpeg"){
            return HttpResponse::Ok().content_type("video/mpeg").body(file_data);
        }
        return HttpResponse::Ok().content_type("application/pdf").body(file_data);
    }

    // 返回
    return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("读取文件失败[2]"));

}