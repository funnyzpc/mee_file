use actix_web::{web, HttpResponse, HttpRequest};
use std::fs;
use std::collections::HashMap;
use std::path::Path;

/*#[path= "../util/auth_util.rs"] mod auth_util;
#[path= "../structs/mod.rs"] mod structs;*/
use crate::structs::result_build::ResultBuild;
use crate::util::auth_util;

// 列出指定目录文件
pub async fn list_api(request: HttpRequest, params: web::Query<HashMap<String,String>>) -> HttpResponse {
    let headers = request.headers();
    println!("list_file::headers=>{:?}",headers);
    info!("list_file::headers=>{:?}",headers);
    println!("list_file::params=>{:?}",&params);
    info!("list_file::params=>{:?}",&params);
    let (auth_result,msg) = auth_util::auth(headers);
    if !auth_result{
        println!("验证[不]通过:{}",msg);
        return HttpResponse::Ok().json(ResultBuild::<i8>::fail_with_msg(msg));
    }

    let base_dir = std::env::var("BASE_DIR").unwrap();
    // example: oneleaf_tb_ad/2021/05/20
    let file_dir = params.get("file_dir");
    if file_dir.is_none(){
        return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("参数缺失[file_dir]"));
    }
    let file_path = file_dir.unwrap();

    let file_full_path = format!("{}/{}",base_dir,file_path);
    let file_path_object = Path::new(file_full_path.as_str());
    if !file_path_object.exists() {
        let msg = format!("下载文件不存在:{}",&file_path);
        println!("{}",msg);
        return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg(msg.as_str()));
    }
    // let file_name = file_path_object.file_name().unwrap().to_str();
    let file_data = fs::read_dir(file_path_object).unwrap();
    //let file_data = fs::read_dir(file_path_object)?;
    let mut user_file_list:Vec<String> = Vec::new();

    for path in file_data {
        // println!("Name: {}", &path.unwrap().path().display());
        user_file_list.push(format!("{}/{}",file_dir.unwrap(),path.unwrap().path().file_name().unwrap().to_str().unwrap()));
    }
    HttpResponse::Ok().json(ResultBuild::success_with_data(user_file_list))

}