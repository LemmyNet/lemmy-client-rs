use lemmy_api_common::{
    comment::*, community::*, custom_emoji::*, person::*, post::*, private_message::*, site::*,
};
use serde::Serialize;
use std::fmt;

use crate::utils::impl_marker_trait;

pub trait LemmyForm: Serialize + Clone + fmt::Debug {}

#[derive(Debug, Clone)]
/// A request to send to lemmy. If you don't want to set the JWT for each request, you can set the Authorization header with [`LemmyClient::headers_mut`](lemmy_client::LemmyClient.headers_mut).
pub struct LemmyRequest<Body>
where
    Body: LemmyForm,
{
    /// The body to send with the request. Uses [`unit`] for when a body is not required.
    pub body: Body,
    /// The JWT that is used when authorization is required.
    pub jwt: Option<String>,
}

impl LemmyRequest<()> {
    /// Returns a request with no body or JWT.
    pub fn empty() -> Self {
        Self {
            body: (),
            jwt: None,
        }
    }

    /// Returns a request with no body and JWT if [`Some`].
    pub fn from_jwt(jwt: Option<String>) -> Self {
        Self { body: (), jwt }
    }
}

impl<Form> From<Form> for LemmyRequest<Form>
where
    Form: LemmyForm,
{
    fn from(body: Form) -> Self {
        Self { body, jwt: None }
    }
}

impl_marker_trait!(
    LemmyForm,
    [
        (),
        String,
        // Comments
        CreateComment,
        CreateCommentLike,
        CreateCommentReport,
        DeleteComment,
        DistinguishComment,
        EditComment,
        GetComment,
        GetComments,
        ListCommentLikes,
        ListCommentReports,
        RemoveComment,
        ResolveCommentReport,
        SaveComment,
        // Communities
        AddModToCommunity,
        BanFromCommunity,
        BlockCommunity,
        CreateCommunity,
        DeleteCommunity,
        EditCommunity,
        FollowCommunity,
        GetCommunity,
        HideCommunity,
        ListCommunities,
        RemoveCommunity,
        TransferCommunity,
        // Emojis
        CreateCustomEmoji,
        DeleteCustomEmoji,
        EditCustomEmoji,
        // Person
        AddAdmin,
        BanPerson,
        BlockPerson,
        ChangePassword,
        DeleteAccount,
        GetPersonDetails,
        GetPersonMentions,
        GetReplies,
        GetReportCount,
        Login,
        MarkCommentReplyAsRead,
        MarkPersonMentionAsRead,
        PasswordChangeAfterReset,
        PasswordReset,
        PersonMentionResponse,
        Register,
        SaveUserSettings,
        UpdateTotp,
        VerifyEmail,
        // Posts
        CreatePost,
        CreatePostLike,
        CreatePostReport,
        DeletePost,
        EditPost,
        FeaturePost,
        GetPost,
        GetPosts,
        GetSiteMetadata,
        ListPostLikes,
        ListPostReports,
        LockPost,
        MarkPostAsRead,
        RemovePost,
        ResolvePostReport,
        SavePost,
        HidePost,
        // Private Messages
        CreatePrivateMessage,
        CreatePrivateMessageReport,
        DeletePrivateMessage,
        EditPrivateMessage,
        GetPrivateMessages,
        ListPrivateMessageReports,
        MarkPrivateMessageAsRead,
        ResolvePrivateMessageReport,
        // Site
        ApproveRegistrationApplication,
        GetRegistrationApplication,
        BlockInstance,
        CreateSite,
        EditSite,
        FederatedInstances,
        GetModlog,
        InstanceWithFederationState,
        ListRegistrationApplications,
        PurgeComment,
        PurgeCommunity,
        PurgePerson,
        PurgePost,
        ResolveObject,
        Search,
        // Media
        ListMedia
    ]
);
