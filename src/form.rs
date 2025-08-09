use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone)]
/// A request to send to lemmy. If you don't want to set the JWT for each request, you can set the
/// Authorization header with [`LemmyClient::headers_mut`](lemmy_client::LemmyClient.headers_mut).
pub struct LemmyRequest<'jwt, Body>
where
  Body: Serialize + Clone + fmt::Debug,
{
  /// The body to send with the request. Uses [`unit`] for when a body is not required.
  pub body: Body,
  /// The JWT that is used when authorization is required.
  pub jwt: Option<&'jwt str>,
}

impl<'jwt> LemmyRequest<'jwt, ()> {
  /// Returns a request with no body or JWT.
  pub fn empty() -> Self {
    Self {
      body: (),
      jwt: None,
    }
  }

  /// Returns a request with no body and JWT if [`Some`].
  pub fn from_jwt(jwt: Option<&'jwt str>) -> Self {
    Self { body: (), jwt }
  }
}

impl<'jwt, Body> From<Body> for LemmyRequest<'jwt, Body>
where
  Body: Serialize + Clone + fmt::Debug,
{
  fn from(body: Body) -> Self {
    Self { body, jwt: None }
  }
}
