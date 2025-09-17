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
    ResolveObject,
    UserBlockInstanceCommunitiesParams,
    UserBlockInstancePersonsParams,
    administration::{AdminAllowInstanceParams, AdminBlockInstanceParams},
  },
  media::{DeleteImageParams, ListMedia},
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
      moderation::{FeaturePost, ListPostLikes, LockPost, PurgePost, RemovePost},
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
    administration::{CreateTagline, DeleteTagline, UpdateTagline},
  },
};
use serde::Serialize;
use std::fmt;

struct LemmyRequest<'jwt, Data>
where
  Data: LemmyRequestData,
{
  data: Data,
  jwt: Option<&'jwt str>,
}

trait LemmyRequestData: Serialize + Clone + fmt::Debug {}

impl<'jwt, Data> From<Data> for LemmyRequest<'jwt, Data>
where
  Data: LemmyRequestData,
{
  fn from(data: Data) -> Self {
    Self { data, jwt: None }
  }
}

impl LemmyRequestData for () {}

// Account
impl LemmyRequestData for Register {}
impl LemmyRequestData for Login {}
impl LemmyRequestData for PasswordReset {}
impl LemmyRequestData for PasswordChangeAfterReset {}
impl LemmyRequestData for ChangePassword {}
impl LemmyRequestData for UpdateTotp {}
impl LemmyRequestData for VerifyEmail {}
impl LemmyRequestData for ResendVerificationEmail {}
impl LemmyRequestData for ListNotifications {}
impl LemmyRequestData for DeleteAccount {}
impl LemmyRequestData for MarkNotificationAsRead {}
impl LemmyRequestData for GetReportCount {}
impl LemmyRequestData for BlockPerson {}
impl LemmyRequestData for BlockCommunity {}
impl LemmyRequestData for UserBlockInstanceCommunitiesParams {}
impl LemmyRequestData for UserBlockInstancePersonsParams {}
impl LemmyRequestData for ListPersonSaved {}
impl LemmyRequestData for ListPersonRead {}
impl LemmyRequestData for ListPersonHidden {}
impl LemmyRequestData for ListPersonLiked {}
impl LemmyRequestData for SaveUserSettings {}
impl LemmyRequestData for UserSettingsBackup {}

// Administration
impl LemmyRequestData for AddAdmin {}
impl LemmyRequestData for ListRegistrationApplications {}
impl LemmyRequestData for ApproveRegistrationApplication {}
impl LemmyRequestData for GetRegistrationApplication {}
impl LemmyRequestData for PurgePerson {}
impl LemmyRequestData for PurgeCommunity {}
impl LemmyRequestData for PurgePost {}
impl LemmyRequestData for PurgeComment {}
impl LemmyRequestData for CreateTagline {}
impl LemmyRequestData for UpdateTagline {}
impl LemmyRequestData for DeleteTagline {}
impl LemmyRequestData for ListTaglines {}
impl LemmyRequestData for BanPerson {}
impl LemmyRequestData for AdminListUsers {}
impl LemmyRequestData for AdminBlockInstanceParams {}
impl LemmyRequestData for AdminAllowInstanceParams {}
impl LemmyRequestData for ListMedia {}

// Comments
impl LemmyRequestData for GetComment {}
impl LemmyRequestData for CreateComment {}
impl LemmyRequestData for EditComment {}
impl LemmyRequestData for DeleteComment {}
impl LemmyRequestData for RemoveComment {}
impl LemmyRequestData for DistinguishComment {}
impl LemmyRequestData for CreateCommentLike {}
impl LemmyRequestData for ListCommentLikes {}
impl LemmyRequestData for SaveComment {}
impl LemmyRequestData for GetComments {}
impl LemmyRequestData for CreateCommentReport {}
impl LemmyRequestData for ResolveCommentReport {}

// Communities
impl LemmyRequestData for GetCommunity {}
impl LemmyRequestData for CreateCommunity {}
impl LemmyRequestData for EditCommunity {}
impl LemmyRequestData for GetRandomCommunity {}
impl LemmyRequestData for ListCommunities {}
impl LemmyRequestData for FollowCommunity {}
impl LemmyRequestData for DeleteCommunity {}
impl LemmyRequestData for RemoveCommunity {}
impl LemmyRequestData for TransferCommunity {}
impl LemmyRequestData for BanFromCommunity {}
impl LemmyRequestData for AddModToCommunity {}
impl LemmyRequestData for CommunityIdQuery {}
impl LemmyRequestData for CreateCommunityTag {}
impl LemmyRequestData for UpdateCommunityTag {}
impl LemmyRequestData for DeleteCommunityTag {}
impl LemmyRequestData for UpdateCommunityNotifications {}
impl LemmyRequestData for GetCommunityPendingFollowsCount {}
impl LemmyRequestData for ListCommunityPendingFollows {}
impl LemmyRequestData for ApproveCommunityPendingFollower {}
impl LemmyRequestData for CreateMultiCommunity {}
impl LemmyRequestData for UpdateMultiCommunity {}
impl LemmyRequestData for GetMultiCommunity {}
impl LemmyRequestData for CreateOrDeleteMultiCommunityEntry {}
impl LemmyRequestData for ListMultiCommunities {}
impl LemmyRequestData for FollowMultiCommunity {}

// Emojis
impl LemmyRequestData for CreateCustomEmoji {}
impl LemmyRequestData for EditCustomEmoji {}
impl LemmyRequestData for DeleteCustomEmoji {}
impl LemmyRequestData for ListCustomEmojis {}

// Persons
impl LemmyRequestData for GetPersonDetails {}
impl LemmyRequestData for ListPersonContent {}
impl LemmyRequestData for NotePerson {}

// Posts
impl LemmyRequestData for GetPost {}
impl LemmyRequestData for CreatePost {}
impl LemmyRequestData for EditPost {}
impl LemmyRequestData for GetSiteMetadata {}
impl LemmyRequestData for DeletePost {}
impl LemmyRequestData for RemovePost {}
impl LemmyRequestData for MarkPostAsRead {}
impl LemmyRequestData for MarkManyPostsAsRead {}
impl LemmyRequestData for HidePost {}
impl LemmyRequestData for LockPost {}
impl LemmyRequestData for FeaturePost {}
impl LemmyRequestData for GetPosts {}
impl LemmyRequestData for CreatePostLike {}
impl LemmyRequestData for ListPostLikes {}
impl LemmyRequestData for SavePost {}
impl LemmyRequestData for CreatePostReport {}
impl LemmyRequestData for ResolvePostReport {}

// Private Messages
impl LemmyRequestData for CreatePrivateMessage {}
impl LemmyRequestData for EditPrivateMessage {}
impl LemmyRequestData for DeletePrivateMessage {}
impl LemmyRequestData for CreatePrivateMessageReport {}
impl LemmyRequestData for ResolvePrivateMessageReport {}

// Reports
impl LemmyRequestData for ListReports {}
impl LemmyRequestData for CreateCommunityReport {}
impl LemmyRequestData for ResolveCommunityReport {}

// Site
impl LemmyRequestData for CreateSite {}
impl LemmyRequestData for EditSite {}
impl LemmyRequestData for DeleteImageParams {}
impl LemmyRequestData for GetModlog {}
impl LemmyRequestData for Search {}
impl LemmyRequestData for ResolveObject {}

// OAuth
impl LemmyRequestData for CreateOAuthProvider {}
impl LemmyRequestData for EditOAuthProvider {}
impl LemmyRequestData for DeleteOAuthProvider {}
impl LemmyRequestData for AuthenticateWithOauth {}
