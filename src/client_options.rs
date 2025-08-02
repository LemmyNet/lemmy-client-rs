use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Options for creating a [`LemmyClient`][client].
///
/// [client]: crate::LemmyClient
pub struct ClientOptions<Domain: Into<Cow<'static, str>>> {
  /// Domain of the instance the client will send requests to.
  /// ```
  /// # use lemmy_client::ClientOptions;
  /// // ❌ You should not include the scheme for the domain.
  /// let options = ClientOptions {
  ///     domain: "https://lemmy.ml",
  ///     secure: true
  /// };
  ///
  /// // ✅ All you need is the domain (including subdomain, if applicable).
  /// let options = ClientOptions {
  ///     domain: "lemmy.ml",
  ///     secure: true
  /// };
  /// ```
  pub domain: Domain,
  /// If true, use HTTPS. If false, use HTTP
  pub secure: bool,
}

/// Internal options used by the Lemmy client implementation.
/// This type being non-generic helps cut down on binary size
/// from monomorphization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientOptionsInternal {
  pub domain: Cow<'static, str>,
  pub secure: bool,
}

impl<Domain> From<ClientOptions<Domain>> for ClientOptionsInternal
where
  Domain: Into<Cow<'static, str>>,
{
  fn from(ClientOptions { domain, secure }: ClientOptions<Domain>) -> Self {
    Self {
      secure,
      domain: domain.into(),
    }
  }
}
