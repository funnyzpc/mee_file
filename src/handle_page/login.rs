use actix_web::{web, HttpResponse};
use std::collections::HashMap;
use crate::structs::result_build::ResultBuild;
use crate::structs::user_token::UserToken;
use actix_web::cookie::Cookie;
use time::Duration;

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

    if None == username || None == password{
        println!("用户或密码为空:{:?}->{:?}",username,password);
        return HttpResponse::MovedPermanently().header("location",format!("{}/{}",context_path,"/login")).finish();
    }
    // 从配置获取用户并检查
    let password_cfg =  std::env::var(format!("U.{}",&username.unwrap()));

    match password_cfg {
        Err(error)=>{
            println!("用户不存在或密码错误:{:?}->{:?},{}",username,password,error);
            return HttpResponse::MovedPermanently().header("location",format!("{}/{}",context_path,"/login")).finish();
        },
        Ok(result)=>{
            if !result.eq(password.unwrap()){
                println!("用户密码不匹配:{:?}->{:?}",username,password);
                return HttpResponse::MovedPermanently()
                    .header("location",format!("{}/{}",context_path,"/login"))
                    .json(ResultBuild::<i8>::fail_with_msg("用户密码不匹配"));
            }
        }
    }
    // println!("登录成功:{:?}->{:?}",username,password);
    // get token
    let token = UserToken::gen_token(username.unwrap());
    // token to session
    // let jwt = format!("bearer {}",token);

    let one_hour = Duration::minutes(60);
    let mut cookie = Cookie::new("Authorization",format!("bearer {}",token));
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_expires(None);
    cookie.set_max_age(one_hour);
    return HttpResponse::MovedPermanently().cookie(cookie).header("location",context_path).finish();
}


/**
 @description 登录页面
 **/
pub async fn login_index() -> HttpResponse {
    println!("===>login_index");
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(INDEX_HTML)
}

const  INDEX_HTML:&str =
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <title>MEE file</title>
        <meta charset="utf-8"/>
        <meta name="viewport" content="width=device-width,initial-scale=0.8,maximum-scale=1,minimum-scale=1,user-scalable=no">
        <link rel="shortcut icon" href="${ctxPath}/static/img/favicon.ico">
        <style type="text/css">
            body{
                margin: 0px;
                background-color:#050563;
                background-size: 100%;
                font-family: "幼圆","Microsoft YaHei",simhei,SimSun;
            }
            div.login {
                width: 320px;
                margin:11% auto;
                padding: 32px 40px 40px 40px;
                background-color: #f4f4ff;
            }
            input[type="number"], input[type="password"],input[type="text"],input[type="url"],textarea {
                border-style:solid;
                border-radius:1px;
                border-color: #eef6ff;
                background-color:#eef6ff;
                outline: none;
                height:32px;
                line-height: normal;
                width: 240px;
                font-size: 16px;
            }
            input[type="submit"]{
                border-color:black;
                background-color:black;
                color:white;
                display: inline-block;
                min-width: 108px;
                padding:8px 12px 8px 12px;
                position: relative;
                text-align: center;
                white-space: nowrap;
                overflow: hidden;
                vertical-align: middle;
                text-overflow: ellipsis;
                touch-action: manipulation;
                border-style: solid;
                cursor: pointer;
                width:240px;
            }
            input[type="submit"]:active{
                padding:8px 13px 8px 13px;
                box-shadow: 0px 1px 0px #2780e3
            }
            .form-submit{right: 44px;
                text-align: right;
                bottom: 44px;
                margin-left: -2px;
                margin-right: -2px;
            }
            .form-title,.form-group,.form-submit{padding:2px 4px;margin-top:4px;}
            .form-submit{margin-top: 24px;padding:4px 2px;}
            .form-title h3{margin:8px 0px;}
            .env_flag_dev,.env_flag_test{width:120px;margin-bottom:8px;background-color:#ffffaa;}
            .env_flag_prod{width:120px;margin-bottom:8px;}
        </style>
    </head>
    <body>
    <div class="container-fluid login">
        <!-- <form method="POST" onsubmit="return chaos()"> -->
        <form method="POST">
            <div class="form-title">
                <div class="col-sm-10">
                    <h3 style="text-align:center;font-size:24px;color:#2307ad;">MEE file</h3>
            </div>
    </div>
    <div class="form-group">
        <div class="col-sm-10" style="text-align:center;">
            <input type="text" class="form-control" name="username" required placeholder="用户">
        </div>
    </div>
    <div class="form-group">
        <div  style="text-align:center;">
            <input type="password"  name="password" required placeholder="密码">
        </div>
    </div>
    <div class="form-submit">
        <div class="col-sm-10" style="text-align:center;font-weight:bold;">
            <input type="submit" value="登&nbsp;&nbsp;录">
        </div>
    </div>
    </form>
    </div>
    <script type="text/javascript">
        //let msg = "<#if msg??>${msg}</#if>";
        window.setTimeout(function(){
            if(msg){alert(msg);}
        },200);

        function chaos(){
            let pwd = document.querySelector("input[name=password]").value;
            let enc_pwd = enc(pwd);
            if(!enc_pwd){
                return false;
            }
            document.querySelector("input[name=password]").value=enc_pwd;
            return true;
        }

        function enc(pwd){
            if(!pwd || " "==pwd){
                return "";
            }
            let enc1 = random(5)+pwd+random(5);
            let enc2 = window.btoa(enc1);
            let enc3 = enc2.split("").reverse().join("");
            let enc4 = (new Date().getTime()/1000 >> 0)+"&&"+enc3;
            let enc5 = window.btoa(enc4);
            let enc6 = strToHex(enc5);
            console.log("result=>"+enc6);
            return enc6;
        }

        function strToHex(str) {
            if(str === ""){
                return "";
            }
            let hex_code = [];
            for(var i = 0; i < str.length; i++) {
                hex_code.push((str.charCodeAt(i)).toString(16));
            }
            return hex_code.join("");
        }
        const chars = 'abcdefghijklmnopqrstuvwxyz12345678ABCDEFGHJKLMNOPQRSTUVWXYZ';
        function random(len) {
            len = len || 32;
            let char_length = chars.length;
            let c = '';
            for (let i = 0; i < len; i++) {
                c += chars.charAt(Math.floor(Math.random() * char_length));
            }
            return c;
        }
    </script>
    </body>
    </html>
    "#;