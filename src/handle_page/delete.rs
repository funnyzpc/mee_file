use actix_web::{HttpResponse, web, HttpRequest};
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use crate::structs::user_token::UserToken;
use crate::structs::result_build::ResultBuild;

// 删除目录或文件
pub async fn delete(params:web::Form<HashMap<String,String>>,req:HttpRequest) -> HttpResponse {
    println!("....=>delete:{:?}",params);
    // 校验权限
    let username =  UserToken::get_username(req);
    if username.is_err() || !"admin".eq(username.unwrap().as_str()){
        return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("非管理员无法操作,请联系管理员!"));
    }

    // 校验参数
    if params.is_empty() || params.get("is_dir").is_none() || params.get("del_path").is_none(){
        return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("参数缺失[is_dir、del_path]"));
    }
    let is_dir = params.get("is_dir").unwrap();
    let del_path = params.get("del_path").unwrap();
    let base_dir = std::env::var("BASE_DIR").unwrap();

    // 校验目录 (比较删除目录绝对地址是否以挂目录绝对地址开始)
    let base_dir_path = Path::new(&base_dir).canonicalize().unwrap().into_os_string().into_string().unwrap();
    let delete_path_str = format!("{}/{}",base_dir,del_path);
    let delete_path = Path::new(&delete_path_str);
    if !delete_path.exists(){
        return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("删除对象不存在~"));
    }
    let delete_absolute_path = delete_path.canonicalize().unwrap().into_os_string().into_string().unwrap();
    if !delete_absolute_path.starts_with(&base_dir_path) || delete_absolute_path.eq(&base_dir_path){
        return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("非法删除!!!"));
    }

    // 执行删除
    if "0".eq(is_dir){
        // 删除文件
        if fs::remove_file(delete_path).is_err(){
            println!("删除文件异常了:{:?}",delete_path);
        }
    }
    if "1".eq(is_dir){
        // 删除目录(判断目录下是否还有文件)
        if fs::read_dir(delete_path).unwrap().count()>0{
            return HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg("目录非空，请清空目录后再行删除~"));
        }
        if fs::remove_dir(delete_path).is_err(){
            println!("删除目录异常了:{:?}",delete_path);
        }
    }
    // 响应用户
    return HttpResponse::Ok().json(ResultBuild::<&str>::success());

}
