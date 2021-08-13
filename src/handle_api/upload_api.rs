use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse, HttpRequest};
use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use std::fs;

/*#[path= "../util/auth_util.rs"] mod auth_util;
#[path= "../structs/handle_page"] mod structs;*/
use crate::structs::result_build::ResultBuild;
use crate::util::auth_util;
use urlencoding::decode;

/// 文件上传模块
// const BASE_DIR:&str = if cfg!(target_os = "windows") {  "D:\\tmp" }else{ "/tmp" };

// 接口上传文件
pub async fn upload_api(mut payload: Multipart, request: HttpRequest) -> Result<HttpResponse, Error> {
    let headers = request.headers();
    println!("upload_file_api::headers=>{:?}",headers);
    let (auth_result,msg) = auth_util::auth(headers);
    if !auth_result{
        println!("upload_file_api::验证[不]通过:{}",msg);
        return Ok(HttpResponse::Ok().json(ResultBuild::<i8>::fail_with_msg(msg)));
    }

    let base_dir = std::env::var("BASE_DIR").unwrap();
    let path = headers.get("path");
    if path.is_none(){
        return Ok(HttpResponse::Ok().json(ResultBuild::<i8>::fail_with_msg("参数为空[path]")));
    }
    let path = decode(path.unwrap().to_str().unwrap()).unwrap();
    // let path = path.unwrap().to_str().unwrap();
    if path.contains(".."){
        return Ok(HttpResponse::Ok().json(ResultBuild::<i8>::fail_with_msg("非法的目录地址")));
    }

    let perfix_path = format!("{}/{}",base_dir,path);
    // create directory
    println!("upload_file_api::perfix_path:{}",perfix_path);
    fs::create_dir_all(&perfix_path)?;

    let mut user_file_list:Vec<String> = Vec::new();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let content_name = content_type.get_name();
        if content_name.is_none(){
            return Ok(HttpResponse::Ok().json(ResultBuild::<String>::fail_with_msg("KEY缺失[1]")));
        }
        let filename = content_type.get_filename();
        if filename.is_none(){
            return Ok(HttpResponse::Ok().json(ResultBuild::<String>::fail_with_msg("文件名缺失[2]")));
        }
        let filename = filename.unwrap();
        if filename.contains("..")
            || filename.contains("/")
            || filename.contains("\\"){
            return Ok(HttpResponse::Ok().json(ResultBuild::<i8>::fail_with_msg("文件名非法[..、/、\\]")));
        }

        let file_path = format!("{}/{}", perfix_path, sanitize_filename::sanitize(&filename));
        user_file_list.push(format!("{}/{}", path, &filename));

        let mut f = web::block(|| std::fs::File::create(file_path)).await.unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    println!("user_file_list:{:?}",user_file_list);
    Ok(HttpResponse::Ok().json(ResultBuild::success_with_data(user_file_list)))
}
