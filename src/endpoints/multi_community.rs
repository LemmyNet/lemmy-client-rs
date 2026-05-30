use crate::{LemmyClient, LemmyResult};
use http::Method;
use lemmy_api_common::{
  PagedResponse,
  SuccessResponse,
  community::{
    CreateMultiCommunity,
    CreateOrDeleteMultiCommunityEntry,
    EditMultiCommunity,
    FollowMultiCommunity,
    GetMultiCommunity,
    GetMultiCommunityResponse,
    ListMultiCommunities,
    MultiCommunityView,
  },
};

impl LemmyClient {
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
    data: EditMultiCommunity,
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
  ) -> LemmyResult<PagedResponse<MultiCommunityView>> {
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
