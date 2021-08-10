
use actix_service::{Service, Transform};
use actix_web::{Error, HttpResponse, dev::{ServiceRequest, ServiceResponse},HttpMessage};
use futures::{
    future::{ok, Ready},
    Future,
};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use actix_web::cookie::Cookie;
use crate::structs::user_token::UserToken;

pub struct Authentication;

impl<S, B> Transform<S> for Authentication
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}


pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticationMiddleware<S>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let uri_path = req.path();
        let context_path = format!("{}/login",std::env::var("CONTEXT_PATH").unwrap());
        println!("context_path===>{}",&context_path);// /mee_file/login
        println!("call===>{}",&uri_path);

        if uri_path.eq(&context_path){
        //if *uri_path == &context_path.as_str(){
            println!("call===>001");
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }
        println!("call===>002");

        let  auth_token:Option<Cookie> = req.cookie("Authorization");
        if let Some(auth_header) = auth_token {
            let auth_str = auth_header.value();
            if auth_str.starts_with("bearer") || auth_str.starts_with("Bearer") {
                let token = auth_str[6..auth_str.len()].trim();
                println!("before::decode_token");
                if let Ok(token_data) = UserToken::decode_token(token.to_string()) {
                    println!("after::decode_token");
                    if UserToken::verify_token(&token_data).is_ok() {
                        println!("after::verify_token");
                        let fut = self.service.call(req);
                        return Box::pin(async move {
                            let res = fut.await?;
                            Ok(res)
                        });
                    }
                }
                println!("验证不通过(Invalid token):{:?}",&auth_header);
            }
        }

        // 验证不通过
        return Box::pin(async move {
            Ok(req.into_response(
                HttpResponse::MovedPermanently()
                    //TODO .del_cookie()
                    .header("location",context_path)
                    .finish()
                    .into_body(),
            ))
        });
    }
}