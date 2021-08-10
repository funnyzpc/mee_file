use actix_web::{web, HttpResponse, HttpRequest};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Local, Timelike};

/*#[path= "../util/auth_util.rs"] mod auth_util;
#[path= "../structs/handle_page"] mod structs;*/

use crate::structs::result_build::ResultBuild;
use crate::structs::download_file::DownLoadFile;
use crate::util::auth_util;

/// 文件上传模块

// const BASE_DIR:&str = if cfg!(target_os = "windows") {  "D:\\tmp" }else{ "/tmp" };

// 下载文件
pub async fn download_api(request: HttpRequest, file: web::Query<DownLoadFile>) -> HttpResponse {
    println!("enter=>download_api");
    let headers = request.headers();
    println!("download_file::headers=>{:?}",headers);
    println!("download_file::file=>{:?}",&file);

    let (auth_result,msg) = auth_util::auth(headers);
    if !auth_result{
        println!("验证[不]通过:{}",msg);
        return HttpResponse::NotFound().json(ResultBuild::<i8>::fail_with_msg(msg));
    }

    let base_dir = std::env::var("BASE_DIR").unwrap();
    // example: oneleaf_tb_ad/2104211116461000.xlsx
    let file_path = &file.file_path.as_ref();
    if file_path.is_none(){
        return HttpResponse::NotFound().json(ResultBuild::<String>::fail_with_msg("参数缺失[file_path]"));
    }
    let file_path = file_path.unwrap();

    // example: /tmp/oneleaf_tb_ad/2104211116461000.xlsx
    let file_full_path = format!("{}/{}",base_dir,file_path);
    let file_path_object = Path::new(file_full_path.as_str());
    if !file_path_object.exists() {
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

#[allow(dead_code)]
fn achive_file(file:&String){
    let local = Local::now();
    if local.hour() > 12 {
        println!(
            "开始执行图片清理 => {:?}",
            local.format("%Y-%m-%d %H:%M:%S").to_string()
        );
        // 归档文件
        exec_archive(&Path::new(file))
    }
}

// 归档文件
#[allow(dead_code)]
fn exec_archive(file_dir: &Path) {
    // let path = Path::new("D:\\tmp\\2101111000099004.xls");
    for entry in fs::read_dir(file_dir).expect("读取目录出错") {
        let ett = entry.expect("获取文件出错");
        // let mut metadata = entry.expect("xxx").metadata().expect("xxx2");
        let metadata = ett.metadata().expect("获取文件头信息失败");
        // file modified time
        let stime = metadata.modified().expect("未获取到文件创建时间");
        let create_time = stime
            .duration_since(UNIX_EPOCH)
            .expect("格式化间隔时间出错");
        // println!("文件修改时间:{:?}", mod_time);
        let now = SystemTime::now();
        let now_time = now.duration_since(UNIX_EPOCH).expect("格式化当前时间出错");
        // timeout 15 minutes will be remove
        if (now_time - create_time).as_secs() / 60 > 15 {
            let file = &ett.path();
            println!("移除文件或文件夹 => {:?}", file.to_str().unwrap());
            // fs::remove_file(ett.path()).expect( "移除文件失败..."+file.display());
            if file.is_file() {
                fs::remove_file(ett.path()).expect(
                    format!("{} => {}", "移除文件失败...", file.to_str().unwrap()).as_str(),
                );
            } else {
                fs::remove_dir_all(ett.path()).expect(
                    format!("{} => {}", "移除目录失败...", file.to_str().unwrap()).as_str(),
                );
            }
        }
    }
}