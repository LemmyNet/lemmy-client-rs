use crate::utils::impl_marker_trait;
use lemmy_api_common::{
    comment::*, community::*, custom_emoji::*, lemmy_db_schema::source::login_token::LoginToken,
    person::*, post::*, private_message::*, site::*, LemmyErrorType, SuccessResponse,
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
        CreateCommentReport,
        GetCommentsResponse,
        ListCommentLikesResponse,
        ListCommentReportsResponse,
        // Communities
        AddModToCommunityResponse,
        BanFromCommunityResponse,
        BlockCommunityResponse,
        CommunityResponse,
        GetCommunityResponse,
        ListCommunitiesResponse,
        // Custom Emojis
        CustomEmojiResponse,
        // Person
        AddAdminResponse,
        BanPersonResponse,
        BannedPersonsResponse,
        BlockPersonResponse,
        CaptchaResponse,
        CommentReplyResponse,
        GenerateTotpSecretResponse,
        GetCaptchaResponse,
        GetPersonDetailsResponse,
        GetPersonMentionsResponse,
        GetRepliesResponse,
        GetReportCountResponse,
        GetUnreadCountResponse,
        LoginResponse,
        PersonMentionResponse,
        UpdateTotpResponse,
        Vec<LoginToken>,
        // Posts
        GetPostResponse,
        GetPostsResponse,
        GetSiteMetadataResponse,
        ListPostLikesResponse,
        ListPostReportsResponse,
        PostReportResponse,
        PostResponse,
        // Private Messages
        ListPrivateMessageReportsResponse,
        PrivateMessageReportResponse,
        PrivateMessageResponse,
        PrivateMessagesResponse,
        // Site
        BlockInstanceResponse,
        GetFederatedInstancesResponse,
        GetModlogResponse,
        GetSiteResponse,
        GetUnreadRegistrationApplicationCountResponse,
        ListRegistrationApplicationsResponse,
        RegistrationApplicationResponse,
        ResolveObjectResponse,
        SearchResponse,
        SiteResponse,
        // Media
        ListMediaResponse
    ]
);
