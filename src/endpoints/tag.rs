use crate::{LemmyClient, LemmyResult};
use http::Method;
use lemmy_api_common::community::{
  Tag,
  actions::moderation::{CreateCommunityTag, DeleteCommunityTag, UpdateCommunityTag},
};

impl LemmyClient {
  /// Create a tag for a community you moderate.
  ///
  /// HTTP POST /community/tag
  pub async fn create_community_tag(&self, data: CreateCommunityTag) -> LemmyResult<Tag> {
    self
      .make_request(Method::POST, "community/tag", data.into())
      .await
  }

  /// Update an existing tag for a community you moderate.
  ///
  /// HTTP PUT /community/tag
  pub async fn update_community_tag(&self, data: UpdateCommunityTag) -> LemmyResult<Tag> {
    self
      .make_request(Method::PUT, "community/tag", data.into())
      .await
  }

  /// Delete an existing tag for a community you moderate.
  ///
  /// HTTP DELETE /community/tag
  pub async fn delete_community_tag(&self, data: DeleteCommunityTag) -> LemmyResult<Tag> {
    self
      .make_request(Method::DELETE, "community/tag", data.into())
      .await
  }
}
