use lemmy_api_common::{
    comment::{
        CommentReportResponse, CommentResponse, CreateCommentReport, GetCommentsResponse,
        ListCommentLikesResponse, ListCommentReportsResponse,
    },
    community::{
        AddModToCommunityResponse, BanFromCommunityResponse, BlockCommunityResponse,
        CommunityResponse, GetCommunityResponse, ListCommunitiesResponse,
    },
    custom_emoji::CustomEmojiResponse,
    person::{
        AddAdminResponse, BanPersonResponse, BannedPersonsResponse, BlockPersonResponse,
        CaptchaResponse, CommentReplyResponse, GenerateTotpSecretResponse, GetCaptchaResponse,
        GetPersonDetailsResponse, GetPersonMentionsResponse, GetRepliesResponse,
        GetReportCountResponse, GetUnreadCountResponse, LoginResponse, PersonMentionResponse,
        UpdateTotpResponse,
    },
    post::{
        GetPostResponse, GetPostsResponse, GetSiteMetadataResponse, ListPostLikesResponse,
        ListPostReportsResponse, PostReportResponse, PostResponse,
    },
    private_message::{
        ListPrivateMessageReportsResponse, PrivateMessageReportResponse, PrivateMessageResponse,
        PrivateMessagesResponse,
    },
    site::{
        BlockInstanceResponse, GetFederatedInstancesResponse, GetModlogResponse, GetSiteResponse,
        GetUnreadRegistrationApplicationCountResponse, ListRegistrationApplicationsResponse,
        RegistrationApplicationResponse, ResolveObjectResponse, SearchResponse, SiteResponse,
    },
};
use serde::Deserialize;

use crate::impl_marker_trait;

pub(crate) trait LemmyResponse: for<'de> Deserialize<'de> {}

impl_marker_trait!(
    LemmyResponse,
    [
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
    ]
);
