use crate::macros::impl_marker_trait;
use lemmy_api_common::{
  SuccessResponse,
  account::{auth::*, *},
  comment::{actions::moderation::*, *},
  community::{
    actions::{moderation::*, *},
    *,
  },
  custom_emoji::*,
  error::LemmyErrorType,
  federation::*,
  media::*,
  modlog::*,
  notification::*,
  oauth::*,
  person::{
    actions::{moderation::*, *},
    *,
  },
  post::{actions::moderation::*, *},
  private_message::*,
  report::*,
  search::*,
  site::{administration::*, *},
  tagline::*,
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
