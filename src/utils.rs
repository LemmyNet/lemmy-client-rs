macro_rules! impl_marker_trait {
    ($trait_name:ty, [$( $impler:ty ),+$(,)?]) => {
        $(
            impl $trait_name for $impler {}
        )*
    };
}

pub(crate) use impl_marker_trait;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Options for instantiating a `LemmyClient`.
pub struct ClientOptions {
    /// Domain of the instance the client will send requests to.
    /// ```
    /// use lemmy_client::ClientOptions;
    /// // ❌ You do not have to include the scheme for the domain.
    /// let options = ClientOptions {
    ///     domain: String::from("https://lemmy.ml"),
    ///     secure: true
    /// };
    ///
    /// // ✅ All you need is the domain (including subdomain, if applicable).
    /// let options = ClientOptions {
    ///     domain: String::from("lemmy.ml"),
    ///     secure: true
    /// };
    /// ```
    pub domain: String,
    /// If true, use HTTPS. If false, use HTTP
    pub secure: bool,
}
