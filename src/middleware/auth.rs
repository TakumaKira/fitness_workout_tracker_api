use std::{
  marker::PhantomData,
  pin::Pin,
  task::{Context, Poll},
};
use pin_project::pin_project;
use actix_utils::future::{ok, Either, Ready};
use actix_web::{
  body::{EitherBody, MessageBody},
  dev::{Service, ServiceRequest, ServiceResponse, Transform},
  http::StatusCode,
  Error, HttpResponse,
};
use futures::{ready, Future};

use crate::services::auth_service::AuthService;
pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: MessageBody,
{
  type Response = ServiceResponse<EitherBody<B>>;
  type Error = Error;
  type InitError = ();
  type Transform = AuthMiddleware<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(AuthMiddleware { service })
  }
}

pub struct AuthMiddleware<S> {
  service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: MessageBody,
{
  type Response = ServiceResponse<EitherBody<B>>;
  type Error = Error;
  type Future = Either<AuthenticationFuture<S, B>, Ready<Result<Self::Response, Self::Error>>>;

  fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.service.poll_ready(cx)
  }

  fn call(&self, req: ServiceRequest) -> Self::Future {
    if let Some(auth_service) = req.app_data::<actix_web::web::Data<AuthService>>() {
      if let Some(cookie) = req.cookie("session") {
        if auth_service.validate_session(cookie.value()) {
          return Either::left(AuthenticationFuture {
            fut: self.service.call(req),
            _phantom: PhantomData,
          });
        }
      }
    }
    let res = HttpResponse::with_body(StatusCode::UNAUTHORIZED, "Unauthorized");
    Either::right(ok(req
      .into_response(res)
      .map_into_boxed_body()
      .map_into_right_body()
    ))
  }
}

#[pin_project]
pub struct AuthenticationFuture<S, B>
where
  S: Service<ServiceRequest>,
{
  #[pin]
  fut: S::Future,
  _phantom: PhantomData<B>,
}

impl<S, B> Future for AuthenticationFuture<S, B>
where
  B: MessageBody,
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
  type Output = Result<ServiceResponse<EitherBody<B>>, Error>;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let res = match ready!(self.project().fut.poll(cx)) {
        Ok(res) => res,
        Err(err) => return Poll::Ready(Err(err.into())),
    };

    Poll::Ready(Ok(res.map_into_left_body()))
  }
}