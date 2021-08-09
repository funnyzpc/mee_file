use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse, HttpRequest};
use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use std::fs;

/*#[path= "../util/auth_util.rs"] mod auth_util;
#[path= "../structs/mod.rs"] mod structs;*/
use crate::structs::result_build::ResultBuild;
use crate::util::auth_util;

/// 文件上传模块

// const BASE_DIR:&str = if cfg!(target_os = "windows") {  "D:\\tmp" }else{ "/tmp" };

// 用户手动上传文件
pub async fn upload(mut payload: Multipart, request: HttpRequest) -> Result<HttpResponse, Error> {
    let headers = request.headers();
    println!("upload_file::headers=>{:?}",headers);
    let (auth_result,msg) = auth_util::auth(headers);
    if !auth_result{
        println!("验证[不]通过:{}",msg);
        return Ok(HttpResponse::Ok().json(ResultBuild::<i8>::fail_with_msg(msg)));
    }

    let base_dir = std::env::var("BASE_DIR").unwrap();
    let path = headers.get("path");
    let path = path.unwrap().to_str().unwrap();
    let date_path = chrono::offset::Local::now().format("%Y/%m/%d").to_string();
    // prefix path
    let perfix_path = format!("{}/{}/{}",base_dir,path,date_path);
    // create directory
    fs::create_dir_all(&perfix_path)?;

    let mut user_file_list:Vec<String> = Vec::new();
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let content_name = content_type.get_name();
        if content_name==Some(""){
            return Ok(HttpResponse::Ok().json(ResultBuild::<i8>::fail_with_msg("KEY缺失[1]")));
        }
        println!("content_name:{}",content_name.unwrap());
        let filename = content_type.get_filename().unwrap();
        println!("filename:{},content_name:{}",filename,content_name.unwrap());
        if filename==""{
            return Ok(HttpResponse::Ok().json(ResultBuild::<String>::fail_with_msg("文件名缺失[2]")));
        }
        let file_path = format!("{}/{}", perfix_path, sanitize_filename::sanitize(&filename));

        let file_path_item = format!("{}/{}/{}", path,date_path, &filename);

        // println!("file_path_item:{}",&file_path_item);
        user_file_list.push(file_path_item);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(file_path)).await.unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    println!("user_file_list:{:?}",user_file_list);
    // let mut data = result_build::success();
    // data.insert(String::from("file_list"), serde_json::to_string(&data).unwrap());
    // Ok(HttpResponse::Ok().json(data))
    Ok(HttpResponse::Ok().json(ResultBuild::success_with_data(user_file_list)))
}


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
    let path = path.unwrap().to_str().unwrap();
    let perfix_path = format!("{}/{}",base_dir,path);
    // create directory
    println!("upload_file_api::perfix_path:{}",perfix_path);
    fs::create_dir_all(&perfix_path)?;

    let mut user_file_list:Vec<String> = Vec::new();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let content_name = content_type.get_name();
        if content_name==Some(""){
            return Ok(HttpResponse::Ok().json(ResultBuild::<String>::fail_with_msg("KEY缺失[1]")));
        }
        let filename = content_type.get_filename().unwrap();
        if filename==""{
            return Ok(HttpResponse::Ok().json(ResultBuild::<String>::fail_with_msg("文件名缺失[2]")));
        }
        let file_path = format!("{}/{}", perfix_path, sanitize_filename::sanitize(&filename));

        let file_path_item = format!("{}/{}", path, &filename);

        user_file_list.push(file_path_item);

        let mut f = web::block(|| std::fs::File::create(file_path)).await.unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    println!("user_file_list:{:?}",user_file_list);
    Ok(HttpResponse::Ok().json(ResultBuild::success_with_data(user_file_list)))
}
