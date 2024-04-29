#![warn(missing_docs)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/LemmyNet/lemmy-ui/main/src/assets/icons/favicon.svg"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/LemmyNet/lemmy-ui/main/src/assets/icons/favicon.svg"
)]
//! A Rust HTTP client for Lemmy.
//! If used when targeting WASM, uses the browser's built-in fetch API to reduce bundle size.
//! # Example
//! ```
//! use lemmy_client::{LemmyClient, ClientOptions};
//!
//! async fn get_site_test() {
//!   let client = LemmyClient::new(ClientOptions {
//!     domain: String::from("lemmy.ml"),
//!     secure: true
//!   });
//!
//!   let res = client.get_site(()).await;
//!
//!   assert!(res.is_ok());
//! }
//! ```
use std::collections::HashMap;

use crate::{lemmy_client_trait::LemmyClientInternal, response::LemmyResult};
use cfg_if::cfg_if;
use lemmy_api_common::{
    comment::*, community::*, custom_emoji::*, lemmy_db_schema::source::login_token::LoginToken,
    person::*, post::*, private_message::*, site::*, SuccessResponse,
};
#[cfg(not(target_family = "wasm"))]
use lemmy_client_internal::ClientWrapper;
#[cfg(target_family = "wasm")]
use lemmy_client_internal::Fetch;

mod form;
mod lemmy_client_internal;
mod lemmy_client_trait;
mod response;
mod utils;

pub use form::LemmyRequest;
pub use lemmy_api_common;
pub use utils::ClientOptions;

/// API wrapper for lemmy
pub struct LemmyClient {
    headers: HashMap<String, String>,
    #[cfg(target_family = "wasm")]
    client: Fetch,
    #[cfg(not(target_family = "wasm"))]
    client: ClientWrapper,
}

macro_rules! expose_wrapped_fn {
    ($name:ident, $form:ty, $response:ty, $doc:expr) => {
        #[doc = $doc]
        pub async fn $name<Request>(&self, request: Request) -> LemmyResult<$response>
        where
            Request: Into<LemmyRequest<$form>>,
        {
            self.client.$name(request.into(), &self.headers).await
        }
    };
}

impl LemmyClient {
    /// Creates a new `LemmyClient`.
    /// # Examples
    /// ```
    /// use lemmy_client::{LemmyClient, ClientOptions};
    /// let client = LemmyClient::new(ClientOptions {
    ///     domain: String::from("lemmy.ml"),
    ///     secure: true
    /// });
    /// ```
    pub fn new(options: ClientOptions) -> Self {
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

    /// Map of headers that will be included with each request.
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Mutable map of headers that will be included with each request. Use this method if you want to add headers other than the JWT.
    pub fn headers_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.headers
    }

