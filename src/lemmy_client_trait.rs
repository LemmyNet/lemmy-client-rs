#[cfg(not(target_family = "wasm"))]
use crate::lemmy_client_internal::ClientWrapper;
#[cfg(target_family = "wasm")]
use crate::lemmy_client_internal::Fetch;
use crate::{
  ClientOptions,
  form::{LemmyForm, LemmyRequest},
  response::{LemmyResponse, LemmyResult},
};
use cfg_if::cfg_if;
use http::Method;
use lemmy_api_common::{
  SuccessResponse,
  account::{
    DeleteAccount,
    ListPersonHidden,
    ListPersonHiddenResponse,
    ListPersonLiked,
    ListPersonLikedResponse,
    ListPersonRead,
    ListPersonReadResponse,
    ListPersonSaved,
    ListPersonSavedResponse,
    MyUserInfo,
    SaveUserSettings,
    auth::{
      ChangePassword,
      GenerateTotpSecretResponse,
      GetCaptchaResponse,
      ListLoginsResponse,
      Login,
      LoginResponse,
      PasswordChangeAfterReset,
      PasswordReset,
      Register,
      ResendVerificationEmail,
      UpdateTotp,
      UpdateTotpResponse,
      UserSettingsBackup,
      VerifyEmail,
    },
  },
  comment::{
    CommentResponse,
    GetComment,
    GetComments,
    GetCommentsResponse,
    GetCommentsSlimResponse,
    actions::{
      CreateComment,
      CreateCommentLike,
      DeleteComment,
      EditComment,
      SaveComment,
      moderation::{
        DistinguishComment,
        ListCommentLikes,
        ListCommentLikesResponse,
        PurgeComment,
        RemoveComment,
      },
    },
  },
  community::{
    CommunityResponse,
    GetCommunity,
    GetCommunityResponse,
    GetRandomCommunity,
    ListCommunities,
    ListCommunitiesResponse,
    Tag,
    actions::{
      BlockCommunity,
      BlockCommunityResponse,
      CreateCommunity,
      FollowCommunity,
      moderation::{
        AddModToCommunity,
        AddModToCommunityResponse,
        ApproveCommunityPendingFollower,
        BanFromCommunity,
        BanFromCommunityResponse,
        CreateCommunityTag,
        DeleteCommunity,
        DeleteCommunityTag,
        EditCommunity,
        GetCommunityPendingFollowsCount,
        GetCommunityPendingFollowsCountResponse,
        ListCommunityPendingFollows,
        ListCommunityPendingFollowsResponse,
        PurgeCommunity,
        RemoveCommunity,
        TransferCommunity,
        UpdateCommunityTag,
      },
    },
  },
  custom_emoji::{
    CreateCustomEmoji,
    CustomEmojiResponse,
    DeleteCustomEmoji,
    EditCustomEmoji,
    ListCustomEmojis,
    ListCustomEmojisResponse,
  },
  federation::{
    GetFederatedInstancesResponse,
    ResolveObject,
    UserBlockInstanceCommunitiesParams,
    UserBlockInstancePersonsParams,
    administration::{AdminAllowInstanceParams, AdminBlockInstanceParams},
  },
  media::{DeleteImageParams, ListMedia, ListMediaResponse, UploadImageResponse},
  modlog::{GetModlog, GetModlogResponse},
  oauth::{
    AuthenticateWithOauth,
    CreateOAuthProvider,
    DeleteOAuthProvider,
    EditOAuthProvider,
    OAuthProvider,
  },
  person::{
    GetPersonDetails,
    GetPersonDetailsResponse,
    actions::{
      BlockPerson,
      BlockPersonResponse,
      ListPersonContent,
      ListPersonContentResponse,
      NotePerson,
      moderation::{
        BanPerson,
        BanPersonResponse,
        GetRegistrationApplication,
        PurgePerson,
        RegistrationApplicationResponse,
      },
    },
  },
  post::{
    GetPost,
    GetPostResponse,
    GetPosts,
    GetPostsResponse,
    GetSiteMetadata,
    GetSiteMetadataResponse,
    PostResponse,
    actions::{
      CreatePost,
      CreatePostLike,
      DeletePost,
      EditPost,
      HidePost,
      MarkManyPostsAsRead,
      MarkPostAsRead,
      SavePost,
      moderation::{
        FeaturePost,
        ListPostLikes,
        ListPostLikesResponse,
        LockPost,
        PurgePost,
        RemovePost,
      },
    },
  },
  private_message::{
    PrivateMessageResponse,
    actions::{CreatePrivateMessage, DeletePrivateMessage, EditPrivateMessage},
  },
  report::{
    CommentReportResponse,
    CommunityReportResponse,
    CreateCommentReport,
    CreateCommunityReport,
    CreatePostReport,
    CreatePrivateMessageReport,
    GetReportCount,
    GetReportCountResponse,
    ListReports,
    ListReportsResponse,
    PostReportResponse,
    PrivateMessageReportResponse,
    ResolveCommentReport,
    ResolveCommunityReport,
    ResolvePostReport,
    ResolvePrivateMessageReport,
  },
  search::{Search, SearchResponse},
  site::{
    GetSiteResponse,
    SiteResponse,
    administration::{
      AddAdmin,
      AddAdminResponse,
      AdminListUsers,
      AdminListUsersResponse,
      ApproveRegistrationApplication,
      CreateSite,
      EditSite,
      GetUnreadRegistrationApplicationCountResponse,
      ListRegistrationApplications,
      ListRegistrationApplicationsResponse,
    },
  },
  tagline::{
    ListTaglines,
    ListTaglinesResponse,
    TaglineResponse,
    aministration::{CreateTagline, DeleteTagline, UpdateTagline},
  },
};
use std::collections::HashMap;

