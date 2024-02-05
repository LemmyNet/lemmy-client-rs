use crate::{form::{LemmyForm, LemmyRequest}, response::{LemmyResponse, LemmyResult}};
use http::Method;
use lemmy_api_common::{
    comment::*, community::*, custom_emoji::*, lemmy_db_schema::source::login_token::LoginToken,
    person::*, post::*, private_message::*, site::*, SuccessResponse,
};

pub mod private_trait {
    use super::{LemmyRequest, Method, LemmyResult, LemmyResponse, LemmyForm};

    pub trait LemmyClientInternal {
        async fn make_request<Response, Form, Request>(
            &self,
            method: Method,
            path: &str,
            form: Request,
        ) -> LemmyResult<Response>
        where
            Response: LemmyResponse,
            Form: LemmyForm,
            Request: Into<LemmyRequest<Form>>;
    }
}

macro_rules! client_fn {
    ($name:ident, $method:expr, $path:expr, $form:ty, $response:ty) => {
        async fn $name<T>(&self, form: T) -> LemmyResult<$response>
            where
                T: Into<LemmyRequest<$form>>
        {
            self.make_request($method, $path, form).await
        }
    };
}

macro_rules! client_fn_no_form {
    ($name:ident, $method:expr, $path:expr, $response:ty) => {
        async fn $name(&self, jwt: Option<String>) -> LemmyResult<$response>
        {
            // The type of the request form doesn't matter here because this endpoint doesn't take arguments
            self.make_request(Method::GET, $path, LemmyRequest::<GetSiteMetadata>{
                body: None,
                jwt
            }).await
        }
    };
}


