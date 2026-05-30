use crate::{LemmyClient, LemmyResult};
use http::Method;
use lemmy_api_common::{
  PagedResponse,
  SuccessResponse,
  community::{
    CommunityResponse,
    CommunityTag,
    CommunityView,
    EditCommunityNotifications,
    GetCommunity,
    GetCommunityResponse,
    GetRandomCommunity,
    ListCommunities,
    PendingFollowerView,
    actions::{
      CreateCommunity,
      FollowCommunity,
      moderation::{
        AddModToCommunity,
        AddModToCommunityResponse,
        ApproveCommunityPendingFollower,
        BanFromCommunity,
        CommunityIdQuery,
        CreateCommunityTag,
        DeleteCommunity,
        DeleteCommunityTag,
        EditCommunity,
        EditCommunityTag,
        ListCommunityPendingFollows,
        RemoveCommunity,
        TransferCommunity,
      },
    },
  },
  media::UploadImageResponse,
  person::PersonResponse,
  report::{CommunityReportResponse, CreateCommunityReport},
};
use reqwest::Body;

impl LemmyClient {
  /// Gets a community.
  ///
  /// HTTP GET /community
  pub async fn get_community(&self, data: GetCommunity) -> LemmyResult<GetCommunityResponse> {
    self.make_request(Method::GET, "community", data).await
  }

  /// Creates a new community.
  ///
  /// HTTP POST /community
  pub async fn create_community(&self, data: CreateCommunity) -> LemmyResult<CommunityResponse> {
    self.make_request(Method::POST, "community", data).await
  }

  /// Edits a community.
  ///
  /// HTTP PUT /community
  pub async fn update_community(&self, data: EditCommunity) -> LemmyResult<CommunityResponse> {
    self.make_request(Method::PUT, "community", data).await
  }

  /// Deletes a community.
  ///
  /// HTTP DELETE /community
  pub async fn delete_community(&self, data: DeleteCommunity) -> LemmyResult<CommunityResponse> {
    self.make_request(Method::DELETE, "community", data).await
  }

  /// Fetches a random community.
  ///
  /// HTTP GET /community/random
  pub async fn get_random_community(
    &self,
    data: GetRandomCommunity,
  ) -> LemmyResult<CommunityResponse> {
    self
      .make_request(Method::GET, "community/random", data)
      .await
  }

  /// Lists communities.
  ///
  /// HTTP GET /community/list
  pub async fn list_communities(
    &self,
    data: ListCommunities,
  ) -> LemmyResult<PagedResponse<CommunityView>> {
    self.make_request(Method::GET, "community/list", data).await
  }

  /// Subscribes to a community.
  ///
  /// HTTP POST /community/follow
  pub async fn follow_community(&self, data: FollowCommunity) -> LemmyResult<CommunityResponse> {
    self
      .make_request(Method::POST, "community/follow", data)
      .await
  }

  /// Report a community.
  ///
  /// HTTP POST /community/report
  pub async fn report_community(
    &self,
    data: CreateCommunityReport,
  ) -> LemmyResult<CommunityReportResponse> {
    self
      .make_request(Method::POST, "community/report", data)
      .await
  }

  /// Resolves a community report.
  ///
  /// HTTP PUT /community/report/resolve
  pub async fn resolve_community_report(
    &self,
    data: CreateCommunityReport,
  ) -> LemmyResult<CommunityReportResponse> {
    self
      .make_request(Method::PUT, "community/report/resolve", data)
      .await
  }

  /// Removes a community (moderation action).
  ///
  /// HTTP POST /community/remove
  pub async fn remove_community(&self, data: RemoveCommunity) -> LemmyResult<CommunityResponse> {
    self
      .make_request(Method::POST, "community/remove", data)
      .await
  }

  /// Transfers a community you own to another user on that community's moderation team.
  ///
  /// HTTP POST community/transfer
  pub async fn transfer_community(
    &self,
    data: TransferCommunity,
  ) -> LemmyResult<GetCommunityResponse> {
    self
      .make_request(Method::POST, "community/transfer", data)
      .await
  }

  /// Bans a user from a community.
  ///
  /// HTTP POST /community/ban_user
  pub async fn ban_from_community(&self, data: BanFromCommunity) -> LemmyResult<PersonResponse> {
    self
      .make_request(Method::POST, "community/ban_user", data)
      .await
  }

  /// Adds a moderator to your community.
  ///
  /// HTTP POST /community/mod
  pub async fn add_mod_to_community(
    &self,
    data: AddModToCommunity,
  ) -> LemmyResult<AddModToCommunityResponse> {
    self.make_request(Method::POST, "community/mod", data).await
  }

  /// Uploads an icon to represent a community.
  ///
  /// HTTP POST /community/icon
  pub async fn upload_community_icon(
    &self,
    query: CommunityIdQuery,
    body: impl Into<Body>,
  ) -> LemmyResult<UploadImageResponse> {
    self.make_file_request("community/icon", query, body).await
  }

  /// Deletes the icon used by a community.
  ///
  /// HTTP DELETE /community/icon
  pub async fn delete_community_icon(
    &self,
    request: CommunityIdQuery,
  ) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::DELETE, "community/icon", request)
      .await
  }

  /// Uploads a banner to add flair to a community
  ///
  /// HTTP POST /community/banner
  pub async fn upload_community_banner(
    &self,
    query: CommunityIdQuery,
    body: impl Into<Body>,
  ) -> LemmyResult<UploadImageResponse> {
    self
      .make_file_request("community/banner", query, body)
      .await
  }

  /// Deletes the banner used by a community.
  ///
  /// HTTP DELETE /community/banner
  pub async fn delete_community_banner(
    &self,
    request: CommunityIdQuery,
  ) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::DELETE, "community/banner", request)
      .await
  }

  /// Create a tag for a community you moderate.
  ///
  /// HTTP POST /community/tag
  pub async fn create_community_tag(&self, data: CreateCommunityTag) -> LemmyResult<CommunityTag> {
    self.make_request(Method::POST, "community/tag", data).await
  }

  /// Update an existing tag for a community you moderate.
  ///
  /// HTTP PUT /community/tag
  pub async fn edit_community_tag(&self, data: EditCommunityTag) -> LemmyResult<CommunityTag> {
    self.make_request(Method::PUT, "community/tag", data).await
  }

  /// Delete an existing tag for a community you moderate.
  ///
  /// HTTP DELETE /community/tag
  pub async fn delete_community_tag(&self, data: DeleteCommunityTag) -> LemmyResult<CommunityTag> {
    self
      .make_request(Method::DELETE, "community/tag", data)
      .await
  }

  /// Set which notifications you want to receive for a community.
  ///
  /// HTTP POST /community/notifications
  pub async fn edit_community_notifications(
    &self,
    data: EditCommunityNotifications,
  ) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "community/notifications", data)
      .await
  }

  /// Gets list of pending follows for a given community.
  ///
  /// HTTP GET /community/pending_follows/list
  pub async fn list_community_pending_follows(
    &self,
    data: ListCommunityPendingFollows,
  ) -> LemmyResult<PagedResponse<PendingFollowerView>> {
    self
      .make_request(Method::GET, "community/pending_follows/list", data)
      .await
  }

  /// Approve a pending follow for a given community.
  ///
  /// HTTP POST /community/pending_follows/approve
  pub async fn approve_community_pending_follow(
    &self,
    data: ApproveCommunityPendingFollower,
  ) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "community/pending_follows/approve", data)
      .await
  }
}
