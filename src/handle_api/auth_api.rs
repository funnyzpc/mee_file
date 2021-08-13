use actix_web::{HttpRequest, HttpResponse};

use crate::structs::result_build::ResultBuild;
use crate::util::auth_util;

/// 验证模块
pub async fn auth(request:HttpRequest)->HttpResponse{
    let headers = request.headers();
    println!("===>headers:{:?}",headers);
    let (result,msg) = auth_util::auth(headers);
    if result{
        println!("验证通过:{}",&msg);
        HttpResponse::Ok().json(ResultBuild::<&str>::success())
    }else{
        println!("验证[不]通过:{}",msg);
        HttpResponse::Ok().json(ResultBuild::<&str>::fail_with_msg(msg))
    }
}