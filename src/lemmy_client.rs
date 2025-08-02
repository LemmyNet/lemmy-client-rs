use crate::{
  ClientOptions,
  client_options::ClientOptionsInternal,
  form::{LemmyForm, LemmyRequest},
  response::{LemmyResponse, LemmyResult},
};
use http::{HeaderMap, Method, header::USER_AGENT};
use lemmy_api_common::{error::LemmyErrorType, media::UploadImageResponse};
use reqwest::{Client, RequestBuilder};
use std::borrow::Cow;

trait WithHeaders {
  fn with_headers(self, headers: &HeaderMap) -> Self;
}

impl WithHeaders for RequestBuilder {
  fn with_headers(self, headers: &HeaderMap) -> Self {
    let mut request_builder = headers
      .iter()
      .fold(self, |request_builder, (header, value)| {
        request_builder.header(header, value)
      });

    if !headers.contains_key(USER_AGENT) {
      request_builder = request_builder.header(USER_AGENT, "Lemmy-Client-rs/1.0.0");
    }

    request_builder
  }
}

trait MaybeWithJwt {
  fn maybe_with_jwt(self, jwt: Option<String>) -> Self;
}

impl MaybeWithJwt for RequestBuilder {
  fn maybe_with_jwt(self, jwt: Option<String>) -> Self {
    if let Some(jwt) = jwt {
      self.bearer_auth(jwt)
    } else {
      self
    }
  }
}

fn build_route(
  route: &str,
  ClientOptionsInternal { domain, secure }: &ClientOptionsInternal,
) -> String {
  format!(
    "http{}://{}/api/v4/{route}",
    if *secure { "s" } else { "" },
    domain.as_ref()
  )
}

fn map_other_error<E: ToString>(e: E) -> LemmyErrorType {
  LemmyErrorType::Unknown(e.to_string())
}

fn deserialize_response<Response: LemmyResponse>(res: &str) -> Result<Response, LemmyErrorType> {
  serde_json::from_str::<Response>(res)
    .map_err(|_| serde_json::from_str::<LemmyErrorType>(res).unwrap_or_else(map_other_error))
}

async fn send_request(request_builder: RequestBuilder) -> Result<String, LemmyErrorType> {
  request_builder
    .send()
    .await
    .map_err(map_other_error)?
    .text()
    .await
    .map_err(map_other_error)
}

/// API wrapper for Lemmy
pub struct LemmyClient {
  headers: HeaderMap,
  client: Client,
  options: ClientOptionsInternal,
}

/// Methods for [`LemmyClient`] that don't have to do with specific endpoints
/// on Lemmy's API.
impl LemmyClient {
  /// Creates a new [`LemmyClient`].
  /// # Examples
  /// ```
  /// # use lemmy_client::{LemmyClient, ClientOptions};
  /// let client = LemmyClient::new(ClientOptions {
  ///     domain: "lemmy.ml",
  ///     secure: true
  /// });
  /// ```
  pub fn new<Domain>(options: ClientOptions<Domain>) -> Self
  where
    Domain: Into<Cow<'static, str>>,
  {
    // Private non-generic function for creating a new ['LemmyClient']
    // to cut down on monomorphized code.
    fn inner(options: ClientOptionsInternal) -> LemmyClient {
      LemmyClient {
        headers: HeaderMap::new(),
        client: Client::new(),
        options,
      }
    }

    inner(options.into())
  }

  /// Returns whether or not the client is making requests over HTTPS.
  pub fn secure(&self) -> bool {
    self.options.secure
  }

  /// Returns the domain of the Lemmy instance the client should make
  /// requests to.
  pub fn domain(&self) -> &str {
    &self.options.domain
  }

  /// Returns a map of headers that will be included with each request.
  pub fn headers(&self) -> &HeaderMap {
    &self.headers
  }

  /// Returns a mutable map of headers that will be included with each request.
  /// Use this method if you want to add headers other than the JWT.
  pub fn headers_mut(&mut self) -> &mut HeaderMap {
    &mut self.headers
  }

  /// Create a [`RequestBuilder`] to use for making requests.
  fn create_request_builder(
    &self,
    method: &Method,
    path: &str,
    jwt: Option<String>,
  ) -> RequestBuilder {
    let route = build_route(path, &self.options);

    let request_builder = match method {
      &Method::GET => self.client.get(route),
      &Method::POST => self.client.post(route),
      &Method::PUT => self.client.put(route),
      _ => unreachable!("This crate does not use other HTTP methods."),
    };

    request_builder
      .with_headers(&self.headers)
      .maybe_with_jwt(jwt)
  }

  pub(crate) async fn make_request<Response, Form>(
    &self,
    method: Method,
    path: &str,
    request: LemmyRequest<Form>,
  ) -> LemmyResult<Response>
  where
    Response: LemmyResponse,
    Form: LemmyForm,
  {
    let LemmyRequest { body, jwt } = request;
    let request_builder = self.create_request_builder(&method, path, jwt);

    let request_builder = match method {
      Method::GET => request_builder.query(&body),
      Method::POST | Method::PUT => request_builder.json(&body),
      _ => unreachable!("This crate does not use other HTTP methods."),
    };

    let res = send_request(request_builder).await?;

    deserialize_response(&res)
  }

  pub(crate) async fn make_file_request(
    &self,
    path: &str,
    request: LemmyRequest<&'static [u8]>,
  ) -> LemmyResult<UploadImageResponse> {
    let LemmyRequest { body, jwt } = request;
    let request_builder = self
      .create_request_builder(&Method::POST, path, jwt)
      .body(body);

    let res = send_request(request_builder).await?;

    deserialize_response(&res)
  }
}
