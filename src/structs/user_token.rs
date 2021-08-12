use serde::{Deserialize, Serialize};

use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, Algorithm};
use jsonwebtoken::{DecodingKey, TokenData, Validation};
use actix_web::{HttpRequest, HttpMessage};
use actix_web::cookie::Cookie;

// token加密方式
const ALGORITHM:Algorithm = Algorithm::HS512;
// token加密密钥
const KEY:&[u8]=  b"+Q3%<.>7W079c@#&Ct#11!g4e+*7)(b1e|aUrKjIxZXxhOQ==";
// token 有效期
const ONE_HOUR: i64 = 60 * 60;

#[derive(Debug,Serialize, Deserialize)]
pub struct UserToken {
    // issued at
    pub iat: i64, // 发布时间
    pub iss: String, // 发布人
    // expiration
    pub exp: i64, // 失效时间
    // data
    pub username: String,
}

impl UserToken {

    // token生成
    pub fn gen_token(username:&String) -> String {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let payload = UserToken {
            iat: now,
            iss: "sys".to_owned(),
            exp: now + ONE_HOUR,
            username: username.clone(),
            //login_session: login.login_session.clone(),
        };
        jsonwebtoken::encode(&Header::new(ALGORITHM), &payload, &EncodingKey::from_secret(&KEY)).unwrap()
    }

    // token验证 (自带超时验证)
    pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
        jsonwebtoken::decode::<UserToken>(&token, &DecodingKey::from_secret(&KEY),  &Validation::new(ALGORITHM))
    }

    // 验证token (目前仅验证用户名)
    pub fn verify_token(token_data: &TokenData<UserToken>) -> Result<(),()> {
        let username = &token_data.claims.username;
        if let Err(error) = std::env::var(format!("U.{}",username)){
            println!("用户名校验失败...:{},{}",username,error);
            return Err(());
        }
        return Ok(());
    }

    // 获取用户名
    pub fn get_username(req:HttpRequest)->Result<String,String>{
        let  auth_token:Option<Cookie> = req.cookie("Authorization");
        if let Some(auth_header) = auth_token{
            let auth_str = auth_header.value();
            if auth_str.starts_with("bearer") || auth_str.starts_with("Bearer") {
                let token = auth_str[6..auth_str.len()].trim();
                if let Ok(token_data) = UserToken::decode_token(token.to_string()) {
                    if UserToken::verify_token(&token_data).is_ok() {
                        return Ok(token_data.claims.username);
                    }
                }
            }
        }
        Err("未能获取到用户名".to_owned())
    }

}