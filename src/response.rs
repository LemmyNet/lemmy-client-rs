use crate::utils::impl_marker_trait;
use lemmy_api_common::{
    comment::*,
    community::*,
    custom_emoji::*,
    person::*,
    post::*,
    private_message::*,
    reports::{
        comment::CommentReportResponse, post::PostReportResponse,
        private_message::PrivateMessageReportResponse,
    },
    site::*,
    tagline::*,
    LemmyErrorType, SuccessResponse,
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
        CommunityResponse,
        GetCommunityResponse,
        ListCommunitiesResponse,
        GetCommunityPendingFollowsCountResponse,
        ListCommunityPendingFollowsResponse,
        // Custom Emojis
        CustomEmojiResponse,
        // Person
        AddAdminResponse,
        BanPersonResponse,
        BannedPersonsResponse,
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
        MyUserInfo,
        ListInboxResponse,
        ListPersonContentResponse,
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
        ResolveObjectResponse,
        SearchResponse,
        SiteResponse,
        TaglineResponse,
        ListTaglinesResponse,
        ListCustomEmojisResponse,
        // Media
        ListMediaResponse
    ]
);
