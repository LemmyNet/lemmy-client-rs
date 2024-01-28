use async_trait::async_trait;
use cfg_if::cfg_if;
use http::method::Method;
use serde::{Deserialize, Serialize};
use std::fmt;

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
    use super::{async_trait, Deserialize, LemmyRequest, Method, Serialize};

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

#[async_trait(?Send)]
trait LemmyClient: private_trait::LemmyClient {}

trait MaybeBearerAuth {
  fn maybe_bearer_auth(self, token: Option<impl fmt::Display>) -> Self;
}

cfg_if! {
  if #[cfg(target_arch = "wasm32")] {
        use gloo_net::http::{Request, RequestBuilder};
    pub struct Fetch;

        impl MaybeBearerAuth for RequestBuilder {
           fn maybe_bearer_auth(self, token: Option<impl fmt::Display>) -> Self {
                if let Some(token) = token {
                    self.header("Authorization", format!("Bearer {token}").as_str())
                } else {
                    self
                }
            }
        }

    #[async_trait(?Send)]
    impl private_trait::LemmyClient for Fetch {
      async fn make_request<Response, Form, Req>(
                &self,
                method: Method,
                path: &str,
                req: Req,
            ) -> LemmyAppResult<Response>
            where
                Response: for<'de> Deserialize<'de>,
                Form: Serialize,
                Req: Into<LemmyRequest<Form>>
            {
                let LemmyRequest { body, .. } = req.into();
                let route = &build_route(path);
                let jwt = get("jwt").and_then(Result::ok);

                // let abort_controller = AbortController::new().ok();
                // let abort_signal = abort_controller.as_ref().map(AbortController::signal);
                // leptos::on_cleanup( move || {
                //     if let Some(abort_controller) = abort_controller {
                //         abort_controller.abort()
                //     }
                // });

                match method {
                    Method::GET =>
                        Request::get(&build_fetch_query(path, body))
                            .maybe_bearer_auth(jwt.as_deref())
                            .abort_signal(abort_signal.as_ref())
                            .build()
                            .expect_throw("Could not parse query params"),
                    Method::POST =>
                        Request::post(route)
                            .maybe_bearer_auth(jwt.as_deref())
                            .abort_signal(abort_signal.as_ref())
                            .json(&body)
                            .expect_throw("Could not parse json body"),
                    Method::PUT =>
                        Request::put(route)
                            .maybe_bearer_auth(jwt.as_deref())
                            .abort_signal(abort_signal.as_ref())
                            .json(&body)
                            .expect_throw("Could not parse json body")
                }.send().await?.json::<Response>().await.map_err(Into::into)
            }
    }
  } else {
        impl MaybeBearerAuth for awc::ClientRequest {
            fn maybe_bearer_auth(self, token: Option<impl fmt::Display>) -> Self {
                if let Some(token) = token {
                    self.bearer_auth(token)
                } else {
                    self
                }
            }
        }
          #[async_trait(?Send)]
        impl private_trait::LemmyClient for awc::Client {
            async fn make_request<Response, Form, Request>(
                &self,
                method: Method,
                path: &str,
                req: Request,
            ) -> Result<Response, ()>
            where
                Response: for<'de> Deserialize<'de>,
                Form: Serialize,
                Request: Into<LemmyRequest<Form>>
            {
                let LemmyRequest {body, jwt} = req.into();
                let route = path;

                match method {
                    Method::GET =>
                        self
                            .get(route)
                            .maybe_bearer_auth(jwt)
                            .query(&body)?
                            .send(),
                    Method::POST =>
                        self
                            .post(route)
                            .maybe_bearer_auth(jwt)
                            .send_json(&body),
                    Method::PUT =>
                        self
                            .put(route)
                            .maybe_bearer_auth(jwt)
                            .send_json(&body)
                }.await?.json::<Response>().await.map_err(Into::into)
            }
        }
  }
}