/// API wrapper for lemmy
pub struct LemmyClient<Domain: AsRef<str>> {
  headers: HashMap<String, String>,
  #[cfg(target_family = "wasm")]
  client: Fetch<Domain>,
  #[cfg(not(target_family = "wasm"))]
  client: ClientWrapper<Domain>,
}

impl<Domain: AsRef<str>> LemmyClient<Domain> {
  /// Creates a new [`LemmyClient`].
  /// # Examples
  /// ```
  /// # use lemmy_client::{LemmyClient, ClientOptions};
  /// let client = LemmyClient::new(ClientOptions {
  ///     domain: String::from("lemmy.ml"),
  ///     secure: true
  /// });
  /// ```
  pub fn new(options: ClientOptions<Domain>) -> Self {
    cfg_if! {
        if #[cfg(target_family = "wasm")] {
            Self {
                client: Fetch::new(options),
                headers: HashMap::new()
            }
        } else {
            Self {
                client: ClientWrapper::new(options),
                headers: HashMap::new()
            }
        }
    }
  }

  /// Gets the options passed to the client.
  pub fn options(&self) -> &ClientOptions<Domain> {
    cfg_if! {
        if #[cfg(target_family = "wasm")] {
            &self.client.0
        } else {
            &self.client.options
        }
    }
  }

  /// Returns a map of headers that will be included with each request.
  pub fn headers(&self) -> &HashMap<String, String> {
    &self.headers
  }

  /// Returns a mutable map of headers that will be included with each request.
  /// Use this method if you want to add headers other than the JWT.
  pub fn headers_mut(&mut self) -> &mut HashMap<String, String> {
    &mut self.headers
  }

  /// Delegates request making logic to the private client implementation.
  async fn make_request<Response, Form>(
    &self,
    method: Method,
    path: &str,
    request: LemmyRequest<Form>,
  ) -> LemmyResult<Response>
  where
    Response: LemmyResponse,
    Form: LemmyForm,
  {
    self
      .client
      .make_request(method, path, request, &self.headers)
      .await
  }

  /// Delegates request making logic (for files) to the private client implementation.
  async fn make_file_request(
    &self,
    path: &str,
    request: LemmyRequest<&'static [u8]>,
  ) -> LemmyResult<UploadImageResponse> {
    self
      .client
      .make_file_request(path, request, &self.headers)
      .await
  }
}

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
        impl<Domain: AsRef<str>> LemmyClient<Domain> {
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
        impl<Domain: AsRef<str>> LemmyClient<Domain> {
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

impl_client![
  // Site
  (
    get_site,
    Method::GET,
    "site",
    (),
    GetSiteResponse,
    r#"Gets the site and, if you pass an authorized JWT, information about the logged in user.

HTTP GET /site"#
  ),
  (
    create_site,
    Method::POST,
    "site",
    CreateSite,
    SiteResponse,
    r#"Creates site during initial setup.

HTTP POST /site"#
  ),
  (
    edit_site,
    Method::PUT,
    "site",
    EditSite,
    SiteResponse,
    r#"Edit settings for the site you administer.

HTTP PUT /site"#
  ),
  // TODO: Add stuff for icon and banner
  (
    get_modlog,
    Method::GET,
    "modlog",
    GetModlog,
    GetModlogResponse,
    r#"Gets the modlog.

HTTP GET /modlog"#
  ),
  (
    search,
    Method::GET,
    "search",
    Search,
    SearchResponse,
    r#"Searches for content.

HTTP GET /search"#
  ),
  (
    resolve_object,
    Method::GET,
    "resolve_object",
    ResolveObject,
    SearchResponse,
    r#"Fetches an object from a non-local instance.

HTTP GET /resolve_object"#
  ),
  (
    get_community,
    Method::GET,
    "community",
    GetCommunity,
    GetCommunityResponse,
    r#"Gets a community.

HTTP GET /community"#
  ),
  (
    create_community,
    Method::POST,
    "community",
    CreateCommunity,
    CommunityResponse,
    r#"Creates a new community.

HTTP POST /community"#
  ),
  (
    update_community,
    Method::PUT,
    "community",
    EditCommunity,
    CommunityResponse,
    r#"Edits a community.

HTTP PUT /community"#
  ),
  (
    get_random_community,
    Method::GET,
    "community/random",
    GetRandomCommunity,
    CommunityResponse,
    r#"Fetches a random community.

HTTP GET /community/random"#
  ),
  (
    list_communities,
    Method::GET,
    "community/list",
    ListCommunities,
    ListCommunitiesResponse,
    r#"Lists communities.

HTTP GET /community/list"#
  ),
  (
    follow_community,
    Method::POST,
    "community/follow",
    FollowCommunity,
    CommunityResponse,
    r#"Subscribes to a community.

HTTP POST /community/follow"#
  ),
  (
    report_community,
    Method::POST,
    "community/report",
    CreateCommunityReport,
    CommunityReportResponse,
    r#"Report a community.

HTTP POST /community/report"#
  ),
  (
    resolve_community_report,
    Method::PUT,
    "community/report/resolve",
    ResolveCommunityReport,
    CommunityReportResponse,
    r#"Resolves a community report.

HTTP PUT /community/report/resolve"#
  ),
  (
    delete_community,
    Method::POST,
    "community/delete",
    DeleteCommunity,
    CommunityResponse,
    r#"Deletes a community.

HTTP POST /community/delete"#
  ),
  (
    remove_community,
    Method::POST,
    "community/remove",
    RemoveCommunity,
    CommunityResponse,
    r#"Removes a community (moderation action).

HTTP POST /community/remove"#
  ),
  (
    transfer_community,
    Method::POST,
    "community/transfer",
    TransferCommunity,
    GetCommunityResponse,
    r#"Transfers a community you own to another user on that community's moderation team.

HTTP POST community/transfer"#
  ),
  (
    ban_from_community,
    Method::POST,
    "community/ban_user",
    BanFromCommunity,
    BanFromCommunityResponse,
    r#"Bans a user from a community.

HTTP POST /community/ban_user"#
  ),
  (
    add_mod_to_community,
    Method::POST,
    "community/mod",
    AddModToCommunity,
    AddModToCommunityResponse,
    r#"Adds a moderator to your community.

HTTP POST /community/mod"#
  ),
  // TODO: Add icon and banner stuff
  (
    create_community_tag,
    Method::POST,
    "community/tag",
    CreateCommunityTag,
    Tag,
    r#"Create a tag for a community you moderate.

HTTP POST /community/tag"#
  ),
  (
    update_community_tag,
    Method::PUT,
    "community/tag",
    UpdateCommunityTag,
    Tag,
    r#"Update an existing tag for a community you moderate.

HTTP PUT /community/tag"#
  ),
  (
    delete_community_tag,
    Method::DELETE,
    "community/tag",
    DeleteCommunityTag,
    Tag,
    r#"Delete an existing tag for a community you moderate.

HTTP DELETE /community/tag"#
  ),
  (
    get_community_pending_follows_count,
    Method::GET,
    "community/pending_follows/count",
    GetCommunityPendingFollowsCount,
    GetCommunityPendingFollowsCountResponse,
    r#"Gets number of pending follows for a given community.

HTTP GET /community/pending_follows/count"#
  ),
  (
    list_community_pending_follows,
    Method::GET,
    "community/pending_follows/list",
    ListCommunityPendingFollows,
    ListCommunityPendingFollowsResponse,
    r#"Gets list of pending follows for a given community.

HTTP GET /community/pending_follows/list"#
  ),
  (
    approve_community_pending_follow,
    Method::POST,
    "community/pending_follows/approve",
    ApproveCommunityPendingFollower,
    SuccessResponse,
    r#"Approve a pending follow for a given community.

HTTP POST community/pending_follows/approve"#
  ),
  // TODO: Need to add multi-community stuff
  (
    get_federated_instances,
    Method::GET,
    "federated_instances",
    (),
    GetFederatedInstancesResponse,
    r#"Gets the instances that are federated with your instance.

HTTP GET /federated_instances"#
  ),
  (
    get_post,
    Method::GET,
    "post",
    GetPost,
    GetPostResponse,
    r#"Gets post.

HTTP GET /post"#
  ),
  (
    create_post,
    Method::POST,
    "post",
    CreatePost,
    PostResponse,
    r#"Creates a post.

HTTP POST /post"#
  ),
  (
    edit_post,
    Method::PUT,
    "post",
    EditPost,
    PostResponse,
    r#"Edits a post you have already created.

HTTP PUT /post"#
  ),
  (
    get_linked_site_metadata,
    Method::GET,
    "post/site_metadata",
    GetSiteMetadata,
    GetSiteMetadataResponse,
    r#"Gets the content type and opengraph data of site linked in a post.

HTTP GET /post/site_metadata"#
  ),
  (
    delete_post,
    Method::POST,
    "post/delete",
    DeletePost,
    PostResponse,
    r#"Deletes a post you have made.

HTTP POST /post/delete"#
  ),
  (
    remove_post,
    Method::POST,
    "post/remove",
    RemovePost,
    PostResponse,
    r#"Removes a post (moderator action).

HTTP POST /post/remove"#
  ),
  (
    mark_post_as_read,
    Method::POST,
    "post/mark_as_read",
    MarkPostAsRead,
    PostResponse,
    r#"Marks a post as read.

HTTP POST /post/mark_as_read"#
  ),
  (
    mark_many_posts_as_read,
    Method::POST,
    "post/mark_as_read/many",
    MarkManyPostsAsRead,
    SuccessResponse,
    r#"Marks several posts as read.

HTTP POST /post/mark_as_read/many"#
  ),
  (
    hide_post,
    Method::POST,
    "post/hide",
    HidePost,
    SuccessResponse,
    r#"Hide a post from list views.

HTTP POST /post/hide"#
  ),
  (
    lock_post,
    Method::POST,
    "post/lock",
    LockPost,
    PostResponse,
    r#"Prevents users from commenting on the post (moderator action).

HTTP POST /post/lock"#
  ),
  (
    feature_post,
    Method::POST,
    "post/feature",
    FeaturePost,
    PostResponse,
    r#"Pins a post to the top of the community page (moderator action).

HTTP POST /post/feature"#
  ),
  (
    list_posts,
    Method::GET,
    "post/list",
    GetPosts,
    GetPostsResponse,
    r#"Gets posts with a variety of filters.

HTTP GET /post/list"#
  ),
  (
    like_post,
    Method::POST,
    "post/like",
    CreatePostLike,
    PostResponse,
    r#"Votes on a post.

HTTP POST /post/like"#
  ),
  (
    list_post_likes,
    Method::GET,
    "post/like/list",
    ListPostLikes,
    ListPostLikesResponse,
    r#"Lists the likes for a post.

HTTP GET /post/like/list"#
  ),
  (
    save_post,
    Method::PUT,
    "post/save",
    SavePost,
    PostResponse,
    r#"Saves a post to your favorites list.

HTTP PUT /post/save"#
  ),
  (
    report_post,
    Method::POST,
    "post/report",
    CreatePostReport,
    PostReportResponse,
    r#"Reports a post to the moderator team of the community the post is in, the admin team of your instance, and the admin team of the poster's instance.

HTTP POST /post/report"#
  ),
  (
    resolve_post_report,
    Method::PUT,
    "post/report/resolve",
    ResolvePostReport,
    PostReportResponse,
    r#"Resolves a post report (moderator action).

HTTP PUT /post/report/resolve"#
  ),
  (
    get_comment,
    Method::GET,
    "comment",
    GetComment,
    CommentResponse,
    r#"Gets a comment.

HTTP GET /comment"#
  ),
  (
    create_comment,
    Method::POST,
    "comment",
    CreateComment,
    CommentResponse,
    r#"Creates a new comment.

HTTP POST /comment"#
  ),
  (
    edit_comment,
    Method::PUT,
    "comment",
    EditComment,
    CommentResponse,
    r#"Edits one of your already-created comments.

HTTP PUT /comment"#
  ),
  (
    delete_comment,
    Method::POST,
    "comment/delete",
    DeleteComment,
    CommentResponse,
    r#"Deletes one of your already-existing comments.

HTTP POST /comment/delete"#
  ),
  (
    remove_comment,
    Method::POST,
    "comment/remove",
    RemoveComment,
    CommentResponse,
    r#"Removes a post (moderator action).

HTTP POST /comment/remove"#
  ),
  //     (
  //         mark_reply_as_read,
  //         Method::POST,
  //         "comment/mark_as_read",
  //         MarkCommentReplyAsRead,
  //         SuccessResponse,
  //         r#"Marks a reply to one of your posts or comments as read.

  // HTTP POST /comment/mark_as_read"#
  //     ),
  (
    distinguish_comment,
    Method::POST,
    "comment/distinguish",
    DistinguishComment,
    CommentResponse,
    r#"Pins a comment to the top of a post's comment section (speak as moderator).

HTTP POST /comment/distinguish"#
  ),
  (
    like_comment,
    Method::POST,
    "comment/like",
    CreateCommentLike,
    CommentResponse,
    r#"Votes on a comment.

HTTP POST /comment/like"#
  ),
  (
    list_comment_likes,
    Method::GET,
    "comment/like/list",
    ListCommentLikes,
    ListCommentLikesResponse,
    r#"Gets the votes for a comment.

HTTP GET /comment/like/list"#
  ),
  (
    save_comment,
    Method::PUT,
    "comment/save",
    SaveComment,
    CommentResponse,
    r#"Saves a comment to your favorites list.

HTTP PUT /comment/save"#
  ),
  (
    list_comments,
    Method::GET,
    "comment/list",
    GetComments,
    GetCommentsResponse,
    r#"Gets comments with various filters.

HTTP GET /comment/list"#
  ),
  (
    list_comments_slim,
    Method::GET,
    "comment/list/slim",
    GetComments,
    GetCommentsSlimResponse,
    r#"Retrieve a slim representation of comments.

HTTP GET /comment/list/slim"#
  ),
  (
    report_comment,
    Method::POST,
    "comment/report",
    CreateCommentReport,
    CommentResponse,
    r#"Reports a comment to the moderator team of the community the comment is in, your instance's admin team, and the commenter's instance's admin team.

HTTP POST /comment/report"#
  ),
  (
    resolve_comment_report,
    Method::PUT,
    "comment/report/resolve",
    ResolveCommentReport,
    CommentReportResponse,
    r#"Resolves a report on a comment made in a community you moderate or instance you administrate.

HTTP PUT /comment/report/resolve"#
  ),
  (
    create_private_message,
    Method::POST,
    "private_message",
    CreatePrivateMessage,
    PrivateMessageResponse,
    r#"Creates and send a private message to another user.

HTTP POST /private_message"#
  ),
  (
    edit_private_message,
    Method::PUT,
    "private_message",
    EditPrivateMessage,
    PrivateMessageResponse,
    r#"Edits a private message you have already sent.

HTTP PUT /private_message"#
  ),
  (
    delete_private_message,
    Method::POST,
    "private_message/delete",
    DeletePrivateMessage,
    PrivateMessageResponse,
    r#"Deletes a private that you have already sent.

HTTP POST /private_message/delete"#
  ),
  //     (
  //         mark_private_message_as_read,
  //         Method::POST,
  //         "private_message/mark_as_read",
  //         MarkPrivateMessageAsRead,
  //         SuccessResponse,
  //         r#"Marks a private message that was sent to you as read.

  // HTTP POST /private_message/mark_as_read"#
  //     ),
  (
    report_private_message,
    Method::POST,
    "private_message/report",
    CreatePrivateMessageReport,
    PrivateMessageReportResponse,
    r#"Reports a private message that was sent to you to your instance's admin team and the sender's instance's admin team.

HTTP POST /private_message/report"#
  ),
  (
    resolve_private_message_report,
    Method::PUT,
    "private_message/report/resolve",
    ResolvePrivateMessageReport,
    PrivateMessageReportResponse,
    r#"Resolves a report of a private message sent to a user on the instance you administrate.

HTTP PUT /private_message/report/resolve"#
  ),
  (
    list_reports,
    Method::GET,
    "report/list",
    ListReports,
    ListReportsResponse,
    r#"List all reports.

HTTP GET /report/list"#
  ),
  (
    register_account,
    Method::POST,
    "account/auth/register",
    Register,
    LoginResponse,
    r#"Registers a new account on an instance.

HTTP POST /account/auth/register"#
  ),
  (
    login,
    Method::POST,
    "account/auth/login",
    Login,
    LoginResponse,
    r#"Logs into the instance, giving you a JWT to use to make authorized requests.

HTTP POST /account/auth/login"#
  ),
  (
    logout,
    Method::POST,
    "account/auth/logout",
    (),
    SuccessResponse,
    r#"Deletes the active session associated with the JWT.

HTTP POST /account/auth/logout"#
  ),
  (
    reset_password,
    Method::POST,
    "account/auth/password_reset",
    PasswordReset,
    SuccessResponse,
    r#"Sends an email to your account (if you have one) with a one time link to change your password. Use this if you forgot your password.

HTTP POST /account/auth/password_reset"#
  ),
  (
    get_captcha,
    Method::GET,
    "account/auth/get_captcha",
    (),
    GetCaptchaResponse,
    r#"Gets a captcha.

HTTP GET /account/auth/get_captcha"#
  ),
  (
    change_password_after_reset,
    Method::POST,
    "account/auth/password_change",
    PasswordChangeAfterReset,
    SuccessResponse,
    r#"Follows through with one time link password reset request.

HTTP POST /account/auth/password_change"#
  ),
  (
    change_password,
    Method::PUT,
    "account/auth/change_password",
    ChangePassword,
    LoginResponse,
    r#"Changes your password if you are already logged in.

HTTP PUT /account/auth/change_password"#
  ),
  (
    generate_totp_secret,
    Method::POST,
    "account/auth/totp/generate",
    (),
    GenerateTotpSecretResponse,
    r#"Generates a secret to enable time-based one time passwords for two-factor authentication.

After this, you will need to call /account/auth/totp/update with a valid token to enable it.

HTTP POST /account/auth/totp/generate"#
  ),
  (
    update_totp,
    Method::POST,
    "account/auth/totp/update",
    UpdateTotp,
    UpdateTotpResponse,
    r#"Enables/disables two-factor authentication.

To enable, you must first call /account/auth/totp/generate to generate a token to pass to this.

You can only disable this if it is already enabled. Again, you must pass a valid TOTP.

HTTP POST /account/auth/totp/update"#
  ),
  (
    verify_email,
    Method::POST,
    "account/auth/verify_email",
    VerifyEmail,
    SuccessResponse,
    r#"Verifies your email. Used when the instance you are registering an account on requires email verification.

HTTP POST /account/auth/verify_email"#
  ),
  (
    resend_verification_email,
    Method::POST,
    "account/auth/resend_verification_email",
    ResendVerificationEmail,
    SuccessResponse,
    r#"Resend a verification email.

HTTP POST /account/auth/resend_verification_email"#
  ),
  (
    get_current_user,
    Method::GET,
    "account",
    (),
    MyUserInfo,
    r#"Return the user associated with the JWT token passed.

HTTP GET /account"#
  ),
  (
    delete_image,
    Method::DELETE,
    "account/media",
    DeleteImageParams,
    SuccessResponse,
    r#"Delete an image that you uploaded.

HTTP DELETE /account/media"#
  ),
  (
    list_media,
    Method::GET,
    "account/media/list",
    ListMedia,
    ListMediaResponse,
    r#"Gets all media posted by the logged in user.

HTTP GET /account/media/list"#
  ),
  //     (
  //         list_inbox,
  //         Method::GET,
  //         "account/inbox",
  //         ListInbox,
  //         ListInboxResponse,
  //         r#"Gets all types of notifications for the current user.

  // HTTP GET /account/inbox"#
  //     ),
  (
    delete_account,
    Method::POST,
    "account/delete",
    DeleteAccount,
    SuccessResponse,
    r#"Deletes your account.

HTTP POST /account/delete"#
  ),
  //     (
  //         mark_comment_mention_as_read,
  //         Method::POST,
  //         "account/mention/comment/mark_as_read",
  //         MarkPersonCommentMentionAsRead,
  //         SuccessResponse,
  //         r#"Mark a comment mention of the current user as read.

  // HTTP POST /account/mention/comment/mark_as_read"#
  //     ),
  //     (
  //         mark_post_mention_as_read,
  //         Method::POST,
  //         "account/mention/post/mark_as_read",
  //         MarkPersonPostMentionAsRead,
  //         SuccessResponse,
  //         r#"Mark a post mention of the current user as read.

  // HTTP POST /account/mention/post/mark_as_read"#
  //     ),
  (
    mark_all_notifications_as_read,
    Method::POST,
    "account/mark_as_read/all",
    (),
    SuccessResponse,
    r#"Marks all notifications (replies, mentions, private messages) as read.

HTTP POST /account/mark_as_read/all"#
  ),
  (
    report_count,
    Method::GET,
    "account/report_count",
    GetReportCount,
    GetReportCountResponse,
    r#"Gets number of reports you can resolve.

HTTP GET /account/report_count"#
  ),
  //     (
  //         unread_count,
  //         Method::GET,
  //         "account/unread_count",
  //         (),
  //         GetUnreadCountResponse,
  //         r#"Gets the number of unread notifications.

  // HTTP GET /account/unread_count"#
  //     ),
  (
    list_logins,
    Method::GET,
    "account/list_logins",
    (),
    ListLoginsResponse,
    r#"Lists login tokens for your user's active sessions.

HTTP GET /account/list_logins"#
  ),
  (
    validate_auth,
    Method::GET,
    "account/validate_auth",
    (),
    SuccessResponse,
    r#"Returns an error message if your auth token is invalid.

HTTP GET /account/validate_auth"#
  ),
  (
    donation_dialog_shown,
    Method::POST,
    "account/donation_dialog_shown",
    (),
    SuccessResponse,
    r#"Make donation dialog appear for users of your instance even if a user dismissed it before.

HTTP POST /account/donation_dialog_shown"#
  ),
  // TODO: Handle Account avatar and banner
  (
    block_person,
    Method::POST,
    "account/block/person",
    BlockPerson,
    BlockPersonResponse,
    r#"Blocks a person.

HTTP POST /account/block/person"#
  ),
  (
    block_community,
    Method::POST,
    "account/block/community",
    BlockCommunity,
    BlockCommunityResponse,
    r#"Blocks a community.

HTTP POST /account/block/community"#
  ),
  (
    user_block_instance_communities,
    Method::POST,
    "account/block/instance/communities",
    UserBlockInstanceCommunitiesParams,
    SuccessResponse,
    r#"Prevents posts from communities from the blocked instance from appearing in your feed.

HTTP POST /account/block/instance/communities"#
  ),
  (
    user_block_instance_users,
    Method::POST,
    "account/block/instance/persons",
    UserBlockInstancePersonsParams,
    SuccessResponse,
    r#"Prevents posts and comments by users from the blocked instance from being fetched.

HTTP POST /account/block/instance/persons"#
  ),
  (
    list_saved,
    Method::GET,
    "account/saved",
    ListPersonSaved,
    ListPersonSavedResponse,
    r#"List posts and comments that were saved by the authenticated user.

HTTP GET /account/saved"#
  ),
  (
    list_read,
    Method::GET,
    "account/read",
    ListPersonRead,
    ListPersonReadResponse,
    r#"List posts and comments that were read by the authenticated user in reverse chronological order.

HTTP GET /account/read"#
  ),
  (
    list_hidden,
    Method::GET,
    "account/hidden",
    ListPersonHidden,
    ListPersonHiddenResponse,
    r#"List posts and comments that were hidden by the authenticated user, ordered by date hidden.

HTTP GET /account/hidden"#
  ),
  (
    list_liked,
    Method::GET,
    "account/liked",
    ListPersonLiked,
    ListPersonLikedResponse,
    r#"List posts and comments that were liked by the authenticated user.

HTTP GET /account/liked"#
  ),
  (
    save_user_settings,
    Method::PUT,
    "account/settings/save",
    SaveUserSettings,
    SuccessResponse,
    r#"Saves your account settings.

HTTP PUT /account/settings/save"#
  ),
  (
    export_settings,
    Method::GET,
    "account/settings/export",
    (),
    UserSettingsBackup,
    r#"Exports a backup of your user settings - including your saved content, followed communities, and blocks - as JSON.

HTTP GET /account/settings/export"#
  ),
  (
    import_settings,
    Method::POST,
    "account/settings/import",
    UserSettingsBackup,
    SuccessResponse,
    r#"Imports a backup of your user settings.

HTTP POST /account/settings/import"#
  ),
  (
    get_person_details,
    Method::GET,
    "person",
    GetPersonDetails,
    GetPersonDetailsResponse,
    r#"Gets the publicly viewable details of a user's account.

HTTP GET /person"#
  ),
  (
    list_person_content,
    Method::GET,
    "person/content",
    ListPersonContent,
    ListPersonContentResponse,
    r#"List posts and comments made by a user.

HTTP GET /person/content"#
  ),
  (
    create_person_note,
    Method::POST,
    "person/note",
    NotePerson,
    SuccessResponse,
    r#"Create a note about another user.

HTTP POST /person/note"#
  ),
  (
    add_admin,
    Method::POST,
    "admin/add",
    AddAdmin,
    AddAdminResponse,
    r#"Adds a user to your instance's admin team.

HTTP POST admin/add"#
  ),
  (
    unread_registration_application_count,
    Method::GET,
    "admin/registration_application/count",
    (),
    GetUnreadRegistrationApplicationCountResponse,
    r#"Gets the number of unread registration applications for the instance you administrate.

HTTP GET /admin/registration_application/count"#
  ),
  (
    list_registration_applications,
    Method::GET,
    "admin/registration_application/list",
    ListRegistrationApplications,
    ListRegistrationApplicationsResponse,
    r#"Gets applications to register an account on the instance you administrate.

HTTP GET /admin/registration_application/list"#
  ),
  (
    approve_registration_application,
    Method::PUT,
    "admin/registration_application/approve",
    ApproveRegistrationApplication,
    RegistrationApplicationResponse,
    r#"Approves a pending registration application.

HTTP PUT /admin/registration_application/approve"#
  ),
  (
    get_registration_application,
    Method::GET,
    "admin/registration_application",
    GetRegistrationApplication,
    RegistrationApplicationResponse,
    r#"Get the application a user submitted when they first registered their account

HTTP GET /admin/registration_application"#
  ),
  (
    purge_person,
    Method::POST,
    "admin/purge/person",
    PurgePerson,
    SuccessResponse,
    r#"Purges a user from the database.

HTTP POST /admin/purge/person"#
  ),
  (
    purge_community,
    Method::POST,
    "admin/purge/community",
    PurgeCommunity,
    SuccessResponse,
    r#"Purges a community from the database.

HTTP POST /admin/purge/community"#
  ),
  (
    purge_post,
    Method::POST,
    "admin/purge/post",
    PurgePost,
    SuccessResponse,
    r#"Purges a post from the database.

HTTP POST /admin/purge/post"#
  ),
  (
    purge_comment,
    Method::POST,
    "admin/purge/comment",
    PurgeComment,
    SuccessResponse,
    r#"Purges a comment from the database.

HTTP POST /admin/purge/comment"#
  ),
  (
    create_tagline,
    Method::POST,
    "admin/tagline",
    CreateTagline,
    TaglineResponse,
    r#"Adds a new tagline to the site.

HTTP POST /admin/tagline"#
  ),
  (
    update_tagline,
    Method::PUT,
    "admin/tagline",
    UpdateTagline,
    TaglineResponse,
    r#"Updates an existing tagline.

HTTP PUT /admin/tagline"#
  ),
  (
    delete_tagline,
    Method::POST,
    "admin/tagline/delete",
    DeleteTagline,
    SuccessResponse,
    r#"Deletes an existing tagline.

HTTP DELETE /admin/tagline/delete"#
  ),
  (
    list_taglines,
    Method::GET,
    "admin/tagline/list",
    ListTaglines,
    ListTaglinesResponse,
    r#"Gets the site's taglines.

HTTP GET /admin/tagline/list"#
  ),
  (
    ban_from_site,
    Method::POST,
    "admin/ban",
    BanPerson,
    BanPersonResponse,
    r#"Bans a person from your instance.

HTTP POST /admin/ban"#
  ),
  (
    list_users,
    Method::GET,
    "admin/users",
    AdminListUsers,
    AdminListUsersResponse,
    r#"Bans a person from your instance.

HTTP GET /admin/users"#
  ),
  (
    leave_admin,
    Method::POST,
    "admin/leave",
    (),
    GetSiteResponse,
    r#"Leave your instance's admin team.

HTTP POST /admin/leave"#
  ),
  (
    admin_block_instance,
    Method::POST,
    "admin/instance/block",
    AdminBlockInstanceParams,
    SuccessResponse,
    r#"Defederates an instance from the current instance.

HTTP POST /admin/instance/block"#
  ),
  (
    admin_allow_instance,
    Method::POST,
    "admin/instance/allow",
    AdminAllowInstanceParams,
    SuccessResponse,
    r#"Allows a given instance to interact with the current instance.

HTTP POST /admin/instance/allow"#
  ),
  (
    create_custom_emoji,
    Method::POST,
    "custom_emoji",
    CreateCustomEmoji,
    CustomEmojiResponse,
    r#"Creates a custom emoji.

HTTP POST /custom_emoji"#
  ),
  (
    edit_custom_emoji,
    Method::PUT,
    "custom_emoji",
    EditCustomEmoji,
    CustomEmojiResponse,
    r#"Edits an existing custom emoji.

HTTP PUT /custom_emoji"#
  ),
  (
    delete_custom_emoji,
    Method::POST,
    "custom_emoji/delete",
    DeleteCustomEmoji,
    CustomEmojiResponse,
    r#"Deletes an existing custom emoji.

HTTP POST /custom_emoji/delete"#
  ),
  (
    list_custom_emojis,
    Method::GET,
    "custom_emoji/list",
    ListCustomEmojis,
    ListCustomEmojisResponse,
    r#"List all custom emojis on the instance.

HTTP GET /custom_emoji/list"#
  ),
  (
    add_oauth_provider,
    Method::POST,
    "oauth_provider",
    CreateOAuthProvider,
    OAuthProvider,
    r#"Add an OAuth provider your users can use to register for and log into your instance.

HTTP POST /oauth_provider"#
  ),
  (
    edit_oauth_provider,
    Method::PUT,
    "oauth_provider",
    EditOAuthProvider,
    OAuthProvider,
    r#"Edit one of your instance's OAuth providers.

HTTP PUT /oauth_provider"#
  ),
  (
    delete_oauth_provider,
    Method::POST,
    "oauth_provider/delete",
    DeleteOAuthProvider,
    SuccessResponse,
    r#"Remove an OAuth provider from your instance.

HTTP POST /oauth_provider/delete"#
  ),
  (
    authenticate_with_oauth,
    Method::POST,
    "oauth/authenticate",
    AuthenticateWithOauth,
    LoginResponse,
    r#"Register/login to an instance using one of the OAuth providers that instance supports.

HTTP POST /oauth/authenticate"#
  ),
  (
    list_all_media,
    Method::GET,
    "admin/list_all_media",
    ListMedia,
    ListMediaResponse,
    r#"Gets all media posted on an instance. Only usable by the instance's admins.

HTTP GET /admin/list_all_media"#
  ),
];
// TODO: OAuth and image stuff

impl_client![
  (
    upload_site_icon,
    "icon",
    r#"Upload an icon for your site. This is shown as the site favicon, in site header, and is used as metadata for
external instance listings like [join-lemmy.org](https://join-lemmy.org/instances).

**Only usable by instance admins**

HTTP POST /icon"#
  ),
  (
    upload_site_banner,
    "banner",
    r#"Upload a banner for your site. This is shown in the site sidebar and is used as metadata for
external instance listings like [join-lemmy.org](https://join-lemmy.org/instances).

**Only usable by instance admins**

HTTP POST /banner"#
  )
];
