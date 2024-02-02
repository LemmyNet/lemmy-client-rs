use crate::{lemmy_client_trait::LemmyClientInternal, response::LemmyResult};
use cfg_if::cfg_if;
use lemmy_api_common::{
    comment::*, community::*, custom_emoji::*, lemmy_db_schema::source::login_token::LoginToken,
    person::*, post::*, private_message::*, site::*, SuccessResponse,
};
#[cfg(not(target_arch = "wasm32"))]
use lemmy_client_internal::ClientWrapper;
#[cfg(target_arch = "wasm32")]
use lemmy_client_internal::Fetch;
use utils::ClientOptions;

mod error;
mod form;
mod lemmy_client_internal;
mod lemmy_client_trait;
mod response;
mod utils;

#[cfg(target_arch = "wasm32")]
pub struct LemmyClient(Fetch);
#[cfg(not(target_arch = "wasm32"))]
pub struct LemmyClient(ClientWrapper);

macro_rules! expose_wrapped_fn {
    ($name:ident, $form:ty, $response:ty) => {
        pub async fn $name(&self, form: $form) -> LemmyResult<$response> {
            self.0.$name(form).await
        }
    };
}

macro_rules! expose_wrapped_fn_no_form {
    ($name:ident, $response:ty) => {
        pub async fn $name(&self, jwt: Option<String>) -> LemmyResult<$response> {
            // The type of the request form doesn't matter here because this endpoint doesn't take arguments
            self.0.$name(jwt).await
        }
    };
}

