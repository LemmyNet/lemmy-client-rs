/// Implements a marker trait for a list of types.
/// To use, call the macro like a function, passing a trait with no
/// functions as the first arg and a list of types you would
/// like to implement the trait for as the second argument.
///
/// # Example
/// ```ignore
/// trait Unsigned {}
///
/// impl_marker_trait!(
///   Unsigned,
///   [
///     u8,
///     u16,
///     u32,
///     u64,
///     u128 // Trailing comma optional
///   ]
/// );
/// ```
macro_rules! impl_marker_trait {
    ($trait_name:ty, [$( $impler:ty ),+$(,)?]) => {
        $(
            impl $trait_name for $impler {}
        )*
    };
}

pub(crate) use impl_marker_trait;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Options for creating a [`LemmyClient`][client].
///
/// [client]: crate::LemmyClient
pub struct ClientOptions<Domain: AsRef<str>> {
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
