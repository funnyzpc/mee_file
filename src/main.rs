
use actix_web::{web, App, HttpServer};
use actix_service::Service;
use futures::FutureExt;


// use actix_files::Files;
use dotenv::dotenv;
use actix_web::middleware::Logger;
use env_logger::Env;

// #[macro_use]
// extern crate actix_web;

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

// #[path= "./handle_page/auth_api"] mod auth_handle;
mod middleware;
mod structs;
mod util;
mod handle_api;
mod handle_page;
use crate::handle_api::{auth_api,upload_api,download_api,list_api};
use crate::handle_page::{index, login, list, download, upload};
use handlebars::Handlebars;


// const BASE_DIR:&str = if cfg!(target_os = "windows") {  "D:\\tmp" }else{ "/tmp" };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载配置
    dotenv().ok();
    // 使用日志
    std::env::set_var("RUST_LOG", "RUST_LOG=debug,actix_server=debug,actix_web=debug");
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    // base 文件目录(必须在.env文件配置)
    let base_dir = std::env::var("BASE_DIR").expect("配置不存在::BASE_DIR");
    let server_port = std::env::var("SERVER_PORT").expect("配置不存在::SERVER_PORT");
    let context_path = std::env::var("CONTEXT_PATH").expect("配置不存在::CONTEXT_PATH");

    let date_time = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S.%f").to_string();
    println!("{} server start at http://127.0.0.1:{}{}",date_time,server_port,&context_path);
    std::fs::create_dir_all(base_dir).unwrap();

    // handlebar
    let hb_ref = web::Data::new(Handlebars::new());

    // route
    HttpServer::new(move|| {
        // let base_dir = std::env::var("BASE_DIR").expect("配置不存在::BASE_DIR");
        let context_path = std::env::var("CONTEXT_PATH").expect("配置不存在::CONTEXT_PATH");

        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            // api接口
            .service(web::scope(&format!("{}/api",context_path))
                .route("auth",web::get().to(auth_api::auth))
                .route("upload",web::get().to(upload_api::upload_api))
                .route("/list",web::get().to(list_api::list_api))
                .route("download",web::get().to(download_api::download_api))
            )
            // 后台
            .service(web::scope(context_path.as_str())
                .app_data(hb_ref.clone())
                .app_data(web::PayloadConfig::new(1024*1024*256))// 256MB
                .wrap(middleware::auth_middleware::Authentication)
                .wrap_fn(|request,service|{service.call(request).map(|response|response)  })
                .route("",web::get().to(index::index))
                .route("login",web::get().to(login::login_index))
                .route("login",web::post().to(login::login))
                .route("list",web::get().to(list::list))
                .route("download",web::get().to(download::download))
                .route("upload",web::post().to(upload::upload))
            )
            // .service(Files::new(&format!("{}/{}",context_path,"list"), base_dir).show_files_listing())
        // 定位所有文件
        // .handle_page(Files::new("/index_file", "./index_file").index_file("*"))
    })
        .bind(format!("0.0.0.0:{}",server_port))?
        .run()
        .await
}