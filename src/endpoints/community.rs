use crate::{LemmyClient, LemmyResult};
use http::Method;
use lemmy_api_common::{
  SuccessResponse,
  community::{
    CommunityResponse,
    CreateMultiCommunity,
    CreateOrDeleteMultiCommunityEntry,
    FollowMultiCommunity,
    GetCommunity,
    GetCommunityResponse,
    GetMultiCommunity,
    GetMultiCommunityResponse,
    GetRandomCommunity,
    ListCommunities,
    ListCommunitiesResponse,
    ListMultiCommunities,
    ListMultiCommunitiesResponse,
    Tag,
    UpdateCommunityNotifications,
    UpdateMultiCommunity,
    actions::{
      CreateCommunity,
      FollowCommunity,
      moderation::{
        AddModToCommunity,
        AddModToCommunityResponse,
        ApproveCommunityPendingFollower,
        BanFromCommunity,
        BanFromCommunityResponse,
        CommunityIdQuery,
        CreateCommunityTag,
        DeleteCommunity,
        DeleteCommunityTag,
        EditCommunity,
        GetCommunityPendingFollowsCount,
        GetCommunityPendingFollowsCountResponse,
        ListCommunityPendingFollows,
        ListCommunityPendingFollowsResponse,
        RemoveCommunity,
        TransferCommunity,
        UpdateCommunityTag,
      },
    },
  },
  media::UploadImageResponse,
};
use reqwest::Body;

// TODO: Add icon and banner stuff

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
  ) -> LemmyResult<ListCommunitiesResponse> {
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

  /// Deletes a community.
  ///
  /// HTTP POST /community/delete
  pub async fn delete_community(&self, data: DeleteCommunity) -> LemmyResult<CommunityResponse> {
    self
      .make_request(Method::POST, "community/delete", data)
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
  pub async fn ban_from_community(
    &self,
    data: BanFromCommunity,
  ) -> LemmyResult<BanFromCommunityResponse> {
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
  pub async fn create_community_tag(&self, data: CreateCommunityTag) -> LemmyResult<Tag> {
    self.make_request(Method::POST, "community/tag", data).await
  }

  /// Update an existing tag for a community you moderate.
  ///
  /// HTTP PUT /community/tag
  pub async fn update_community_tag(&self, data: UpdateCommunityTag) -> LemmyResult<Tag> {
    self.make_request(Method::PUT, "community/tag", data).await
  }

  /// Delete an existing tag for a community you moderate.
  ///
  /// HTTP DELETE /community/tag
  pub async fn delete_community_tag(&self, data: DeleteCommunityTag) -> LemmyResult<Tag> {
    self
      .make_request(Method::DELETE, "community/tag", data)
      .await
  }

  /// Set which notifications you want to receive for a community.
  ///
  /// HTTP POST /community/notifications
  pub async fn update_community_notifications(
    &self,
    data: UpdateCommunityNotifications,
  ) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "community/notifications", data)
      .await
  }

  /// Gets number of pending follows for a given community.
  ///
  /// HTTP GET /community/pending_follows/count
  pub async fn get_community_pending_follows_count(
    &self,
    data: GetCommunityPendingFollowsCount,
  ) -> LemmyResult<GetCommunityPendingFollowsCountResponse> {
    self
      .make_request(Method::GET, "community/pending_follows/count", data)
      .await
  }

  /// Gets list of pending follows for a given community.
  ///
  /// HTTP GET /community/pending_follows/list
  pub async fn list_community_pending_follows(
    &self,
    data: ListCommunityPendingFollows,
  ) -> LemmyResult<ListCommunityPendingFollowsResponse> {
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

  /// Create a multi community.
  ///
  /// HTTP POST /multi_community
  pub async fn create_multi_community(
    &self,
    data: CreateMultiCommunity,
  ) -> LemmyResult<GetMultiCommunityResponse> {
    self
      .make_request(Method::POST, "multi_community", data)
      .await
  }

  /// Update a multi community.
  ///
  /// HTTP PUT /multi_community
  pub async fn edit_multi_community(
    &self,
    data: UpdateMultiCommunity,
  ) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::PUT, "multi_community", data)
      .await
  }

  /// Get a specific multi community.
  ///
  /// HTTP GET /multi_community
  pub async fn get_multi_community(
    &self,
    data: GetMultiCommunity,
  ) -> LemmyResult<GetMultiCommunityResponse> {
    self
      .make_request(Method::GET, "multi_community", data)
      .await
  }

  /// Add a community to a multi community.
  ///
  /// HTTP POST /multi_community/entry
  pub async fn add_multi_community_entry(
    &self,
    data: CreateOrDeleteMultiCommunityEntry,
  ) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "multi_community/entry", data)
      .await
  }

  /// Remove a community from a multi community.
  ///
  /// HTTP DELETE /multi_community/entry
  pub async fn remove_multi_community_entry(
    &self,
    data: CreateOrDeleteMultiCommunityEntry,
  ) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::DELETE, "multi_community/entry", data)
      .await
  }

  /// List multi communities.
  ///
  /// HTTP GET /multi_community/list
  pub async fn list_multi_communities(
    &self,
    data: ListMultiCommunities,
  ) -> LemmyResult<ListMultiCommunitiesResponse> {
    self
      .make_request(Method::GET, "multi_community/list", data)
      .await
  }

  /// Remove a community from a multi community.
  ///
  /// HTTP POST /multi_community/follow
  pub async fn follow_multi_community(
    &self,
    data: FollowMultiCommunity,
  ) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "multi_community/follow", data)
      .await
  }
}
