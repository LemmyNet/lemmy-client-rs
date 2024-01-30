use crate::{error::Error, forms::LemmyForm, response::LemmyResponse};
use cfg_if::cfg_if;
use http::method::Method::{self, GET, POST, PUT};
use std::fmt;
use lemmy_api_common::{
    comment::*, community::*, custom_emoji::*, lemmy_db_schema::source::login_token::LoginToken, person::*, post::*, private_message::*, site::*, SuccessResponse
};

mod error;
mod forms;
mod response;
mod utils;

type LemmyResult<R> = Result<R, Error>;

struct LemmyRequest<R: LemmyForm> {
    pub body: Option<R>,
    pub jwt: Option<String>,
}

// impl<R: LemmyForm> LemmyRequest<R> {
//     pub fn from_jwt(jwt: Option<String>) -> Self {
//         Self {
//             body: None::<R>,
//             jwt,
//         }
//     }
// }

impl<R: LemmyForm> From<R> for LemmyRequest<R> {
    fn from(body: R) -> Self {
        LemmyRequest {
            body: Some(body),
            jwt: None,
        }
    }
}

mod private_trait {
    use crate::LemmyResult;

    use super::{LemmyForm, LemmyRequest, LemmyResponse, Method};

    pub trait LemmyClient {
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

macro_rules! client_fn_no_arg {
    ($name:ident, $method:expr, $path:expr, $response:ty) => {
        async fn $name(&self) -> LemmyResult<$response>
        {
            // The type of the request form doesn't matter here because this endpoint doesn't take arguments
            self.make_request(GET, $path, LemmyRequest::<GetSiteMetadata>{
                body: None,
                jwt: None
            }).await
        }
    };
}

trait LemmyClient: private_trait::LemmyClient {
    client_fn_no_arg!(get_site, GET, "site", GetSiteResponse);
    client_fn!(create_site, POST, "site", CreateSite, GetSiteResponse);
    client_fn!(edit_site, PUT, "site", EditSite, GetSiteResponse);
    client_fn!(get_modlog, GET, "modlog", GetModlog, GetModlogResponse);
    client_fn!(search, GET, "search", Search, SearchResponse);
    client_fn!(resolve_object, GET, "resolve_object", ResolveObject, ResolveObjectResponse);
    client_fn!(get_community, GET, "community", GetCommunity, GetCommunityResponse);
    client_fn!(create_community, POST, "community", CreateCommunity, GetCommunityResponse);
    client_fn!(edit_community, PUT, "community", EditCommunity, GetCommunityResponse);
    client_fn!(hide_community, PUT, "community/hide", HideCommunity, GetCommunityResponse);
    client_fn!(list_communities, GET, "community/list", ListCommunities, ListCommunitiesResponse);
    client_fn!(follow_community, POST, "community/follow", FollowCommunity, GetCommunityResponse);
    client_fn!(block_community, POST, "community/block", BlockCommunity, GetCommunityResponse);
    client_fn!(delete_community, POST, "community/delete", DeleteCommunity, GetCommunityResponse);
    client_fn!(remove_community, POST, "community/remove", RemoveCommunity, GetCommunityResponse);
    client_fn!(transfer_community, POST, "community/transfer", TransferCommunity, GetCommunityResponse);
    client_fn!(ban_from_community, POST, "community/ban_user", BanFromCommunity, BanFromCommunityResponse);
    client_fn!(add_mod_to_community, POST, "community/mod", AddModToCommunity, AddModToCommunityResponse);
    client_fn!(get_federated_instances, GET, "federated_instances", FederatedInstances, GetFederatedInstancesResponse);
    client_fn!(get_post, GET, "post", GetPost, GetPostResponse);
    client_fn!(create_post, POST, "post", CreatePost, GetPostResponse);
    client_fn!(edit_post, PUT, "post", EditPost, GetPostResponse);
    client_fn!(delete_post, POST, "post/delete", DeletePost, GetPostResponse);
    client_fn!(remove_post, POST, "post/remove", RemovePost, GetPostResponse);
    client_fn!(mark_post_as_read, POST, "post/mark_as_read", MarkPostAsRead, GetPostResponse);
    client_fn!(lock_post, POST, "post/lock", LockPost, GetPostResponse);
    client_fn!(feature_post, POST, "post/feature", FeaturePost, GetPostResponse);
    client_fn!(list_posts, GET, "post/list", GetPosts, GetPostsResponse);
    client_fn!(like_post, POST, "post/like", CreatePostLike, GetPostResponse);
    client_fn!(list_post_likes, GET, "post/like/list", ListPostLikes, ListPostLikesResponse);
    client_fn!(save_post, PUT, "post/save", SavePost, GetPostResponse);
    client_fn!(report_post, POST, "post/report", CreatePostReport, PostReportResponse);
    client_fn!(resolve_post_report, PUT, "post/report/resolve", ResolvePostReport, PostReportResponse);
    client_fn!(list_post_reports, GET, "post/report/list", ListPostReports, ListPostReportsResponse);
    client_fn!(get_post_url_metadata, GET, "post/site_metadata", GetSiteMetadata, GetSiteMetadataResponse);
    client_fn!(get_comment, GET, "comment", GetComment, CommentResponse);
    client_fn!(create_comment, POST, "comment", CreateComment, CommentResponse);
    client_fn!(edit_comment, PUT, "comment", EditComment, CommentResponse);
    client_fn!(delete_comment, POST, "comment/delete", DeleteComment, CommentResponse);
    client_fn!(remove_comment, POST, "comment/remove", RemoveComment, CommentResponse);
    client_fn!(mark_reply_as_read, POST, "comment/mark_as_read", MarkCommentReplyAsRead, CommentReplyResponse);
    client_fn!(distinguish_comment, POST, "comment/distinguish", DistinguishComment, CommentResponse);
    client_fn!(like_comment, POST, "comment/like", LikeComment, CommentResponse);
    client_fn!(list_comment_likes, GET, "comment/like/list", ListCommentLikes, ListCommentLikesResponse);
    client_fn!(save_comment, PUT, "comment/save", SaveComment, CommentResponse);
    client_fn!(list_comments, GET, "comment/list", GetComments, GetCommentsResponse);
    client_fn!(create_comment_report, POST, "comment/report", ReportComment, CommentResponse);
    client_fn!(resolve_client_report, PUT, "comment/report/resolve", ResolveCommentReport, CommentReportResponse);
    client_fn!(list_comment_reports, GET, "comment/report/list", ListCommentReports, ListCommentReportsResponse);
    client_fn!(create_private_message, POST, "private_message", CreatePrivateMessage, PrivateMessageResponse);
    client_fn!(edit_private_message, PUT, "private_message", EditPrivateMessage, PrivateMessageResponse);
    client_fn!(list_private_messages, GET, "private_message/list", ListPrivateMessages, GetPrivateMessages);
    client_fn!(delete_private_message, POST, "private_message/delete", DeletePrivateMessage, PrivateMessageResponse);
    client_fn!(mark_private_message_as_read, POST, "private_message/mark_as_read", MarkPrivateMessageAsRead, PrivateMessageResponse);
    client_fn!(create_private_message_report, POST, "private_message/report", CreatePrivateMessageReport, PrivateMessageReportResponse);
    client_fn!(resolve_private_message_report, PUT, "private_message/report/resolve", ResolvePrivateMessageReport, PrivateMessageReportResponse);
    client_fn!(list_private_message_reports, GET, "private_message/report/list", ListPrivateMessageReports, ListPrivateMessageReportsResponse);
    client_fn!(get_person, GET, "user", GetPersonDetails, GetPersonDetailsResponse);
    client_fn!(register_account, POST, "user/register", Register, RegistrationApplicationResponse);
    client_fn_no_arg!(get_captcha, GET, "user/get_captcha", GetCaptchaResponse);
    client_fn_no_arg!(export_settings, GET, "user/export_settings", String);
    client_fn!(import_settings, POST, "user/import_settings", String, SuccessResponse);
    client_fn!(list_mentions, GET, "user/mention", GetPersonMentions, GetPersonMentionsResponse);
    client_fn!(mark_mention_as_read, POST, "user/mention/mark_as_read", MarkPersonMentionAsRead, PersonMentionResponse);
    client_fn!(list_replies, GET, "user/replies", GetReplies, GetRepliesResponse);
    client_fn!(ban_from_site, POST, "user/ban", BanPerson, BanPersonResponse);
    client_fn_no_arg!(list_banned_users, GET, "user/banned", BannedPersonsResponse);
    client_fn!(block_person, POST, "user/block", BlockPerson, BlockPersonResponse);
    client_fn!(login, POST, "user/login", Login, LoginResponse);
    client_fn_no_arg!(logout, POST, "user/logout", String);
    client_fn!(delete_account, POST, "user/delete_account", DeleteAccount, GetPersonDetailsResponse);
    client_fn!(reset_password, POST, "user/password_reset", PasswordReset, SuccessResponse);
    client_fn!(change_password_after_reset, POST, "user/password_change", PasswordChangeAfterReset, SuccessResponse);
    client_fn_no_arg!(mark_all_notifications_as_read, POST, "user/mark_all_as_read", GetRepliesResponse);
    client_fn!(save_user_settings, PUT, "user/save_user_settings", SaveUserSettings, SuccessResponse);
    client_fn!(change_password, PUT, "user/change_password", ChangePassword, LoginResponse);
    client_fn!(report_count, GET, "user/report_count", GetReportCount, GetReportCountResponse);
    client_fn_no_arg!(unread_count, GET, "user/unread_count", GetUnreadCountResponse);
    client_fn!(verify_email, POST, "user/verify_email", VerifyEmail, SuccessResponse);
    client_fn_no_arg!(leave_admin, POST, "user/verify_email", GetSiteResponse);
    client_fn_no_arg!(generate_totp_secret, POST, "user/totp/generate", GenerateTotpSecretResponse);
    client_fn!(update_totp, POST, "user/totp/update", UpdateTotp, UpdateTotpResponse);
    client_fn_no_arg!(list_logins, GET, "user/list_logins", Vec<LoginToken>);
    client_fn_no_arg!(validate_auth, GET, "user/validate_auth", SuccessResponse);
    client_fn!(add_admin, POST, "admin/add", AddAdmin, AddAdminResponse);
    client_fn_no_arg!(unread_registration_application_count, GET, "admin/registration_application/count", GetUnreadRegistrationApplicationCountResponse);
    client_fn!(list_registration_applications, GET, "admin/registration_application/list", ListRegistrationApplications, ListRegistrationApplicationsResponse);
    client_fn!(approve_registration_application, PUT, "admin/registration_application/approve", ApproveRegistrationApplication, RegistrationApplicationResponse);
    client_fn!(purge_person, POST, "admin/purge/person", PurgePerson, SuccessResponse);
    client_fn!(purge_community, POST, "admin/purge/community", PurgeCommunity, SuccessResponse);
    client_fn!(purge_post, POST, "admin/purge/post", PurgePost, SuccessResponse);
    client_fn!(purge_comment, POST, "admin/purge/comment", PurgeComment, SuccessResponse);
    client_fn!(create_custom_emoji, POST, "custom_emoji", CreateCustomEmoji, CustomEmojiResponse);
    client_fn!(edit_custom_emoji, PUT, "custom_emoji", EditCustomEmoji, CustomEmojiResponse);
    client_fn!(delete_custom_emoji, POST, "custom_emoji/delete", DeleteCustomEmoji, CustomEmojiResponse);
}

trait MaybeBearerAuth {
    fn maybe_bearer_auth(self, token: Option<impl fmt::Display>) -> Self;
}

cfg_if! {
  if #[cfg(target_arch = "wasm32")] {
        use gloo_net::http::{Request, RequestBuilder};
    pub struct Fetch;

        impl MaybeBearerAuth for RequestBuilder {
           fn maybe_bearer_auth(self, token: Option<impl fmt::Display>) -> Self {
                if let Some(token) = token {
                    self.header("Authorization", format!("Bearer {token}").as_str())
                } else {
                    self
                }
            }
        }

    impl private_trait::LemmyClient for Fetch {
      async fn make_request<Response, Form, Req>(
                &self,
                method: Method,
                path: &str,
                req: Req,
            ) -> LemmyAppResult<Response>
            where
                Response: LemmyResponse,
                Form: LemmyForm,
                Req: Into<LemmyRequest<Form>>
            {
                let LemmyRequest { body, .. } = req.into();
                let route = &build_route(path);
                let jwt = get("jwt").and_then(Result::ok);

                // let abort_controller = AbortController::new().ok();
                // let abort_signal = abort_controller.as_ref().map(AbortController::signal);
                // leptos::on_cleanup( move || {
                //     if let Some(abort_controller) = abort_controller {
                //         abort_controller.abort()
                //     }
                // });

                match method {
                    Method::GET =>
                        Request::get(&build_fetch_query(path, body))
                            .maybe_bearer_auth(jwt.as_deref())
                            .abort_signal(abort_signal.as_ref())
                            .build()
                            .expect_throw("Could not parse query params"),
                    Method::POST =>
                        Request::post(route)
                            .maybe_bearer_auth(jwt.as_deref())
                            .abort_signal(abort_signal.as_ref())
                            .json(&body)
                            .expect_throw("Could not parse json body"),
                    Method::PUT =>
                        Request::put(route)
                            .maybe_bearer_auth(jwt.as_deref())
                            .abort_signal(abort_signal.as_ref())
                            .json(&body)
                            .expect_throw("Could not parse json body"),
                    _ => unreachable!("This crate does not use other HTTP methods.")
                }.send().await?.json::<Response>().await.map_err(Into::into)
            }
    }
  } else {
        impl MaybeBearerAuth for awc::ClientRequest {
            fn maybe_bearer_auth(self, token: Option<impl fmt::Display>) -> Self {
                if let Some(token) = token {
                    self.bearer_auth(token)
                } else {
                    self
                }
            }
        }


        impl private_trait::LemmyClient for awc::Client {
            async fn make_request<Response, Form, Request>(
                &self,
                method: Method,
                path: &str,
                req: Request,
            ) -> LemmyResult<Response>
            where
                Response: LemmyResponse,
                Form: LemmyForm,
                Request: Into<LemmyRequest<Form>>
            {
                let LemmyRequest {body, jwt} = req.into();
                let route = path;

                match method {
                    Method::GET =>
                        self
                            .get(route)
                            .maybe_bearer_auth(jwt)
                            .query(&body)?
                            .send(),
                    Method::POST =>
                        self
                            .post(route)
                            .maybe_bearer_auth(jwt)
                            .send_json(&body),
                    Method::PUT =>
                        self
                            .put(route)
                            .maybe_bearer_auth(jwt)
                            .send_json(&body),
                    _ => unreachable!("This crate does not use other HTTP methods.")
                }.await?.json::<Response>().await.map_err(Into::into)
            }
        }
  }
}
