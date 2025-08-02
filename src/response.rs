use crate::utils::impl_marker_trait;
use lemmy_api_common::{
  SuccessResponse,
  account::{
    ListPersonHiddenResponse,
    ListPersonLikedResponse,
    ListPersonReadResponse,
    ListPersonSavedResponse,
    MyUserInfo,
    auth::{
      CaptchaResponse,
      ExportDataResponse,
      GenerateTotpSecretResponse,
      GetCaptchaResponse,
      ListLoginsResponse,
      LoginResponse,
      UpdateTotpResponse,
      UserSettingsBackup,
    },
  },
  comment::{
    CommentResponse,
    GetCommentsResponse,
    GetCommentsSlimResponse,
    actions::moderation::ListCommentLikesResponse,
  },
  community::{
    CommunityResponse,
    GetCommunityResponse,
    GetMultiCommunityResponse,
    ListCommunitiesResponse,
    ListMultiCommunitiesResponse,
    Tag,
    actions::{
      BlockCommunityResponse,
      moderation::{
        AddModToCommunityResponse,
        BanFromCommunityResponse,
        GetCommunityPendingFollowsCountResponse,
        ListCommunityPendingFollowsResponse,
      },
    },
  },
  custom_emoji::{CustomEmojiResponse, ListCustomEmojisResponse},
  error::LemmyErrorType,
  federation::GetFederatedInstancesResponse,
  media::{ImageGetParams, ListMediaResponse, UploadImageResponse},
  modlog::GetModlogResponse,
  notification::{GetUnreadCountResponse, ListNotificationsResponse},
  oauth::OAuthProvider,
  person::{
    GetPersonDetailsResponse,
    actions::{
      BlockPersonResponse,
      ListPersonContentResponse,
      moderation::{BanPersonResponse, RegistrationApplicationResponse},
    },
  },
  post::{
    GetPostResponse,
    GetPostsResponse,
    GetSiteMetadataResponse,
    PostResponse,
    actions::moderation::ListPostLikesResponse,
  },
  private_message::PrivateMessageResponse,
  report::{
    CommentReportResponse,
    CommunityReportResponse,
    GetReportCountResponse,
    ListReportsResponse,
    PostReportResponse,
    PrivateMessageReportResponse,
  },
  search::SearchResponse,
  site::{
    GetSiteResponse,
    SiteResponse,
    administration::{
      AddAdminResponse,
      AdminListUsersResponse,
      GetUnreadRegistrationApplicationCountResponse,
      ListRegistrationApplicationsResponse,
    },
  },
  tagline::{ListTaglinesResponse, TaglineResponse},
};
use serde::Deserialize;

pub trait LemmyResponse: for<'de> Deserialize<'de> {}

pub type LemmyResult<R> = Result<R, LemmyErrorType>;

impl_marker_trait!(
  LemmyResponse,
  [
    String,
    SuccessResponse,
    // Comments
    CommentReportResponse,
    CommentResponse,
    GetCommentsResponse,
    GetCommentsSlimResponse,
    ListCommentLikesResponse,
    // Communities
    AddModToCommunityResponse,
    BanFromCommunityResponse,
    BlockCommunityResponse,
    CommunityReportResponse,
    CommunityResponse,
    GetCommunityResponse,
    ListCommunitiesResponse,
    GetCommunityPendingFollowsCountResponse,
    ListCommunityPendingFollowsResponse,
    ListReportsResponse,
    Tag,
    GetMultiCommunityResponse,
    ListMultiCommunitiesResponse,
    // Custom Emojis
    CustomEmojiResponse,
    ListCustomEmojisResponse,
    // Person
    AddAdminResponse,
    BanPersonResponse,
    BlockPersonResponse,
    CaptchaResponse,
    GenerateTotpSecretResponse,
    GetCaptchaResponse,
    GetPersonDetailsResponse,
    GetReportCountResponse,
    GetUnreadCountResponse,
    LoginResponse,
    UpdateTotpResponse,
    ListLoginsResponse,
    ListPersonSavedResponse,
    ListPersonReadResponse,
    ListPersonHiddenResponse,
    ListPersonLikedResponse,
    MyUserInfo,
    ListNotificationsResponse,
    ListPersonContentResponse,
    AdminListUsersResponse,
    UserSettingsBackup,
    ExportDataResponse,
    // Posts
    GetPostResponse,
    GetPostsResponse,
    GetSiteMetadataResponse,
    ListPostLikesResponse,
    PostReportResponse,
    PostResponse,
    // Private Messages
    PrivateMessageReportResponse,
    PrivateMessageResponse,
    // Site
    GetFederatedInstancesResponse,
    GetModlogResponse,
    GetSiteResponse,
    GetUnreadRegistrationApplicationCountResponse,
    ListRegistrationApplicationsResponse,
    RegistrationApplicationResponse,
    SearchResponse,
    SiteResponse,
    TaglineResponse,
    ListTaglinesResponse,
    // Media
    ListMediaResponse,
    UploadImageResponse,
    ImageGetParams,
    //OAuth
    OAuthProvider
  ]
);
