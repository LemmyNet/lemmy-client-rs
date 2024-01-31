use crate::impl_marker_trait;
use cfg_if::cfg_if;
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
    lemmy_db_schema::source::login_token::LoginToken,
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
    SuccessResponse,
};
use serde::Deserialize;

cfg_if! {
    if #[cfg(feature = "leptos")] {
        pub trait LemmyResponse: leptos::Serializable + for<'de> Deserialize<'de> {}
    } else {
        pub trait LemmyResponse: for<'de> Deserialize<'de> {}
    }
}

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
    ]
);
