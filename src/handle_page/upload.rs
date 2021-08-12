use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse, HttpRequest};
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

use crate::structs::result_build::ResultBuild;
use urlencoding::decode;

// 手动上传文件
pub async fn upload(mut payload: Multipart, req:HttpRequest) -> Result<HttpResponse, Error> {
    println!("upload->headers:{:?}",req.headers());
    let base_dir = std::env::var("BASE_DIR").unwrap();
    let  file_dir = req.headers().get("file_dir");//.unwrap().to_str().unwrap();
    if file_dir.is_none(){
        return Ok(HttpResponse::Ok().json(ResultBuild::<u8>::fail_with_msg("字段名缺失[file_dir]")));
    }
    let file_dir = decode(file_dir.unwrap().to_str().unwrap()).unwrap();
    // println!("upload=>file_dir:{:?}",file_dir);
    // prefix path
    let perfix_path = format!("{}/{}",base_dir,file_dir);

    let mut file_list:Vec<String> = Vec::new();
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let content_name = content_type.get_name();
        println!("content_name=>{:?}",content_name);
        if content_name.is_none(){
            return Ok(HttpResponse::Ok().json(ResultBuild::<u8>::fail_with_msg("字段名缺失")));
        }
        let filename = content_type.get_filename();
        if filename.is_none(){
            return Ok(HttpResponse::Ok().json(ResultBuild::<String>::fail_with_msg("文件名缺失[2]")));
        }
        let filename = filename.unwrap();
        let file_path = format!("{}/{}", perfix_path, sanitize_filename::sanitize(&filename));
        println!("file_path=>{}",file_path);
        file_list.push(format!("{}/{}", file_dir, &filename));
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(file_path)).await.unwrap();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
        return Ok(HttpResponse::Ok().json(ResultBuild::success_with_data(file_list)));
    }
    println!("file_list:{:?}",file_list);
    Ok(HttpResponse::Ok().json(ResultBuild::<i8>::fail_with_msg("上传失败")))
}

