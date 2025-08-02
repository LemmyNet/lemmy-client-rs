use crate::{
  ClientOptions,
  form::{LemmyForm, LemmyRequest},
  response::{LemmyResponse, LemmyResult},
  utils::ClientOptionsInternal,
};
use http::{HeaderMap, Method, header::USER_AGENT};
use lemmy_api_common::{error::LemmyErrorType, media::UploadImageResponse};
use reqwest::{Client, RequestBuilder};
use std::borrow::Cow;

trait WithHeaders {
  fn with_headers(self, headers: &HeaderMap) -> Self;
}

trait MaybeWithJwt {
  fn maybe_with_jwt(self, jwt: Option<String>) -> Self;
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

impl MaybeWithJwt for RequestBuilder {
  fn maybe_with_jwt(self, jwt: Option<String>) -> Self {
    if let Some(jwt) = jwt {
      self.bearer_auth(jwt)
    } else {
      self
    }
  }
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
    let route = build_route(path, &self.options);
    let LemmyRequest { body, jwt } = request;

    let res = match method {
      Method::GET => self
        .client
        .get(route)
        .with_headers(&self.headers)
        .maybe_with_jwt(jwt)
        .query(&body),
      Method::POST => self
        .client
        .post(route)
        .with_headers(&self.headers)
        .maybe_with_jwt(jwt)
        .json(&body),
      Method::PUT => self
        .client
        .put(route)
        .with_headers(&self.headers)
        .maybe_with_jwt(jwt)
        .json(&body),
      _ => unreachable!("This crate does not use other HTTP methods."),
    }
    .send()
    .await
    .map_err(map_other_error)?
    .text()
    .await
    .map_err(map_other_error)?;

    deserialize_response(&res)
  }

  pub(crate) async fn make_file_request(
    &self,
    path: &str,
    request: LemmyRequest<&'static [u8]>,
  ) -> LemmyResult<UploadImageResponse> {
    let route = build_route(path, &self.options);
    let LemmyRequest { body, jwt } = request;

    let res = self
      .client
      .post(route)
      .with_headers(&self.headers)
      .maybe_with_jwt(jwt)
      .body(body)
      .send()
      .await
      .map_err(map_other_error)?
      .text()
      .await
      .map_err(map_other_error)?;

    deserialize_response(&res)
  }
}
