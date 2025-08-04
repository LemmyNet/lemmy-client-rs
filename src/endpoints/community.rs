use crate::{LemmyClient, LemmyResult};
use http::Method;
use lemmy_api_common::{
  SuccessResponse,
  community::{
    CommunityResponse,
    GetCommunity,
    GetCommunityResponse,
    GetRandomCommunity,
    ListCommunities,
    ListCommunitiesResponse,
    actions::{
      CreateCommunity,
      FollowCommunity,
      moderation::{
        AddModToCommunity,
        AddModToCommunityResponse,
        ApproveCommunityPendingFollower,
        BanFromCommunity,
        BanFromCommunityResponse,
        DeleteCommunity,
        EditCommunity,
        GetCommunityPendingFollowsCount,
        GetCommunityPendingFollowsCountResponse,
        ListCommunityPendingFollows,
        ListCommunityPendingFollowsResponse,
        RemoveCommunity,
        TransferCommunity,
      },
    },
  },
  report::{CommunityReportResponse, CreateCommunityReport, ResolveCommunityReport},
};

// TODO: Add icon and banner stuff

impl LemmyClient {
  /// Gets a community.
  ///
  /// HTTP GET /community
  pub async fn get_community(&self, data: GetCommunity) -> LemmyResult<GetCommunityResponse> {
    self
      .make_request(Method::GET, "community", data.into())
      .await
  }

  /// Creates a new community.
  ///
  /// HTTP POST /community
  pub async fn create_community(&self, data: CreateCommunity) -> LemmyResult<CommunityResponse> {
    self
      .make_request(Method::POST, "community", data.into())
      .await
  }

  /// Edits a community.
  ///
  /// HTTP PUT /community
  pub async fn update_community(&self, data: EditCommunity) -> LemmyResult<CommunityResponse> {
    self
      .make_request(Method::PUT, "community", data.into())
      .await
  }

  /// Fetches a random community.
  ///
  /// HTTP GET /community/random
  pub async fn get_random_community(
    &self,
    data: GetRandomCommunity,
  ) -> LemmyResult<CommunityResponse> {
    self
      .make_request(Method::GET, "community/random", data.into())
      .await
  }

  /// Lists communities.
  ///
  /// HTTP GET /community/list
  pub async fn list_communities(
    &self,
    data: ListCommunities,
  ) -> LemmyResult<ListCommunitiesResponse> {
    self
      .make_request(Method::GET, "community/list", data.into())
      .await
  }

  /// Subscribes to a community.
  ///
  /// HTTP POST /community/follow
  pub async fn follow_community(&self, data: FollowCommunity) -> LemmyResult<CommunityResponse> {
    self
      .make_request(Method::POST, "community/follow", data.into())
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
      .make_request(Method::POST, "community/report", data.into())
      .await
  }

  /// Resolves a community report.
  ///
  /// HTTP PUT /community/report/resolve
  pub async fn resolve_community_report(
    &self,
    data: ResolveCommunityReport,
  ) -> LemmyResult<CommunityReportResponse> {
    self
      .make_request(Method::PUT, "community/report/resolve", data.into())
      .await
  }

  /// Deletes a community.
  ///
  /// HTTP POST /community/delete
  pub async fn delete_community(&self, data: DeleteCommunity) -> LemmyResult<CommunityResponse> {
    self
      .make_request(Method::POST, "community/delete", data.into())
      .await
  }

  /// Removes a community (moderation action).
  ///
  /// HTTP POST /community/remove
  pub async fn remove_community(&self, data: RemoveCommunity) -> LemmyResult<CommunityResponse> {
    self
      .make_request(Method::POST, "community/remove", data.into())
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
      .make_request(Method::POST, "community/transfer", data.into())
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
      .make_request(Method::POST, "community/ban_user", data.into())
      .await
  }

  /// Adds a moderator to your community.
  ///
  /// HTTP POST /community/mod
  pub async fn add_mod_to_community(
    &self,
    data: AddModToCommunity,
  ) -> LemmyResult<AddModToCommunityResponse> {
    self
      .make_request(Method::POST, "community/mod", data.into())
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
      .make_request(Method::GET, "community/pending_follows/count", data.into())
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
      .make_request(Method::GET, "community/pending_follows/list", data.into())
      .await
  }

  /// Approve a pending follow for a given community.
  ///
  /// HTTP POST community/pending_follows/approve
  pub async fn approve_community_pending_follow(
    &self,
    data: ApproveCommunityPendingFollower,
  ) -> LemmyResult<SuccessResponse> {
    self
      .make_request(
        Method::POST,
        "community/pending_follows/approve",
        data.into(),
      )
      .await
  }
}
