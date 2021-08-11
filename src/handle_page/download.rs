use actix_web::{web, HttpResponse};
use std::fs;
use std::path::Path;

use crate::structs::result_build::ResultBuild;
use crate::structs::download_file::DownLoadFile;

// const BASE_DIR:&str = if cfg!(target_os = "windows") {  "D:\\tmp" }else{ "/tmp" };

// 用户下载文件
pub async fn download(/*request: HttpRequest, */file: web::Query<DownLoadFile>) -> HttpResponse {
    // println!("enter=>download");
    // let headers = request.headers();
    // println!("download_file::headers=>{:?}",headers);
    // println!("download_file::file=>{:?}",&file);

    let base_dir = std::env::var("BASE_DIR").unwrap();
    let file_path = &file.file_path.as_ref();
    if file_path.is_none(){
        return HttpResponse::NotFound().json(ResultBuild::<String>::fail_with_msg("参数缺失[file_path]"));
    }
    let file_path = file_path.unwrap();

    // example: /tmp/oneleaf_tb_ad/2104211116461000.xlsx
    let file_full_path = format!("{}/{}",base_dir,file_path);
    let file_path_object = Path::new(&file_full_path);
    if !file_path_object.exists() || file_path_object.is_dir() {
        let msg = format!("下载文件不存在:{}",&file_path);
        println!("{}",msg);
        return HttpResponse::NotFound().json(ResultBuild::<&str>::fail_with_msg(msg.as_str()));
    }
    // example: 2104211116461000.xlsx
    let file_name = file_path_object.file_name().unwrap().to_str();
    // achive_file(&file_full_path);
    let file_data = fs::read(file_path_object).unwrap();
    let content_disposition = format!("form-data; filename={}",file_name.unwrap());
    HttpResponse::Ok().header("Content-Disposition",content_disposition).body(file_data)
}
