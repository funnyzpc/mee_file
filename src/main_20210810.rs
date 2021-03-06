
use actix_web::{web, App, HttpServer};
use actix_service::Service;
use futures::FutureExt;


use actix_files::Files;
use dotenv::dotenv;
use actix_web::middleware::Logger;
use env_logger::Env;

// #[macro_use]
// extern crate actix_web;

#[macro_use]
extern crate log;

// #[path= "./handle_page/auth_api"] mod auth_handle;
mod middleware;
mod structs;
mod util;
mod handle;
use crate::handle::{auth_handle,index_handle,upload_handle,download_handle,list_handle,login_handle};


// const BASE_DIR:&str = if cfg!(target_os = "windows") {  "D:\\tmp" }else{ "/tmp" };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载配置
    dotenv().ok();
    // 使用日志
    std::env::set_var("RUST_LOG", "RUST_LOG=debug,actix_server=debug,actix_web=debug");

    // base 文件目录(必须在.env文件配置)
    let base_dir = std::env::var("BASE_DIR").expect("配置不存在::BASE_DIR");
    let server_port = std::env::var("SERVER_PORT").expect("配置不存在::SERVER_PORT");
    let context_path = std::env::var("CONTEXT_PATH").expect("配置不存在::CONTEXT_PATH");

    let date_time = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S.%f").to_string();
    println!("{} server start at http://127.0.0.1:{}{}",date_time,server_port,&context_path);
    std::fs::create_dir_all(base_dir).unwrap();
    // log init
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    // env_logger::init();

    // 路由
    // format_args!()
    HttpServer::new(move|| {
        let base_dir = std::env::var("BASE_DIR").expect("配置不存在::BASE_DIR");
        let context_path = std::env::var("CONTEXT_PATH").expect("配置不存在::CONTEXT_PATH");

        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            // api接口部分

            // 后台部分
            .wrap(middleware::auth_middleware::Authentication)
            .wrap_fn(|request,service|{service.call(request).map(|response|response)  })
            .route(&context_path,web::get().to(index_handle::index))

            //.route(&format!("{}/{}",context_path,"upload_api"),web::post().to(upload_handle::upload_api))
            .route(&format!("{}/{}",context_path,"api/auth"),web::get().to(auth_handle::auth))
            .route(&format!("{}/{}",context_path,"api/upload"),web::post().to(upload_handle::upload))
            .route(&format!("{}/{}",context_path,"api/list"),web::get().to(list_handle::list_api))
            .route(&format!("{}/{}",context_path,"api/download"),web::get().to(download_handle::download))

            .route(&format!("{}/{}",context_path,"login"),web::get().to(login_handle::login_index))
            .route(&format!("{}/{}",context_path,"login"),web::post().to(login_handle::login))
            // 遍历所有文件
            .service(Files::new(&format!("{}/{}",context_path,"list"), base_dir).show_files_listing())
        // 定位所有文件
        // .handle_page(Files::new("/index_file", "./index_file").index_file("*"))
    })
        .bind(format!("0.0.0.0:{}",server_port))?
        .run()
        .await
}