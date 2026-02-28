use std::future::{Ready, ready};

use actix_web::{
    Error, HttpResponse,
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform}
};
use actix_web::cookie::Cookie;
use futures_util::future::LocalBoxFuture;
use futures_util::{FutureExt, TryFutureExt};
use crate::structs::user_token::UserToken;

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}
pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let uri_path = req.path();
        let context_path = format!("{}/login",std::env::var("CONTEXT_PATH").unwrap());
        //println!("call===>{}",&uri_path);

        if uri_path.eq(&context_path){
            //if *uri_path == &context_path.as_str(){
            println!("call===>001");
            // let fut = self.service.call(req);
            // return Box::pin(async move {
            //     let res = fut.await?;
            //     Ok(res)
            // });
            return self.service
                .call(req)
                .map_ok(ServiceResponse::map_into_left_body)
                .boxed_local();
        }

        // println!("call===>002");

        let  auth_token:Option<Cookie> = req.cookie("Authorization");
        if let Some(auth_header) = auth_token {
            let auth_str = auth_header.value();
            if auth_str.starts_with("bearer") || auth_str.starts_with("Bearer") {
                let token = auth_str[6..auth_str.len()].trim();
                // println!("before::decode_token");
                if let Ok(token_data) = UserToken::decode_token(token.to_string()) {
                    // println!("after::decode_token");
                    if UserToken::verify_token(&token_data).is_ok() {
                        // println!("after::verify_token");
                        // let fut = self.service.call(req);
                        // return Box::pin(async move {
                        //     let res = fut.await?;
                        //     Ok(res)
                        // });
                        return self.service
                            .call(req)
                            .map_ok(ServiceResponse::map_into_left_body)
                            .boxed_local();
                    }
                }
                println!("验证不通过(Invalid token):{:?}",&auth_header);
            }
        }

        // return Box::pin(async {
        //     Ok(req.into_response(
        //         HttpResponse::TooManyRequests()
        //             .finish()
        //             .map_into_right_body(),
        //     ))
        // });

        // 验证不通过
        return Box::pin(async move {
            let r = chrono::offset::Local::now().timestamp();
            Ok(req.into_response(
                HttpResponse::MovedPermanently()
                    .insert_header(("Location",format!("{}?r={}",context_path,r)))
                    // 一定要有,否则GET会被缓存
                    .insert_header(("Cache-Control", "no-cache, no-store, must-revalidate"))// HTTP 1.1.
                    .insert_header(("Pragma", "no-cache")) // HTTP 1.0.
                    .insert_header(("Expires", "0"))
                    .finish()
                    .map_into_right_body(),
            ))
        });
    }
}