    expose_wrapped_fn!(
        get_site,
        (),
        GetSiteResponse,
        r#"Gets the site and, if you pass an authorized JWT, information about the logged in user.

HTTP GET /site"#
    );
    expose_wrapped_fn!(
        create_site,
        CreateSite,
        SiteResponse,
        r#"Creates site during initial setup.

HTTP POST /site"#
    );
    expose_wrapped_fn!(
        edit_site,
        EditSite,
        SiteResponse,
        r#"Edits your site.

HTTP PUT /site"#
    );
    expose_wrapped_fn!(block_instance, BlockInstance, BlockInstanceResponse, r#"Blocks an instance.

HTTP POST /site/block"#);
    expose_wrapped_fn!(
        get_modlog,
        GetModlog,
        GetModlogResponse,
        r#"Gets the modlog.

HTTP GET /modlog"#
    );
    expose_wrapped_fn!(
        search,
        Search,
        SearchResponse,
        r#"Searches for content.

HTTP GET /search"#
    );
    expose_wrapped_fn!(
        resolve_object,
        ResolveObject,
        ResolveObjectResponse,
        r#"Fetches an object from a non-local instance.

HTTP GET /resolve_object"#
    );
    expose_wrapped_fn!(
        get_community,
        GetCommunity,
        GetCommunityResponse,
        r#"Gets a community.

HTTP GET /community"#
    );
    expose_wrapped_fn!(
        create_community,
        CreateCommunity,
        CommunityResponse,
        r#"Creates a new community.

HTTP POST /community"#
    );
    expose_wrapped_fn!(
        edit_community,
        EditCommunity,
        CommunityResponse,
        r#"Edits a community.

HTTP PUT /community"#
    );
    expose_wrapped_fn!(
        hide_community,
        HideCommunity,
        SuccessResponse,
        r#"Hides a community from public view.

HTTP PUT /community_hide"#
    );
    expose_wrapped_fn!(
        list_communities,
        ListCommunities,
        ListCommunitiesResponse,
        r#"Lists communities.

HTTP GET /community/list"#
    );
    expose_wrapped_fn!(
        follow_community,
        FollowCommunity,
        CommunityResponse,
        r#"Subscribes to a community.

HTTP POST /community/follow"#
    );
    expose_wrapped_fn!(
        block_community,
        BlockCommunity,
        BlockCommunityResponse,
        r#"Blocks a community.

HTTP POST /community/block"#
    );
    expose_wrapped_fn!(
        delete_community,
        DeleteCommunity,
        CommunityResponse,
        r#"Deletes a community.

HTTP POST /community/delete"#
    );
    expose_wrapped_fn!(
        remove_community,
        RemoveCommunity,
        CommunityResponse,
        r#"Removes a community (moderation action).

HTTP POST /community/remove"#
    );
    expose_wrapped_fn!(
        transfer_community,
        TransferCommunity,
        GetCommunityResponse,
        r#"Transfers a community you own to another user on that community's moderation team.

HTTP POST community/transfer"#
    );
    expose_wrapped_fn!(
        ban_from_community,
        BanFromCommunity,
        BanFromCommunityResponse,
        r#"Bans a user from a community.

HTTP POST /community/ban_user"#
    );
    expose_wrapped_fn!(
        add_mod_to_community,
        AddModToCommunity,
        AddModToCommunityResponse,
        r#"Adds a moderator to your community.

HTTP POST /community/mod"#
    );
    expose_wrapped_fn!(
        get_federated_instances,
        FederatedInstances,
        GetFederatedInstancesResponse,
        r#"Gets the instances that are federated with your instance.

HTTP GET /federated_instances"#
    );
    expose_wrapped_fn!(
        get_post,
        GetPost,
        GetPostResponse,
        r#"Gets post.

HTTP GET /post"#
    );
    expose_wrapped_fn!(
        create_post,
        CreatePost,
        PostResponse,
        r#"Creates a post.

HTTP POST /post"#
    );
    expose_wrapped_fn!(
        edit_post,
        EditPost,
        PostResponse,
        r#"Edits a post you have already created.

HTTP PUT /post"#
    );
    expose_wrapped_fn!(
        delete_post,
        DeletePost,
        PostResponse,
        r#"Deletes a post you have made.

HTTP POST /post/delete"#
    );
    expose_wrapped_fn!(
        remove_post,
        RemovePost,
        PostResponse,
        r#"Removes a post (moderator action).

HTTP POST /post/remove"#
    );
    expose_wrapped_fn!(
        mark_post_as_read,
        MarkPostAsRead,
        SuccessResponse,
        r#"Marks a post as read.

HTTP POST /post/mark_as_read"#
    );
    expose_wrapped_fn!(
        lock_post,
        LockPost,
        PostResponse,
        r#"Prevents users from commenting on the post (moderator action).

HTTP POST /post/lock"#
    );
    expose_wrapped_fn!(
        feature_post,
        FeaturePost,
        PostResponse,
        r#"Pins a post to the top of the community page (moderator action).

HTTP POST /post/feature"#
    );
    expose_wrapped_fn!(
        list_posts,
        GetPosts,
        GetPostsResponse,
        r#"Gets posts with a variety of filters.

HTTP GET /post/list"#
    );
    expose_wrapped_fn!(
        like_post,
        CreatePostLike,
        PostResponse,
        r#"Votes on a post.

HTTP POST /post/like"#
    );
    expose_wrapped_fn!(
        list_post_likes,
        ListPostLikes,
        ListPostLikesResponse,
        r#"Lists the likes for a post.

HTTP GET /post/like/list"#
    );
    expose_wrapped_fn!(
        save_post,
        SavePost,
        PostResponse,
        r#"Saves a post to your favorites list.

HTTP PUT /post/save"#
    );
    expose_wrapped_fn!(
        report_post,
        CreatePostReport,
        PostReportResponse,
        r#"Reports a post to the moderator team of the community the post is in, the admin team of your instance, and the admin team of the poster's instance.

HTTP POST /post/report"#
    );
    expose_wrapped_fn!(
        resolve_post_report,
        ResolvePostReport,
        PostReportResponse,
        r#"Resolves a post report (moderator action).

HTTP PUT /post/report/resolve"#
    );
    expose_wrapped_fn!(
        list_post_reports,
        ListPostReports,
        ListPostReportsResponse,
        r#"Gets reports of posts that you are able to moderate.

HTTP GET /post/report/list"#
    );
    expose_wrapped_fn!(
        get_post_url_metadata,
        GetSiteMetadata,
        GetSiteMetadataResponse,
        r#"Gets the metadata of a given site.

HTTP POST /post/site_metadata"#
    );
    expose_wrapped_fn!(
        hide_post,
        HidePost,
        SuccessResponse,
        r#"Hide a post from list views.

HTTP POST /post/hide"#
    );
    expose_wrapped_fn!(
        get_comment,
        GetComment,
        CommentResponse,
        r#"Gets a comment.

HTTP GET /comment"#
    );
    expose_wrapped_fn!(
        create_comment,
        CreateComment,
        CommentResponse,
        r#"Creates a new comment.

HTTP POST /comment"#
    );
    expose_wrapped_fn!(
        edit_comment,
        EditComment,
        CommentResponse,
        r#"Edits one of your already-created comments.

HTTP PUT /comment"#
    );
    expose_wrapped_fn!(
        delete_comment,
        DeleteComment,
        CommentResponse,
        r#"Deletes one of your already-existing comments.

HTTP POST /comment/delete"#
    );
    expose_wrapped_fn!(
        remove_comment,
        RemoveComment,
        CommentResponse,
        r#"Removes a post (moderator action).

HTTP POST /comment/remove"#
    );
    expose_wrapped_fn!(
        mark_reply_as_read,
        MarkCommentReplyAsRead,
        CommentReplyResponse,
        r#"Marks a reply to one of your posts or comments as read.

HTTP POST /comment/mark_as_read"#
    );
    expose_wrapped_fn!(
        distinguish_comment,
        DistinguishComment,
        CommentResponse,
        r#"Pins a comment to the top of a post's comment section (speak as moderator).

HTTP POST /comment/distinguish"#
    );
    expose_wrapped_fn!(
        like_comment,
        CreateCommentLike,
        CommentResponse,
        r#"Votes on a comment.

HTTP POST /comment/like"#
    );
    expose_wrapped_fn!(
        list_comment_likes,
        ListCommentLikes,
        ListCommentLikesResponse,
        r#"Gets the votes for a comment.

HTTP GET /comment/like/list"#
    );
    expose_wrapped_fn!(
        save_comment,
        SaveComment,
        CommentResponse,
        r#"Saves a comment to your favorites list.

HTTP PUT /comment/save"#
    );
    expose_wrapped_fn!(
        list_comments,
        GetComments,
        GetCommentsResponse,
        r#"Gets comments with various filters.

HTTP GET /comment/list"#
    );
    expose_wrapped_fn!(
        create_comment_report,
        CreateCommentReport,
        CommentResponse,
        r#"Reports a comment to the moderator team of the community the comment is in, your instance's admin team, and the commentor's instance's admin team.

HTTP POST /comment/report"#
    );
    expose_wrapped_fn!(
        resolve_comment_report,
        ResolveCommentReport,
        CommentReportResponse,
        r#"Resolves a report on a comment made in a community you moderate or instance you administrate.

HTTP PUT /comment/report/resolve"#
    );
    expose_wrapped_fn!(
        list_comment_reports,
        ListCommentReports,
        ListCommentReportsResponse,
        r#"Lists reports for comments in communities you moderate or instances you adminstrate.

HTTP GET /comment/report/list"#
    );
    expose_wrapped_fn!(
        create_private_message,
        CreatePrivateMessage,
        PrivateMessageResponse,
        r#"Creates and send a private message to another user.

HTTP POST /private_message"#
    );
    expose_wrapped_fn!(
        edit_private_message,
        EditPrivateMessage,
        PrivateMessageResponse,
        r#"Edits a private message you have already sent.

HTTP PUT /private_message"#
    );
    expose_wrapped_fn!(
        list_private_messages,
        GetPrivateMessages,
        PrivateMessagesResponse,
        r#"Lists private messages that have been sent to you.

HTTP GET /private_message/list"#
    );
    expose_wrapped_fn!(
        delete_private_message,
        DeletePrivateMessage,
        PrivateMessageResponse,
        r#"Deletes a private that you have already sent.

HTTP POST /private_message/delete"#
    );
    expose_wrapped_fn!(
        mark_private_message_as_read,
        MarkPrivateMessageAsRead,
        PrivateMessageResponse,
        r#"Marks a private message that was sent to you as read.

HTTP POST /private_message/mark_as_read"#
    );
    expose_wrapped_fn!(
        create_private_message_report,
        CreatePrivateMessageReport,
        PrivateMessageReportResponse,
        r#"Reports a private message that was sent to you to your instance's admin team and the sender's instance's admin team.

HTTP POST /private_message/report"#
    );
    expose_wrapped_fn!(
        resolve_private_message_report,
        ResolvePrivateMessageReport,
        PrivateMessageReportResponse,
        r#"Resolves a report of a private message sent to a user on the instance you administrate.

HTTP PUT /private_message/report/resolve"#
    );
    expose_wrapped_fn!(
        list_private_message_reports,
        ListPrivateMessageReports,
        ListPrivateMessageReportsResponse,
        r#"Lists reports of private messages received on the isntance you administrate.

HTTP GET /private_message/report/list"#
    );
    expose_wrapped_fn!(
        get_person,
        GetPersonDetails,
        GetPersonDetailsResponse,
        r#"Gets the publicly viewable details of a user's account.

HTTP GET /user"#
    );
    expose_wrapped_fn!(
        register_account,
        Register,
        RegistrationApplicationResponse,
        r#"Registers a new account on an instance.

HTTP POST /user/register"#
    );
    expose_wrapped_fn!(
        get_captcha,
        (),
        GetCaptchaResponse,
        r#"Gets a captcha.

HTTP GET /user/get_captcha"#
    );
    expose_wrapped_fn!(
        export_settings,
        (),
        String,
        r#"Exports a backup of your user settings - including your saved content, followed communities, and blocks - as JSON.

HTTP GET /user/export_settings"#
    );
    expose_wrapped_fn!(
        import_settings,
        String,
        SuccessResponse,
        r#"Imports a backup of your user settings.

HTTP POST /user/import_settings"#
    );
    expose_wrapped_fn!(
        list_mentions,
        GetPersonMentions,
        GetPersonMentionsResponse,
        r#"Gets mentions of the authenticated user.

HTTP GET /user/mention"#
    );
    expose_wrapped_fn!(
        mark_mention_as_read,
        MarkPersonMentionAsRead,
        PersonMentionResponse,
        r#"Marks a mention as read.

HTTP POST /user/mention/mark_as_read"#
    );
    expose_wrapped_fn!(
        list_replies,
        GetReplies,
        GetRepliesResponse,
        r#"Gets replies to your posts and comments.

HTTP GET /user/replies"#
    );
    expose_wrapped_fn!(
        ban_from_site,
        BanPerson,
        BanPersonResponse,
        r#"Bans a person from your instance.

HTTP POST /user/ban"#
    );
    expose_wrapped_fn!(
        list_banned_users,
        (),
        BannedPersonsResponse,
        r#"Gets users banned who are banned from your isntance.

HTTP GET /user/banned"#
    );
    expose_wrapped_fn!(
        block_person,
        BlockPerson,
        BlockPersonResponse,
        r#"Blocks a person.

HTTP POST /user/block"#
    );
    expose_wrapped_fn!(
        login,
        Login,
        LoginResponse,
        r#"Logs into the instance, giving you a JWT to use to make authorized requests.

HTTP POST /user/login"#
    );
    expose_wrapped_fn!(
        logout,
        (),
        SuccessResponse,
        r#"Deletes the active session associated with the JWT.

HTTP POST /user/logout"#
    );
    expose_wrapped_fn!(
        delete_account,
        DeleteAccount,
        SuccessResponse,
        r#"Deletes your account.

HTTP POST /user/delete_account"#
    );
    expose_wrapped_fn!(
        reset_password,
        PasswordReset,
        SuccessResponse,
        r#"Sends an email to your account (if you have one) with a one time link to change your password. Use this if you forgot your password.

HTTP POST /user/password_reset"#
    );
    expose_wrapped_fn!(
        change_password_after_reset,
        PasswordChangeAfterReset,
        SuccessResponse,
        r#"Follows through with one time link password reset request.

HTTP POST /user/password_change"#
    );
    expose_wrapped_fn!(
        mark_all_notifications_as_read,
        (),
        GetRepliesResponse,
        r#"Marks all notifications (replies, mentions, private messages) as read.

HTTP POST /user/mark_all_as_read"#
    );
    expose_wrapped_fn!(
        save_user_settings,
        SaveUserSettings,
        SuccessResponse,
        r#"Saves your account settings.

HTTP PUT /user/save_user_settings"#
    );
    expose_wrapped_fn!(
        change_password,
        ChangePassword,
        LoginResponse,
        r#"Changes your password if you are already logged in.

HTTP PUT /user/change_password"#
    );
    expose_wrapped_fn!(
        report_count,
        GetReportCount,
        GetReportCountResponse,
        r#"Gets number of reports you can resolve.

HTTP GET /user/report_count"#
    );
    expose_wrapped_fn!(
        unread_count,
        (),
        GetUnreadCountResponse,
        r#"Gets the number of unread notifications.

HTTP GET /user/unread_count"#
    );
    expose_wrapped_fn!(
        verify_email,
        VerifyEmail,
        SuccessResponse,
        r#"Verifies your email. Used when the instance you are registering an account on requires email verification.

HTTP POST /user/verify_email"#
    );
    expose_wrapped_fn!(
        leave_admin,
        (),
        GetSiteResponse,
        r#"Leave your instance's admin team.

HTTP POST /user/leave_admin"#
    );
    expose_wrapped_fn!(
        generate_totp_secret,
        (),
        GenerateTotpSecretResponse,
        r#"Generates a secret to enable time-based one time passwords for two-factor authentication.

After this, you will need to call /user/totp/update with a vaild token to enable it.

HTTP POST /user/totp/generate"#
    );
    expose_wrapped_fn!(
        update_totp,
        UpdateTotp,
        UpdateTotpResponse,
        r#"Enables/disables two-factor authentivation.

To enable, you must first call /user/totp/generate to generate a token to pass to this.

You can only disable this if it is already enabled. Again, you must pass a valid TOTP.

HTTP POST /user/totp/update"#
    );
    expose_wrapped_fn!(
        list_logins,
        (),
        Vec<LoginToken>,
        r#"Lists login tokens for your user's active sessions.

HTTP GET /user/list_logins"#
    );
    expose_wrapped_fn!(
        validate_auth,
        (),
        SuccessResponse,
        r#"Returns an error message if your auth token is invalid.

HTTP GET /user/validate_auth"#
    );
    expose_wrapped_fn!(
        add_admin,
        AddAdmin,
        AddAdminResponse,
        r#"Adds a user to your instance's admin team.

HTTP POST admin/add"#
    );
    expose_wrapped_fn!(
        unread_registration_application_count,
        (),
        GetUnreadRegistrationApplicationCountResponse,
        r#"Gets the number of unread registration applications for the instance you administrate.

HTTP GET /admin/registration_application/count"#
    );
    expose_wrapped_fn!(
        list_registration_applications,
        ListRegistrationApplications,
        ListRegistrationApplicationsResponse,
        r#"Gets applications to register an account on the instance you administer.

HTTP GET /admin/registration_application/list"#
    );
    expose_wrapped_fn!(
        approve_registration_application,
        ApproveRegistrationApplication,
        RegistrationApplicationResponse,
        r#"Approves a pending registration application.

HTTP PUT /admin/registration_application/approve"#
    );
    expose_wrapped_fn!(
        purge_person,
        PurgePerson,
        SuccessResponse,
        r#"Purges a user from the database.

HTTP POST /admin/purge/person"#
    );
    expose_wrapped_fn!(
        purge_community,
        PurgeCommunity,
        SuccessResponse,
        r#"Purges a community from the database.

HTTP POST /admin/purge/community"#
    );
    expose_wrapped_fn!(
        purge_post,
        PurgePost,
        SuccessResponse,
        r#"Purges a post from the datbase.

HTTP POST /admin/purge/post"#
    );
    expose_wrapped_fn!(
        purge_comment,
        PurgeComment,
        SuccessResponse,
        r#"Purges a comment from the database.

HTTP POST /admin/purge/comment"#
    );
    expose_wrapped_fn!(
        create_custom_emoji,
        CreateCustomEmoji,
        CustomEmojiResponse,
        r#"Creates a custom emoji.

HTTP POST /custom_emoji"#
    );
    expose_wrapped_fn!(
        edit_custom_emoji,
        EditCustomEmoji,
        CustomEmojiResponse,
        r#"Edits an existing custom emoji.

HTTP PUT /custom_emoji"#
    );
    expose_wrapped_fn!(
        delete_custom_emoji,
        DeleteCustomEmoji,
        CustomEmojiResponse,
        r#"Deletes an existing custom emoji.

HTTP POST /custom_emoji/delete"#
    );
    expose_wrapped_fn!(
        list_media,
        ListMedia,
        ListMediaResponse,
        r#"Gets all media posted by the logged in user.

HTTP GET /account/list_media"#
    );
    expose_wrapped_fn!(
        list_all_media,
        ListMedia,
        ListMediaResponse,
        r#"Gets all media posted on an instance. Only usable by the instance's admins.

HTTP GET /admin/list_all_media"#
    );
}
