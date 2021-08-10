use serde::{Deserialize, Serialize};

use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, Algorithm};
use jsonwebtoken::{DecodingKey, TokenData, Validation};

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

}