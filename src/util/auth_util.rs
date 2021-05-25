use hmac_sha512::HMAC;
use actix_web::http::{HeaderValue, HeaderMap};

// 认证密钥
// const COMMON_KEY:&str= "|1a59deaf6^%$#@5b14fa09a*&^3a27c749b6)f9(23|";

// 权限认证
pub fn auth(headers:&HeaderMap) ->(bool, &'static str){
    //let headers = request.headers();
    let timestamp = headers.get("timestamp");
    let access_token = headers.get("access_token");
    let from = headers.get("from");
    if timestamp==None || access_token==None || from==None{
        return (false,"参数缺失[timestamp、from、access_token]");
    }
    // 加密处理
    // 8f44e9837aad5ee29ac1054f3559631d042a8de3ae2dcee03d6cc34e8af76a9f5ed99d3968410eca6802c9b9b4f0ee52530cb8e8e0433652f32d21c874226409
    let timestamp = timestamp.unwrap().to_str().unwrap();
    let from = from.unwrap().to_str().unwrap();

    // check time
    let u_timestamp = timestamp.parse::<i64>().expect("auth::timestamp::参数转换出错");
    let i_timestamp = chrono::offset::Local::now().timestamp();
    if i_timestamp-u_timestamp > 60 || i_timestamp-u_timestamp < -60{
        println!("u_timestamp:{},i_timestamp:{}",u_timestamp,i_timestamp);
        return (false,"auth::timestamp::超时");
    }

    let (result,access_token) = filter_token(access_token);
    if !result{
        return (false,"auth::access_token有误");
    }
    let input_str = format!("{}||{}",timestamp,from);

    let common_key = std::env::var("COMMON_KEY").expect("配置不存在::COMMON_KEY");
    let enc_base64 = HMAC::mac(input_str,common_key);

    // // 加密后的[u8;64]
    // println!("h:\n{:?}",h);
    // let dec_str = format!("{:x?}", h.to_vec());
    // // 格式化为16进制字符串
    // println!("dec_str:\n{}", dec_str);

    if enc_base64.eq(access_token.as_slice()){
        (true,"success")
    }else{
        (false,"验证错误")
    }
}

fn filter_token(access_token:Option<&HeaderValue>)->(bool,Vec<u8>){
    let access_token = hex::decode(access_token.unwrap().to_str().unwrap());
    match access_token {
        Err(_)=>return (false,Vec::new()),
        Ok(access_token)=>return (true,access_token),
    };
}