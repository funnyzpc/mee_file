use actix_web::{HttpResponse, web};
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use crate::structs::result_build::ResultBuild;

// 创建目录
pub async fn create_dir(params:web::Form<HashMap<String,String>>/*,req:HttpRequest*/) -> HttpResponse {
    println!("create_dir=>{:?}",params);
    // 校验权限
    // let username =  UserToken::get_username(req);
    // if username.is_err() || !"admin".eq(username.unwrap().as_str()){
    //     return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("非管理员无法操作,请联系管理员!"));
    // }

    // 校验参数
    if params.is_empty() || params.get("file_dir").is_none() || params.get("create_dir").is_none(){
        return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("参数缺失[file_dir or create_dir]"));
    }
    let file_dir = params.get("file_dir").unwrap();
    let create_dir = params.get("create_dir").unwrap();
    if create_dir.contains("..") || create_dir.contains("/") || create_dir.contains("\\"){
        return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("目录名非法[不可包含.\\/..]"));
    }
    let base_dir = std::env::var("BASE_DIR").unwrap();

    // 校验目录 (比较删除目录绝对地址是否以挂目录绝对地址开始)
    let base_dir_path = Path::new(&base_dir).canonicalize().unwrap().into_os_string().into_string().unwrap();
    let create_dir_path_str = format!("{}/{}/{}",base_dir,file_dir,create_dir);
    println!("create_dir_path_str=>{}",create_dir_path_str);
    let create_dir_path = Path::new(&create_dir_path_str);
    if create_dir_path.exists(){
        return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("目录已存在,请检查~"));
    }
    // 判断上一级目录是否匹配(因为目标目录未创建)
    // let create_dir_parent_str = format!("{}/{}",base_dir,file_dir);
    // let create_dir_parent = Path::new(&create_dir_parent_str);
    let create_dir_parent = Path::new(&create_dir_path_str).parent().unwrap();
    let create_dir_parent_absolute_path = create_dir_parent.canonicalize().unwrap().into_os_string().into_string().unwrap();
    if !create_dir_parent_absolute_path.starts_with(&base_dir_path){
        return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("非法创建!!!"));
    }

    // 执行创建
    if fs::create_dir(create_dir_path).is_ok(){
        // 响应用户
        return HttpResponse::Ok().json(ResultBuild::<&str>::success());
    }
    // 响应用户
    return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("创建目录失败"));

}
