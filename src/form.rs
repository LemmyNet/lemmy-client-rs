use crate::macros::impl_marker_trait;
use lemmy_api_common::{
  account::{
    DeleteAccount,
    ListPersonHidden,
    ListPersonLiked,
    ListPersonRead,
    ListPersonSaved,
    SaveUserSettings,
    auth::{
      ChangePassword,
      Login,
      PasswordChangeAfterReset,
      PasswordReset,
      Register,
      ResendVerificationEmail,
      UpdateTotp,
      UserSettingsBackup,
      VerifyEmail,
    },
  },
  comment::{
    GetComment,
    GetComments,
    actions::{
      CreateComment,
      CreateCommentLike,
      DeleteComment,
      EditComment,
      SaveComment,
      moderation::{DistinguishComment, ListCommentLikes, PurgeComment, RemoveComment},
    },
  },
  community::{
    CreateMultiCommunity,
    CreateOrDeleteMultiCommunityEntry,
    FollowMultiCommunity,
    GetCommunity,
    GetMultiCommunity,
    GetRandomCommunity,
    ListCommunities,
    ListMultiCommunities,
    UpdateCommunityNotifications,
    UpdateMultiCommunity,
    actions::{
      BlockCommunity,
      CreateCommunity,
      FollowCommunity,
      HideCommunity,
      moderation::{
        AddModToCommunity,
        ApproveCommunityPendingFollower,
        BanFromCommunity,
        CommunityIdQuery,
        CreateCommunityTag,
        DeleteCommunity,
        DeleteCommunityTag,
        EditCommunity,
        GetCommunityPendingFollowsCount,
        ListCommunityPendingFollows,
        PurgeCommunity,
        RemoveCommunity,
        TransferCommunity,
        UpdateCommunityTag,
      },
    },
  },
  custom_emoji::{CreateCustomEmoji, DeleteCustomEmoji, EditCustomEmoji, ListCustomEmojis},
  federation::{
    FederatedInstances,
    InstanceWithFederationState,
    ResolveObject,
    UserBlockInstanceCommunitiesParams,
    UserBlockInstancePersonsParams,
    administration::{AdminAllowInstanceParams, AdminBlockInstanceParams},
  },
  media::{DeleteImageParams, ImageProxyParams, ListMedia},
  modlog::GetModlog,
  notification::{ListNotifications, MarkNotificationAsRead},
  oauth::{AuthenticateWithOauth, CreateOAuthProvider, DeleteOAuthProvider, EditOAuthProvider},
  person::{
    GetPersonDetails,
    actions::{
      BlockPerson,
      ListPersonContent,
      NotePerson,
      moderation::{BanPerson, GetRegistrationApplication, PurgePerson},
    },
  },
  post::{
    GetPost,
    GetPosts,
    GetSiteMetadata,
    actions::{
      CreatePost,
      CreatePostLike,
      DeletePost,
      EditPost,
      HidePost,
      MarkManyPostsAsRead,
      MarkPostAsRead,
      SavePost,
      moderation::{FeaturePost, ListPostLikes, LockPost, ModEditPost, PurgePost, RemovePost},
    },
  },
  private_message::actions::{CreatePrivateMessage, DeletePrivateMessage, EditPrivateMessage},
  report::{
    CreateCommentReport,
    CreateCommunityReport,
    CreatePostReport,
    CreatePrivateMessageReport,
    GetReportCount,
    ListReports,
    ResolveCommentReport,
    ResolveCommunityReport,
    ResolvePostReport,
    ResolvePrivateMessageReport,
  },
  search::Search,
  site::administration::{
    AddAdmin,
    AdminListUsers,
    ApproveRegistrationApplication,
    CreateSite,
    EditSite,
    ListRegistrationApplications,
  },
  tagline::{
    ListTaglines,
    aministration::{CreateTagline, DeleteTagline, UpdateTagline},
  },
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
