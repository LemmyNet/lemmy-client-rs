use crate::{ClientOptions, client_options::ClientOptionsInternal};
use http::{
  HeaderMap,
  HeaderValue,
  Method,
  header::{AUTHORIZATION, InvalidHeaderValue, USER_AGENT},
};
use lemmy_api_common::{error::LemmyErrorType, media::UploadImageResponse};
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt};

/// A return type for the lemmy result
pub type LemmyResult<R> = Result<R, LemmyErrorType>;

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

fn deserialize_response<Response>(res: &str) -> Result<Response, LemmyErrorType>
where
  Response: for<'de> Deserialize<'de>,
{
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

  /// Set the Authorization header with a JWT token.
  /// There is no need to include the "Bearer" part of the header value.
  pub fn set_jwt(&mut self, jwt: &str) -> Result<(), InvalidHeaderValue> {
    self.headers.insert(
      AUTHORIZATION,
      HeaderValue::try_from(format!("Bearer {jwt}"))?,
    );

    Ok(())
  }

  /// Clear the JWT used by the client.
  ///
  /// <div class="warning">
  ///
  /// **Important Note**: Requests made with the client after this point will not
  /// be able to use endpoints the require authentication unless another JWT is
  /// set with [set_jwt][set_jwt].
  ///
  /// </div>
  ///
  /// [set_jwt]: LemmyClient::set_jwt
  pub fn clear_jwt(&mut self) {
    self.headers.remove(AUTHORIZATION);
  }

  /// Create a [`RequestBuilder`] to use for making requests.
  fn create_request_builder(&self, method: &Method, path: &str) -> RequestBuilder {
    let route = build_route(path, &self.options);

    let mut request_builder = match *method {
      Method::GET => self.client.get(route),
      Method::POST => self.client.post(route),
      Method::PUT => self.client.put(route),
      Method::DELETE => self.client.delete(route),
      _ => unreachable!("This crate does not use other HTTP methods."),
    };

    if !self.headers.contains_key(USER_AGENT) {
      request_builder = request_builder.header(USER_AGENT, "Lemmy-Client-rs/1.0.0");
    }

    request_builder.headers(self.headers.clone())
  }

  pub(crate) async fn make_request<Response, Form>(
    &self,
    method: Method,
    path: &str,
    body: Form,
  ) -> LemmyResult<Response>
  where
    // TODO in the future, we can use trait aliases for these: https://doc.rust-lang.org/unstable-book/language-features/trait-alias.html
    Response: for<'de> Deserialize<'de>,
    Form: Serialize + Clone + fmt::Debug,
  {
    let request_builder = self.create_request_builder(&method, path);

    let request_builder = match method {
      Method::GET | Method::DELETE => request_builder.query(&body),
      Method::POST | Method::PUT => request_builder.json(&body),
      _ => unreachable!("This crate does not use other HTTP methods."),
    };

    let res = send_request(request_builder).await?;

    deserialize_response(&res)
  }

  pub(crate) async fn make_file_request<Form>(
    &self,
    path: &str,
    query: Form,
    body: &'static [u8],
  ) -> LemmyResult<UploadImageResponse>
  where
    Form: Serialize + Clone + fmt::Debug,
  {
    let request_builder = self
      .create_request_builder(&Method::POST, path)
      .query(&query)
      .body(body);

    let res = send_request(request_builder).await?;

    deserialize_response(&res)
  }
}