impl LemmyClient {
    pub fn new(options: ClientOptions) -> Self {
        cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                Self(Fetch::new(options))
            } else {
                Self(ClientWrapper::new(options))
            }
        }
    }

    expose_wrapped_fn_no_form!(get_site, GetSiteResponse);
    expose_wrapped_fn!(create_site, CreateSite, GetSiteResponse);
    expose_wrapped_fn!(edit_site, EditSite, GetSiteResponse);
    expose_wrapped_fn!(get_modlog, GetModlog, GetModlogResponse);
    expose_wrapped_fn!(search, Search, SearchResponse);
    expose_wrapped_fn!(resolve_object, ResolveObject, ResolveObjectResponse);
    expose_wrapped_fn!(get_community, GetCommunity, GetCommunityResponse);
    expose_wrapped_fn!(create_community, CreateCommunity, GetCommunityResponse);
    expose_wrapped_fn!(edit_community, EditCommunity, GetCommunityResponse);
    expose_wrapped_fn!(hide_community, HideCommunity, GetCommunityResponse);
    expose_wrapped_fn!(list_communities, ListCommunities, ListCommunitiesResponse);
    expose_wrapped_fn!(follow_community, FollowCommunity, GetCommunityResponse);
    expose_wrapped_fn!(block_community, BlockCommunity, GetCommunityResponse);
    expose_wrapped_fn!(delete_community, DeleteCommunity, GetCommunityResponse);
    expose_wrapped_fn!(remove_community, RemoveCommunity, GetCommunityResponse);
    expose_wrapped_fn!(transfer_community, TransferCommunity, GetCommunityResponse);
    expose_wrapped_fn!(
        ban_from_community,
        BanFromCommunity,
        BanFromCommunityResponse
    );
    expose_wrapped_fn!(
        add_mod_to_community,
        AddModToCommunity,
        AddModToCommunityResponse
    );
    expose_wrapped_fn!(
        get_federated_instances,
        FederatedInstances,
        GetFederatedInstancesResponse
    );
    expose_wrapped_fn!(get_post, GetPost, GetPostResponse);
    expose_wrapped_fn!(create_post, CreatePost, GetPostResponse);
    expose_wrapped_fn!(edit_post, EditPost, GetPostResponse);
    expose_wrapped_fn!(delete_post, DeletePost, GetPostResponse);
    expose_wrapped_fn!(remove_post, RemovePost, GetPostResponse);
    expose_wrapped_fn!(mark_post_as_read, MarkPostAsRead, GetPostResponse);
    expose_wrapped_fn!(lock_post, LockPost, GetPostResponse);
    expose_wrapped_fn!(feature_post, FeaturePost, GetPostResponse);
    expose_wrapped_fn!(list_posts, GetPosts, GetPostsResponse);
    expose_wrapped_fn!(like_post, CreatePostLike, GetPostResponse);
    expose_wrapped_fn!(list_post_likes, ListPostLikes, ListPostLikesResponse);
    expose_wrapped_fn!(save_post, SavePost, GetPostResponse);
    expose_wrapped_fn!(report_post, CreatePostReport, PostReportResponse);
    expose_wrapped_fn!(resolve_post_report, ResolvePostReport, PostReportResponse);
    expose_wrapped_fn!(list_post_reports, ListPostReports, ListPostReportsResponse);
    expose_wrapped_fn!(
        get_post_url_metadata,
        GetSiteMetadata,
        GetSiteMetadataResponse
    );
    expose_wrapped_fn!(get_comment, GetComment, CommentResponse);
    expose_wrapped_fn!(create_comment, CreateComment, CommentResponse);
    expose_wrapped_fn!(edit_comment, EditComment, CommentResponse);
    expose_wrapped_fn!(delete_comment, DeleteComment, CommentResponse);
    expose_wrapped_fn!(remove_comment, RemoveComment, CommentResponse);
    expose_wrapped_fn!(
        mark_reply_as_read,
        MarkCommentReplyAsRead,
        CommentReplyResponse
    );
    expose_wrapped_fn!(distinguish_comment, DistinguishComment, CommentResponse);
    expose_wrapped_fn!(like_comment, CreateCommentLike, CommentResponse);
    expose_wrapped_fn!(
        list_comment_likes,
        ListCommentLikes,
        ListCommentLikesResponse
    );
    expose_wrapped_fn!(save_comment, SaveComment, CommentResponse);
    expose_wrapped_fn!(list_comments, GetComments, GetCommentsResponse);
    expose_wrapped_fn!(create_comment_report, CreateCommentReport, CommentResponse);
    expose_wrapped_fn!(
        resolve_client_report,
        ResolveCommentReport,
        CommentReportResponse
    );
    expose_wrapped_fn!(
        list_comment_reports,
        ListCommentReports,
        ListCommentReportsResponse
    );
    expose_wrapped_fn!(
        create_private_message,
        CreatePrivateMessage,
        PrivateMessageResponse
    );
    expose_wrapped_fn!(
        edit_private_message,
        EditPrivateMessage,
        PrivateMessageResponse
    );
    expose_wrapped_fn!(
        list_private_messages,
        GetPrivateMessages,
        PrivateMessagesResponse
    );
    expose_wrapped_fn!(
        delete_private_message,
        DeletePrivateMessage,
        PrivateMessageResponse
    );
    expose_wrapped_fn!(
        mark_private_message_as_read,
        MarkPrivateMessageAsRead,
        PrivateMessageResponse
    );
    expose_wrapped_fn!(
        create_private_message_report,
        CreatePrivateMessageReport,
        PrivateMessageReportResponse
    );
    expose_wrapped_fn!(
        resolve_private_message_report,
        ResolvePrivateMessageReport,
        PrivateMessageReportResponse
    );
    expose_wrapped_fn!(
        list_private_message_reports,
        ListPrivateMessageReports,
        ListPrivateMessageReportsResponse
    );
    expose_wrapped_fn!(get_person, GetPersonDetails, GetPersonDetailsResponse);
    expose_wrapped_fn!(register_account, Register, RegistrationApplicationResponse);
    expose_wrapped_fn_no_form!(get_captcha, GetCaptchaResponse);
    expose_wrapped_fn_no_form!(export_settings, String);
    expose_wrapped_fn!(import_settings, String, SuccessResponse);
    expose_wrapped_fn!(list_mentions, GetPersonMentions, GetPersonMentionsResponse);
    expose_wrapped_fn!(
        mark_mention_as_read,
        MarkPersonMentionAsRead,
        PersonMentionResponse
    );
    expose_wrapped_fn!(list_replies, GetReplies, GetRepliesResponse);
    expose_wrapped_fn!(ban_from_site, BanPerson, BanPersonResponse);
    expose_wrapped_fn_no_form!(list_banned_users, BannedPersonsResponse);
    expose_wrapped_fn!(block_person, BlockPerson, BlockPersonResponse);
    expose_wrapped_fn!(login, Login, LoginResponse);
    expose_wrapped_fn_no_form!(logout, String);
    expose_wrapped_fn!(delete_account, DeleteAccount, GetPersonDetailsResponse);
    expose_wrapped_fn!(reset_password, PasswordReset, SuccessResponse);
    expose_wrapped_fn!(
        change_password_after_reset,
        PasswordChangeAfterReset,
        SuccessResponse
    );
    expose_wrapped_fn_no_form!(mark_all_notifications_as_read, GetRepliesResponse);
    expose_wrapped_fn!(save_user_settings, SaveUserSettings, SuccessResponse);
    expose_wrapped_fn!(change_password, ChangePassword, LoginResponse);
    expose_wrapped_fn!(report_count, GetReportCount, GetReportCountResponse);
    expose_wrapped_fn_no_form!(unread_count, GetUnreadCountResponse);
    expose_wrapped_fn!(verify_email, VerifyEmail, SuccessResponse);
    expose_wrapped_fn_no_form!(leave_admin, GetSiteResponse);
    expose_wrapped_fn_no_form!(generate_totp_secret, GenerateTotpSecretResponse);
    expose_wrapped_fn!(update_totp, UpdateTotp, UpdateTotpResponse);
    expose_wrapped_fn_no_form!(list_logins, Vec<LoginToken>);
    expose_wrapped_fn_no_form!(validate_auth, SuccessResponse);
    expose_wrapped_fn!(add_admin, AddAdmin, AddAdminResponse);
    expose_wrapped_fn_no_form!(
        unread_registration_application_count,
        GetUnreadRegistrationApplicationCountResponse
    );
    expose_wrapped_fn!(
        list_registration_applications,
        ListRegistrationApplications,
        ListRegistrationApplicationsResponse
    );
    expose_wrapped_fn!(
        approve_registration_application,
        ApproveRegistrationApplication,
        RegistrationApplicationResponse
    );
    expose_wrapped_fn!(purge_person, PurgePerson, SuccessResponse);
    expose_wrapped_fn!(purge_community, PurgeCommunity, SuccessResponse);
    expose_wrapped_fn!(purge_post, PurgePost, SuccessResponse);
    expose_wrapped_fn!(purge_comment, PurgeComment, SuccessResponse);
    expose_wrapped_fn!(create_custom_emoji, CreateCustomEmoji, CustomEmojiResponse);
    expose_wrapped_fn!(edit_custom_emoji, EditCustomEmoji, CustomEmojiResponse);
    expose_wrapped_fn!(delete_custom_emoji, DeleteCustomEmoji, CustomEmojiResponse);
}
