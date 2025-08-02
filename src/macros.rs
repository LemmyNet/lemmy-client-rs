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

/// Allows the various API methods to be added to the client without the need to
/// repeat a bunch of boilerplate.
///
/// # Usage
/// The macro is called with a list of tuples for defining methods on the client.
/// 2 types of tuple are used:
/// ## 6-item tuple
/// These tuples are used to define methods that make requests that are serialized to JSON.
/// ### Example
/// ```ignore
/// impl_client![
///   (
///     // Name of the public method that will be defined on the client.
///     create_community,
///     // HTTP method the client will perform when making the request.
///     Method::POST,
///     // Endpoint on Lemmy's API that the request will be made to.
///     // Note that the leading slash is omitted.
///     "community",
///     // Request type.
///     CreateCommunity,
///     // Response type.
///     CommunityResponse,
///     // Doc comment that will appear for the method on docs.rs and IDEs.
///     r#"Creates a new community.
///
/// HTTP POST /community"#
///   ),
///   (
///     get_site,
///     Method::GET,
///     "site",
///     // If an endpoint doesn't need a request body, just use the unit type.
///     (),
///     GetSiteResponse,
///     r#"Gets the site and, if you pass an authorized JWT, information about the logged in user.
///
/// HTTP GET /site"#
///   ) // Trailing comment optional.
/// ];
/// ```
///
/// ## 3-item tuples
/// These tuples are used to define methods for uploading files.
/// Since these requests always have the same request body type (&'static [u8]),
/// response type ([`UploadImageResponse`][image_response]), and HTTP method (POST), all that is
/// needed for these are the method name, endpoint, and doc comment.
///
/// ### Example
/// ```ignore
/// impl_client![
///   (
///     // Name of the public method that will be defined on the client.
///     upload_site_icon,
///     "icon",
///     r#"Upload an icon for your site. This is shown as the site favicon, in site header, and is used as metadata for
/// external instance listings like [join-lemmy.org](https://join-lemmy.org/instances).
///
/// **Only usable by instance admins**
///
/// HTTP POST /icon"#
///   )
/// ];
/// ```
///
/// # IMPORTANT
/// **6-item tuples and 3-item tuples cannot be mixed together in the same call to
/// `impl_client`.**
///
/// ```ignore
/// // ❌ Don't do this
/// impl_client![
///   (
///     create_community,
///     Method::POST,
///     "community",
///     CreateCommunity,
///     CommunityResponse,
///     r#"Creates a new community.
///
/// HTTP POST /community"#
///   ),
///   (
///     get_site,
///     Method::GET,
///     "site",
///     (),
///     GetSiteResponse,
///     r#"Gets the site and, if you pass an authorized JWT, information about the logged in user.
///
/// HTTP GET /site"#
///   ),
///   // Tuple for file request mixed together with tuples for JSON requests.
///   (
///     upload_site_icon,
///     "icon",
///     r#"Upload an icon for your site. This is shown as the site favicon, in site header, and is used as metadata for
/// external instance listings like [join-lemmy.org](https://join-lemmy.org/instances).
///
/// **Only usable by instance admins**
///
/// HTTP POST /icon"#
///   )
/// ];
///
/// // ✅ Call the macro separately for methods that uses JSON and file requests.
/// impl_client![
///   (
///     create_community,
///     Method::POST,
///     "community",
///     CreateCommunity,
///     CommunityResponse,
///     r#"Creates a new community.
///
/// HTTP POST /community"#
///   ),
///   // Can still pass multiple tuples: they just have to have the same number of items.
///   (
///     get_site,
///     Method::GET,
///     "site",
///     (),
///     GetSiteResponse,
///     r#"Gets the site and, if you pass an authorized JWT, information about the logged in user.
///
/// HTTP GET /site"#
///   )
/// ];
///
/// // Call impl_client separately for file requests.
/// impl_client![
///   (
///     upload_site_icon,
///     "icon",
///     r#"Upload an icon for your site. This is shown as the site favicon, in site header, and is used as metadata for
/// external instance listings like [join-lemmy.org](https://join-lemmy.org/instances).
///
/// **Only usable by instance admins**
///
/// HTTP POST /icon"#
///   )
/// ];
/// ```
///
/// [image_response]: lemmy_api_common::media::UploadImageResponse
macro_rules! impl_client {
    ($(($name:ident, $method:expr, $path:expr, $form:ty, $response:ty, $doc:expr)),+$(,)?) => {
        /// Methods for making JSON requests to Lemmy's API endpoints.
        impl LemmyClient {
            $(
                #[doc = $doc]
                pub async fn $name<Request>(&self, request: Request) -> LemmyResult<$response>
                where
                    Request: Into<LemmyRequest<$form>>,
                {
                    self.make_request($method, $path, request.into()).await
                }
            )*
        }
    };

    ($(($name:ident, $path:expr, $doc:expr)),+$(,)?) => {
        /// Methods for uploading images to Lemmy's API endpoints.
        impl LemmyClient {
            $(
                #[doc = $doc]
                pub async fn $name<Request>(&self, request: Request) -> LemmyResult<UploadImageResponse>
                where
                    Request: Into<LemmyRequest<&'static [u8]>>,
                {
                    self.make_file_request($path, request.into()).await
                }
            )*
        }
    };
}
pub(crate) use impl_client;
