use actix_web::{HttpResponse, web};
use handlebars::Handlebars;
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use crate::structs::dir_item::DirItem;
use std::fs::DirEntry;
use chrono::{DateTime, Local};
use std::ops::Index;

const LIST_HTML: &str = include_str!("../html/list.html");


// 用户文件列表
pub async fn list(hb: web::Data<Handlebars<'_>>,params:web::Query<HashMap<String,String>>) -> HttpResponse {
    // println!("mee_file => list:{:?}",params);
    let  base_dir = std::env::var("BASE_DIR").unwrap();

    // 查找参数目录
    let file_dir = params.get("file_dir");
    if file_dir.is_some() /*&& !file_dir.unwrap().contains("..")*/{
        let  file_dir =  file_dir.unwrap();
        let full_dir_path = format!("{}/{}",base_dir,file_dir);
        let path_object = Path::new(&full_dir_path);
        if path_object.exists() && path_object.is_dir() {
            // 校验目录,防止目录穿越
            let absolute_path = path_object.canonicalize().unwrap().into_os_string().into_string().unwrap();
            let base_dir_path = Path::new(&base_dir).canonicalize().unwrap().into_os_string().into_string().unwrap();
            // println!("path_object:{:?},base_dir:{}",absolute_path,&base_dir_path);// 绝对路径
            if !absolute_path.starts_with(&base_dir_path) || absolute_path.eq(&base_dir_path){
                return list_base_dir(hb,&base_dir).await;
            }
            let mut dir_data_list:Vec<DirItem> = Vec::new();
            for entry in  fs::read_dir(path_object).unwrap(){
                let file_entry:DirEntry = entry.unwrap();
                let date: DateTime<Local> = file_entry.metadata().unwrap().modified().unwrap().into();
                let file_size:u64 = file_entry.metadata().unwrap().len()/1024;
                let dir_item = DirItem{
                    date:date.format("%Y-%m-%d %H:%M:%S").to_string(),
                    file_name:file_entry.file_name().into_string().unwrap(),
                    file_dir:file_dir.to_owned(),
                    is_dir: file_entry.file_type().unwrap().is_dir(),
                    file_size:if file_size>1024{file_size/1024}else{file_size},
                    file_size_unit:if file_size>1024{"MB".to_owned()}else{"KB".to_owned()},
                };
                dir_data_list.push(dir_item);
            }
            let file_dir = absolute_path.index(base_dir_path.len()+1..);
            // println!("file_dir:{},dir_data_list:{:?}",file_dir.replace("\\","/"),dir_data_list);
            let context_path = std::env::var("CONTEXT_PATH").unwrap();
            let data_model = json!({"context_path":context_path,"file_dir":file_dir.replace("\\","/"),"file_list":dir_data_list});
            let result_html = hb.render_template(LIST_HTML,&data_model).unwrap_or(String::from("<p>获取目录失败</p>"));
            // 返回
            return HttpResponse::Ok().content_type("text/html; charset=utf-8").body(result_html);
        }
    }
    return list_base_dir(hb,&base_dir).await;

}

async fn list_base_dir(hb: web::Data<Handlebars<'_>>,base_dir:&String) -> HttpResponse {
    let path_object = Path::new(&base_dir);
    let mut dir_data_list:Vec<DirItem> = Vec::new();
    for entry in  fs::read_dir(path_object).unwrap(){
        let file_entry:DirEntry = entry.unwrap();
        let date: DateTime<Local> = file_entry.metadata().unwrap().modified().unwrap().into();
        let file_size:u64 = file_entry.metadata().unwrap().len()/1024;
        let dir_item = DirItem{
            date:date.format("%Y-%m-%d %H:%M:%S").to_string(),
            file_name:file_entry.file_name().into_string().unwrap(),
            file_dir:".".to_owned(),
            is_dir: file_entry.file_type().unwrap().is_dir(),
            file_size:if file_size>1024{file_size/1024}else{file_size},
            file_size_unit:if file_size>1024{"MB".to_owned()}else{"KB".to_owned()},
        };
        dir_data_list.push(dir_item);
    }
    let context_path = std::env::var("CONTEXT_PATH").unwrap();
    let data_model = json!({"context_path":context_path,"file_dir":".","file_list":dir_data_list});
    let result_html = hb.render_template(LIST_HTML,&data_model).unwrap_or(String::from("<p>获取目录失败</p>"));
    // 返回
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(result_html)
}