pub trait LemmyClientInternal: private_trait::LemmyClientInternal {
    client_fn_no_form!(get_site, Method::GET, "site", GetSiteResponse);
    client_fn!(create_site, Method::POST, "site", CreateSite, GetSiteResponse);
    client_fn!(edit_site, Method::PUT, "site", EditSite, GetSiteResponse);
    client_fn!(get_modlog, Method::GET, "modlog", GetModlog, GetModlogResponse);
    client_fn!(search, Method::GET, "search", Search, SearchResponse);
    client_fn!(resolve_object, Method::GET, "resolve_object", ResolveObject, ResolveObjectResponse);
    client_fn!(get_community, Method::GET, "community", GetCommunity, GetCommunityResponse);
    client_fn!(create_community, Method::POST, "community", CreateCommunity, GetCommunityResponse);
    client_fn!(edit_community, Method::PUT, "community", EditCommunity, GetCommunityResponse);
    client_fn!(hide_community, Method::PUT, "community/hide", HideCommunity, GetCommunityResponse);
    client_fn!(list_communities, Method::GET, "community/list", ListCommunities, ListCommunitiesResponse);
    client_fn!(follow_community, Method::POST, "community/follow", FollowCommunity, GetCommunityResponse);
    client_fn!(block_community, Method::POST, "community/block", BlockCommunity, GetCommunityResponse);
    client_fn!(delete_community, Method::POST, "community/delete", DeleteCommunity, GetCommunityResponse);
    client_fn!(remove_community, Method::POST, "community/remove", RemoveCommunity, GetCommunityResponse);
    client_fn!(transfer_community, Method::POST, "community/transfer", TransferCommunity, GetCommunityResponse);
    client_fn!(ban_from_community, Method::POST, "community/ban_user", BanFromCommunity, BanFromCommunityResponse);
    client_fn!(add_mod_to_community, Method::POST, "community/mod", AddModToCommunity, AddModToCommunityResponse);
    client_fn!(get_federated_instances, Method::GET, "federated_instances", FederatedInstances, GetFederatedInstancesResponse);
    client_fn!(get_post, Method::GET, "post", GetPost, GetPostResponse);
    client_fn!(create_post, Method::POST, "post", CreatePost, GetPostResponse);
    client_fn!(edit_post, Method::PUT, "post", EditPost, GetPostResponse);
    client_fn!(delete_post, Method::POST, "post/delete", DeletePost, GetPostResponse);
    client_fn!(remove_post, Method::POST, "post/remove", RemovePost, GetPostResponse);
    client_fn!(mark_post_as_read, Method::POST, "post/mark_as_read", MarkPostAsRead, GetPostResponse);
    client_fn!(lock_post, Method::POST, "post/lock", LockPost, GetPostResponse);
    client_fn!(feature_post, Method::POST, "post/feature", FeaturePost, GetPostResponse);
    client_fn!(list_posts, Method::GET, "post/list", GetPosts, GetPostsResponse);
    client_fn!(like_post, Method::POST, "post/like", CreatePostLike, GetPostResponse);
    client_fn!(list_post_likes, Method::GET, "post/like/list", ListPostLikes, ListPostLikesResponse);
    client_fn!(save_post, Method::PUT, "post/save", SavePost, GetPostResponse);
    client_fn!(report_post, Method::POST, "post/report", CreatePostReport, PostReportResponse);
    client_fn!(resolve_post_report, Method::PUT, "post/report/resolve", ResolvePostReport, PostReportResponse);
    client_fn!(list_post_reports, Method::GET, "post/report/list", ListPostReports, ListPostReportsResponse);
    client_fn!(get_post_url_metadata, Method::GET, "post/site_metadata", GetSiteMetadata, GetSiteMetadataResponse);
    client_fn!(get_comment, Method::GET, "comment", GetComment, CommentResponse);
    client_fn!(create_comment, Method::POST, "comment", CreateComment, CommentResponse);
    client_fn!(edit_comment, Method::PUT, "comment", EditComment, CommentResponse);
    client_fn!(delete_comment, Method::POST, "comment/delete", DeleteComment, CommentResponse);
    client_fn!(remove_comment, Method::POST, "comment/remove", RemoveComment, CommentResponse);
    client_fn!(mark_reply_as_read, Method::POST, "comment/mark_as_read", MarkCommentReplyAsRead, CommentReplyResponse);
    client_fn!(distinguish_comment, Method::POST, "comment/distinguish", DistinguishComment, CommentResponse);
    client_fn!(like_comment, Method::POST, "comment/like", CreateCommentLike, CommentResponse);
    client_fn!(list_comment_likes, Method::GET, "comment/like/list", ListCommentLikes, ListCommentLikesResponse);
    client_fn!(save_comment, Method::PUT, "comment/save", SaveComment, CommentResponse);
    client_fn!(list_comments, Method::GET, "comment/list", GetComments, GetCommentsResponse);
    client_fn!(create_comment_report, Method::POST, "comment/report", CreateCommentReport, CommentResponse);
    client_fn!(resolve_comment_report, Method::PUT, "comment/report/resolve", ResolveCommentReport, CommentReportResponse);
    client_fn!(list_comment_reports, Method::GET, "comment/report/list", ListCommentReports, ListCommentReportsResponse);
    client_fn!(create_private_message, Method::POST, "private_message", CreatePrivateMessage, PrivateMessageResponse);
    client_fn!(edit_private_message, Method::PUT, "private_message", EditPrivateMessage, PrivateMessageResponse);
    client_fn!(list_private_messages, Method::GET, "private_message/list", GetPrivateMessages, PrivateMessagesResponse);
    client_fn!(delete_private_message, Method::POST, "private_message/delete", DeletePrivateMessage, PrivateMessageResponse);
    client_fn!(mark_private_message_as_read, Method::POST, "private_message/mark_as_read", MarkPrivateMessageAsRead, PrivateMessageResponse);
    client_fn!(create_private_message_report, Method::POST, "private_message/report", CreatePrivateMessageReport, PrivateMessageReportResponse);
    client_fn!(resolve_private_message_report, Method::PUT, "private_message/report/resolve", ResolvePrivateMessageReport, PrivateMessageReportResponse);
    client_fn!(list_private_message_reports, Method::GET, "private_message/report/list", ListPrivateMessageReports, ListPrivateMessageReportsResponse);
    client_fn!(get_person, Method::GET, "user", GetPersonDetails, GetPersonDetailsResponse);
    client_fn!(register_account, Method::POST, "user/register", Register, RegistrationApplicationResponse);
    client_fn_no_form!(get_captcha, Method::GET, "user/get_captcha", GetCaptchaResponse);
    client_fn_no_form!(export_settings, Methd::GET, "user/export_settings", String);
    client_fn!(import_settings, Method::POST, "user/import_settings", String, SuccessResponse);
    client_fn!(list_mentions, Method::GET, "user/mention", GetPersonMentions, GetPersonMentionsResponse);
    client_fn!(mark_mention_as_read, Method::POST, "user/mention/mark_as_read", MarkPersonMentionAsRead, PersonMentionResponse);
    client_fn!(list_replies, Method::GET, "user/replies", GetReplies, GetRepliesResponse);
    client_fn!(ban_from_site, Method::POST, "user/ban", BanPerson, BanPersonResponse);
    client_fn_no_form!(list_banned_users, Method::GET, "user/banned", BannedPersonsResponse);
    client_fn!(block_person, Method::POST, "user/block", BlockPerson, BlockPersonResponse);
    client_fn!(login, Method::POST, "user/login", Login, LoginResponse);
    client_fn_no_form!(logout, Method::POST, "user/logout", String);
    client_fn!(delete_account, Method::POST, "user/delete_account", DeleteAccount, GetPersonDetailsResponse);
    client_fn!(reset_password, Method::POST, "user/password_reset", PasswordReset, SuccessResponse);
    client_fn!(change_password_after_reset, Method::POST, "user/password_change", PasswordChangeAfterReset, SuccessResponse);
    client_fn_no_form!(mark_all_notifications_as_read, Method::POST, "user/mark_all_as_read", GetRepliesResponse);
    client_fn!(save_user_settings, Method::PUT, "user/save_user_settings", SaveUserSettings, SuccessResponse);
    client_fn!(change_password, Method::PUT, "user/change_password", ChangePassword, LoginResponse);
    client_fn!(report_count, Method::GET, "user/report_count", GetReportCount, GetReportCountResponse);
    client_fn_no_form!(unread_count, Method::GET, "user/unread_count", GetUnreadCountResponse);
    client_fn!(verify_email, Method::POST, "user/verify_email", VerifyEmail, SuccessResponse);
    client_fn_no_form!(leave_admin, Method::POST, "user/verify_email", GetSiteResponse);
    client_fn_no_form!(generate_totp_secret, Method::POST, "user/totp/generate", GenerateTotpSecretResponse);
    client_fn!(update_totp, Method::POST, "user/totp/update", UpdateTotp, UpdateTotpResponse);
    client_fn_no_form!(list_logins, Method::GET, "user/list_logins", Vec<LoginToken>);
    client_fn_no_form!(validate_auth, Method::GET, "user/validate_auth", SuccessResponse);
    client_fn!(add_admin, Method::POST, "admin/add", AddAdmin, AddAdminResponse);
    client_fn_no_form!(unread_registration_application_count, Method::GET, "admin/registration_application/count", GetUnreadRegistrationApplicationCountResponse);
    client_fn!(list_registration_applications, Method::GET, "admin/registration_application/list", ListRegistrationApplications, ListRegistrationApplicationsResponse);
    client_fn!(approve_registration_application, Method::PUT, "admin/registration_application/approve", ApproveRegistrationApplication, RegistrationApplicationResponse);
    client_fn!(purge_person, Method::POST, "admin/purge/person", PurgePerson, SuccessResponse);
    client_fn!(purge_community, Method::POST, "admin/purge/community", PurgeCommunity, SuccessResponse);
    client_fn!(purge_post, Method::POST, "admin/purge/post", PurgePost, SuccessResponse);
    client_fn!(purge_comment, Method::POST, "admin/purge/comment", PurgeComment, SuccessResponse);
    client_fn!(create_custom_emoji, Method::POST, "custom_emoji", CreateCustomEmoji, CustomEmojiResponse);
    client_fn!(edit_custom_emoji, Method::PUT, "custom_emoji", EditCustomEmoji, CustomEmojiResponse);
    client_fn!(delete_custom_emoji, Method::POST, "custom_emoji/delete", DeleteCustomEmoji, CustomEmojiResponse);
}