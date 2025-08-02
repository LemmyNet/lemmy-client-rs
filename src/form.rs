use crate::macros::impl_marker_trait;
use lemmy_api_common::{
  account::{auth::*, *},
  comment::{
    actions::{moderation::*, *},
    *,
  },
  community::{
    actions::{moderation::*, *},
    *,
  },
  custom_emoji::*,
  federation::{administration::*, *},
  media::*,
  modlog::*,
  notification::*,
  oauth::*,
  person::{
    actions::{moderation::*, *},
    *,
  },
  post::{
    actions::{moderation::*, *},
    *,
  },
  private_message::actions::*,
  report::*,
  search::*,
  site::administration::*,
  tagline::{aministration::*, *},
};
use serde::Serialize;
use std::fmt;

pub trait LemmyForm: Serialize + Clone + fmt::Debug {}

#[derive(Debug, Clone)]
/// A request to send to lemmy. If you don't want to set the JWT for each request, you can set the
/// Authorization header with [`LemmyClient::headers_mut`](lemmy_client::LemmyClient.headers_mut).
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
        &'static [u8],
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
        RemoveComment,
        ResolveCommentReport,
        SaveComment,
        // Communities
        AddModToCommunity,
        BanFromCommunity,
        BlockCommunity,
        CreateCommunity,
        CreateCommunityReport,
        CommunityIdQuery,
        ResolveCommunityReport,
        DeleteCommunity,
        EditCommunity,
        FollowCommunity,
        GetCommunity,
        HideCommunity,
        ListCommunities,
        RemoveCommunity,
        TransferCommunity,
        GetRandomCommunity,
        GetCommunityPendingFollowsCount,
        ListCommunityPendingFollows,
        ApproveCommunityPendingFollower,
        CreateCommunityTag,
        DeleteCommunityTag,
        CreateMultiCommunity,
        UpdateMultiCommunity,
        GetMultiCommunity,
        CreateOrDeleteMultiCommunityEntry,
        ListMultiCommunities,
        FollowMultiCommunity,
        UpdateCommunityNotifications,
        UpdateCommunityTag,
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
        GetReportCount,
        Login,
        PasswordChangeAfterReset,
        PasswordReset,
        Register,
        SaveUserSettings,
        UpdateTotp,
        VerifyEmail,
        ResendVerificationEmail,
        ListPersonSaved,
        ListPersonRead,
        ListPersonHidden,
        ListPersonLiked,
        ListNotifications,
        MarkNotificationAsRead,
        UserBlockInstanceCommunitiesParams,
        UserBlockInstancePersonsParams,
        ListPersonContent,
        ListReports,
        NotePerson,
        AdminListUsers,
        UserSettingsBackup,
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
        LockPost,
        MarkPostAsRead,
        MarkManyPostsAsRead,
        RemovePost,
        ResolvePostReport,
        SavePost,
        HidePost,
        ModEditPost,
        // Private Messages
        CreatePrivateMessage,
        CreatePrivateMessageReport,
        DeletePrivateMessage,
        EditPrivateMessage,
        ResolvePrivateMessageReport,
        // Site
        ApproveRegistrationApplication,
        GetRegistrationApplication,
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
        CreateTagline,
        UpdateTagline,
        DeleteTagline,
        ListTaglines,
        AdminBlockInstanceParams,
        AdminAllowInstanceParams,
        ListCustomEmojis,
        // Media
        ListMedia,
        DeleteImageParams,
        ImageProxyParams,
        // OAuth
        CreateOAuthProvider,
        EditOAuthProvider,
        DeleteOAuthProvider,
        AuthenticateWithOauth
    ]
);
