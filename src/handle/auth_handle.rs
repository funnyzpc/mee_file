use actix_web::{HttpRequest, HttpResponse};

/*#[path= "../util/auth_util.rs"] mod auth_util;
#[path= "../structs/mod.rs"] mod structs;*/
use crate::structs::result_build::ResultBuild;
use crate::util::auth_util;

/// 验证模块
pub async fn auth(request:HttpRequest)->HttpResponse{
    let headers = request.headers();
    println!("===>headers:{:?}",headers);
    let (result,msg) = auth_util::auth(headers);
    if result{
        println!("验证通过:{}",&msg);
        HttpResponse::Ok().json(ResultBuild::<i32>::success())
    }else{
        println!("验证[不]通过:{}",msg);
        HttpResponse::Ok().json(ResultBuild::<i32>::fail_with_msg(msg))
    }
}

// const  COMMON_KEY:&str= "|1a59deaf6^%$#@5b14fa09a*&^3a27c749b6)f9(23|";
// pub async fn auth(request: HttpRequest) -> HttpResponse {
//     let headers = request.headers();
//     let timestamp = headers.get("timestamp");
//     let filename = headers.get("filename");
//     let access_token = headers.get("access_token");
//     if timestamp==None || filename==None || access_token==None{
//         return HttpResponse::Ok().json(result_build::fail_with_msg(&"参数缺失[filename]"));
//     }
//     // 加密处理
//     // 8f44e9837aad5ee29ac1054f3559631d042a8de3ae2dcee03d6cc34e8af76a9f5ed99d3968410eca6802c9b9b4f0ee52530cb8e8e0433652f32d21c874226409
//     let timestamp = timestamp.unwrap().to_str().unwrap();
//     let filename = filename.unwrap().to_str().unwrap();
//     let access_token = hex::decode(access_token.unwrap().to_str().unwrap()).expect("Decoding failed");
//
//     let input = format!("{}||{}",timestamp,filename);
//
//     let h = HMAC::mac(input,COMMON_KEY);
//
//     // 加密后的[u8;64]
//     println!("h:\n{:?}",h);
//     let dec_str = format!("{:x?}", h.to_vec());
//     // 格式化为16进制字符串
//     println!("dec_str:\n{}", dec_str);
//
//     if h.eq(access_token.as_slice()){
//         HttpResponse::Ok().json(result_build::success_with_msg("签名验证通过啦"))
//     }else{
//         HttpResponse::Ok().json(result_build::fail_with_msg("验证不通过"))
//     }
//
// }