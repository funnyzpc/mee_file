use actix_web::{web, HttpResponse, HttpRequest};
use std::collections::HashMap;
use crate::structs::result_build::ResultBuild;
use crate::structs::user_token::UserToken;
use actix_web::cookie::Cookie;
use actix_web::cookie::SameSite;
use actix_web::http::header;

/**
   @auther shadow
   @description 登录处理
**/

// 登录处理
pub async fn login(params: web::Form<HashMap<String,String>>) -> HttpResponse {
    println!("===>login");
    let username = params.get("username");
    let password = params.get("password");
    let context_path = std::env::var("CONTEXT_PATH").unwrap();

    if username.is_none() || password.is_none(){
        println!("用户或密码为空:{:?}->{:?}",username,password);
        return HttpResponse::MovedPermanently().insert_header(("Location",format!("{}/{}",context_path,"/login"))).finish();
    }
    // 从配置获取用户并检查
    let password_cfg =  std::env::var(format!("U.{}",&username.unwrap()));

    match password_cfg {
        Err(error)=>{
            println!("用户不存在或密码错误:{:?}->{:?},{}",username,password,error);
            return HttpResponse::MovedPermanently().insert_header(("Location",format!("{}/{}",context_path,"/login"))).finish();
        },
        Ok(result)=>{
            if !result.eq(password.unwrap()){
                println!("用户密码不匹配:{:?}->{:?}",username,password);
                return HttpResponse::MovedPermanently()
                    .insert_header(("location",format!("{}/{}",context_path,"/login")))
                    .json(ResultBuild::<i8>::fail_with_msg("用户密码不匹配"));
            }
        }
    }
    let token = UserToken::gen_token(username.unwrap());

    let r = chrono::offset::Local::now().timestamp();
    // let one_hour = Duration::minutes(60);
    let one_hour = actix_web::cookie::time::Duration::minutes(60);

    let mut cookie = Cookie::new("Authorization",format!("bearer {}",token));
    //cookie.set_path("/");
    cookie.set_path(&context_path);
    cookie.set_http_only(true);
    //cookie.set_expires(None);
    cookie.set_max_age(one_hour);
    cookie.set_same_site(SameSite::Strict);
    HttpResponse::MovedPermanently().cookie(cookie).insert_header(("Location",format!("{}?r={}",context_path,r))).finish()
}


/**
 @description 登录页面
 **/
const LOGIN_HTML: &str = include_str!("../html/login.html");
pub async fn login_index(req:HttpRequest) -> HttpResponse {
    //println!("===>login_index");
    // check auth
    let  auth_token:Option<Cookie> = req.cookie("Authorization");
    if let Some(auth_header) = auth_token{
        let auth_str = auth_header.value();
        if auth_str.starts_with("bearer") || auth_str.starts_with("Bearer") {
            let token = auth_str[6..auth_str.len()].trim();
            if let Ok(token_data) = UserToken::decode_token(token.to_string()) {
                if UserToken::verify_token(&token_data).is_ok() {
                    println!("cookie有效...:{:?}",req.headers());
                    let context_path = std::env::var("CONTEXT_PATH").unwrap();
                    let r = chrono::offset::Local::now().timestamp();
                    //return HttpResponse::MovedPermanently().header("Location",format!("{}/list?r={}",context_path,r)).finish();
                    return HttpResponse::MovedPermanently().insert_header(("Location",format!("{}?r={}",context_path,r))).finish();
                }
            }
        }
    }
    HttpResponse::Ok()
        .insert_header((header::CACHE_CONTROL, "no-cache, no-store, must-revalidate"))// HTTP 1.1.
        .insert_header(("Pragma", "no-cache")) // HTTP 1.0.
        .insert_header(("Expires", "0"))
        .content_type("text/html; charset=utf-8")
        .body(LOGIN_HTML)
}
