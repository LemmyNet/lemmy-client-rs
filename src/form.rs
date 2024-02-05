use lemmy_api_common::{
    comment::*, community::*, custom_emoji::*, person::*, post::*, private_message::*, site::*,
};
use serde::Serialize;

use crate::utils::impl_marker_trait;

pub trait LemmyForm: Serialize {}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// A request to send to the Lemmy API.
pub struct LemmyRequest<R: LemmyForm> {
    /// The body of the request/
    pub body: Option<R>,
    /// The JWT token to use in the Authorization header.
    pub jwt: Option<String>,
}

impl<F: LemmyForm> From<F> for LemmyRequest<F> {
    fn from(value: F) -> Self {
        Self {
            body: Some(value),
            jwt: None,
        }
    }
}

impl_marker_trait!(
    LemmyForm,
    [
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
        Search
    ]
);
