
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
        // println!("call===>{}",&req.uri());
        // let mut auth_pass: bool = false;

        // Bypass some account routes
        // let headers = req.headers_mut();
        // headers.append(HeaderName::from_static("content-length"),HeaderValue::from_static("true"));

        let ignore_path = std::env::var("IGNORE_PATH").unwrap();
        let ignore_path:Vec<&str> = ignore_path.split(";").collect();
        let request_path = req.path();
        // 忽略验证的资源直接通过
        if ignore_path.iter().any(|&item| item.eq(request_path)){
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        // for ignore_route in ignore_path.iter() {
        //     if req.path().eq(*ignore_route) {
        //         auth_pass = true;
        //         break;
        //     }
        // }

        let  auth_token:Option<Cookie> = req.cookie("Authorization");
        if let Some(auth_header) = auth_token {
            let auth_str = auth_header.value();
            if auth_str.starts_with("bearer") || auth_str.starts_with("Bearer") {
                let token = auth_str[6..auth_str.len()].trim();
                if let Ok(token_data) = UserToken::decode_token(token.to_string()) {
                    if UserToken::verify_token(&token_data).is_ok() {
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
        let context_path = std::env::var("CONTEXT_PATH").unwrap();
        return Box::pin(async move {
            Ok(req.into_response(
                HttpResponse::MovedPermanently()
                    .header("location",format!("{}/{}",context_path,"/login"))
                    .finish()
                    .into_body(),
            ))
        });
    }
}