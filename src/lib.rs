use serde::{Deserialize, Serialize};
use http::method::Method;

pub struct LemmyRequest<R: Serialize> {
  pub body: Option<R>,
  pub jwt: Option<String>,
}

impl<R: Serialize> LemmyRequest<R> {
  pub fn from_jwt(jwt: Option<String>) -> Self {
    Self {
      body: None::<R>,
      jwt,
    }
  }
}

impl<R: Serialize> From<R> for LemmyRequest<R> {
  fn from(body: R) -> Self {
    LemmyRequest {
      body: Some(body),
      jwt: None,
    }
  }
}

mod private_trait {
  use async_trait::async_trait;
use super::{Deserialize, Serialize, LemmyRequest, Method};


  #[async_trait(?Send)]
  pub trait LemmyClient {
    async fn make_request<Response, Form, Request>(
      &self,
      method: Method,
      path: &str,
      form: Request,
    ) -> Result<Response, ()>
    where
      Response: for<'de> Deserialize<'de>,
      Form: Serialize,
      Request: Into<LemmyRequest<Form>>;
  }
}
