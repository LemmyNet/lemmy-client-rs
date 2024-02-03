use crate::{
    form::{LemmyForm, LemmyRequest},
    lemmy_client_trait::{private_trait, LemmyClientInternal},
    response::{LemmyResponse, LemmyResult},
    utils::ClientOptions,
};
use cfg_if::cfg_if;
use http::Method;
use std::fmt;

trait MaybeBearerAuth {
    fn maybe_bearer_auth(self, token: Option<impl fmt::Display>) -> Self;
}

fn build_route(route: &str, ClientOptions { domain, secure }: &ClientOptions) -> String {
    format!(
        "http{}://{domain}/api/v3/{route}",
        if *secure { "s" } else { "" }
    )
}

cfg_if! {
  if #[cfg(target_arch = "wasm32")] {
    use gloo_net::http::{Request, RequestBuilder};
    use http::header;
    use web_sys::wasm_bindgen::UnwrapThrowExt;
    pub struct Fetch(ClientOptions);

    impl Fetch {
        pub fn new(options: ClientOptions) -> Self {
            Self(options)
        }

        fn build_fetch_query<T: serde::Serialize>(&self, path: &str, form: &T) -> String {
            let form_str = serde_urlencoded::to_string(form).unwrap_or_else(|_| path.to_string());
            format!("{}?{}", build_route(path, &self.0), form_str)
        }
    }

    impl MaybeBearerAuth for RequestBuilder {
        fn maybe_bearer_auth(self, token: Option<impl fmt::Display>) -> Self {
            if let Some(token) = token {
                self.header(header::AUTHORIZATION.as_str(), format!("Bearer {token}").as_str())
            } else {
                self
            }
        }
    }

    impl private_trait::LemmyClientInternal for Fetch {
      async fn make_request<Response, Form, Req>(
                &self,
                method: Method,
                path: &str,
                req: Req,
            ) -> LemmyResult<Response>
            where
                Response: LemmyResponse,
                Form: LemmyForm,
                Req: Into<LemmyRequest<Form>>
            {
                let LemmyRequest { body, jwt } = req.into();
                let route = &build_route(path, &self.0);

                #[allow(unused_mut)]
                let mut req = match method {
                    Method::GET => Request::get(&self.build_fetch_query(path, &body)),
                    Method::POST => Request::post(route),
                    Method::PUT => Request::put(route),
                    method => unreachable!("This crate only uses GET, POST, and PUT HTTP methods. Got {method:?}")
                }.maybe_bearer_auth(jwt.as_deref());

                #[cfg(all(feature = "leptos", target_arch = "wasm32"))]
                {
                    use web_sys::AbortController;
                    let abort_controller = AbortController::new().ok();
                    let abort_signal = abort_controller.as_ref().map(AbortController::signal);
                    leptos::on_cleanup( move || {
                        if let Some(abort_controller) = abort_controller {
                            abort_controller.abort()
                        }
                    });
                    req = req.abort_signal(abort_signal.as_ref());
                }

                match method {
                    Method::GET => req.build().expect_throw("Could not parse query params"),
                    Method::POST | Method::PUT => req.json(&body).expect_throw("Could not parse JSON body"),
                    method => unreachable!("This crate only uses GET, POST, and PUT HTTP methods. Got {method:?}")
                }.send()
                 .await?
                 .json::<Response>()
                 .await
                 .map_err(Into::into)
        }
    }

    impl LemmyClientInternal for Fetch {}
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

      pub struct ClientWrapper {
          client: awc::Client,
          options: ClientOptions
      }

      impl ClientWrapper {
          pub fn new(options: ClientOptions) -> Self {
              Self {
                  client: awc::Client::new(),
                  options
              }
          }
      }

      impl private_trait::LemmyClientInternal for ClientWrapper {
            async fn make_request<Response, Form, Request>(
                &self,
                method: Method,
                path: &str,
                req: Request,
            ) -> LemmyResult<Response>
            where
                Response: LemmyResponse,
                Form: LemmyForm,
                Request: Into<LemmyRequest<Form>>
            {
                let LemmyRequest { body, jwt } = req.into();
                let route = build_route(path, &self.options);

                match method {
                    Method::GET =>
                        self
                            .client
                            .get(route)
                            .maybe_bearer_auth(jwt)
                            .query(&body)?
                            .send(),
                    Method::POST =>
                        self
                            .client
                            .post(route)
                            .maybe_bearer_auth(jwt)
                            .send_json(&body),
                    Method::PUT =>
                        self
                            .client
                            .put(route)
                            .maybe_bearer_auth(jwt)
                            .send_json(&body),
                    _ => unreachable!("This crate does not use other HTTP methods.")
                }.await?.json::<Response>().await.map_err(Into::into)
            }
        }

      impl LemmyClientInternal for ClientWrapper {}
  }
}
