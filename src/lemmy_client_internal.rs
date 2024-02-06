use crate::{
    form::LemmyForm,
    lemmy_client_trait::{private_trait, LemmyClientInternal},
    response::{LemmyResponse, LemmyResult},
    utils::ClientOptions,
};
use cfg_if::cfg_if;
use http::Method;
use std::collections::HashMap;

trait WithHeaders {
    fn with_headers(self, headers: &HashMap<String, String>) -> Self;
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

    impl WithHeaders for RequestBuilder {
        fn with_headers(self, headers: &HashMap<String, String>) -> Self {
            headers.iter().fold(self, |acc, (header, value)| acc.header(header.as_str(), value.as_str()))
        }
    }

    impl private_trait::LemmyClientInternal for Fetch {
      async fn make_request<Response, Form>(
                &self,
                method: Method,
                path: &str,
                form: Option<Form>,
                headers: &HashMap<String, String>
            ) -> LemmyResult<Response>
            where
                Response: LemmyResponse,
                Form: LemmyForm,
            {
                let route = &build_route(path, &self.0);

                #[allow(unused_mut)]
                let mut req = match method {
                    Method::GET => Request::get(&self.build_fetch_query(path, &form)),
                    Method::POST => Request::post(route),
                    Method::PUT => Request::put(route),
                    method => unreachable!("This crate only uses GET, POST, and PUT HTTP methods. Got {method:?}")
                }.with_headers(headers);

                #[cfg(all(feature = "leptos", target_arch = "wasm32"))]
                {
                    use web_sys::AbortController;
                    let abort_controller = AbortController::new().ok();
                    let abort_signal = abort_controller.as_ref().map(AbortController::signal);
                    leptos::on_cleanup(move || {
                        if let Some(abort_controller) = abort_controller {
                            abort_controller.abort()
                        }
                    });
                    req = req.abort_signal(abort_signal.as_ref());
                }

                match method {
                    Method::GET => req.build().expect_throw("Could not parse query params"),
                    Method::POST | Method::PUT => req.json(&form).expect_throw("Could not parse JSON body"),
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
        impl WithHeaders for awc::ClientRequest {
            fn with_headers(self, headers: &HashMap<String, String>) -> Self {
                headers.iter().fold(self, |acc, (header, value)| acc.insert_header((header.as_str(), value.as_str())))
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
            async fn make_request<Response, Form>(
                &self,
                method: Method,
                path: &str,
                form: Option<Form>,
                headers: &HashMap<String, String>
            ) -> LemmyResult<Response>
            where
                Response: LemmyResponse,
                Form: LemmyForm,
            {
                let route = build_route(path, &self.options);

                match method {
                    Method::GET =>
                        self
                            .client
                            .get(route)
                            .with_headers(headers)
                            .query(&form)?
                            .send(),
                    Method::POST =>
                        self
                            .client
                            .post(route)
                            .with_headers(headers)
                            .send_json(&form),
                    Method::PUT =>
                        self
                            .client
                            .put(route)
                            .with_headers(headers)
                            .send_json(&form),
                    _ => unreachable!("This crate does not use other HTTP methods.")
                }.await?.json::<Response>().await.map_err(Into::into)
            }
        }

      impl LemmyClientInternal for ClientWrapper {}
  }
}
