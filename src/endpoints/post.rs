use crate::{LemmyClient, LemmyResult};
use http::Method;
use lemmy_api_common::{
  PagedResponse,
  SuccessResponse,
  VoteView,
  post::{
    CreatePostWarning,
    GetPost,
    GetPostResponse,
    GetPosts,
    GetSiteMetadata,
    GetSiteMetadataResponse,
    PostResponse,
    PostView,
    actions::{
      CreatePost,
      CreatePostLike,
      DeletePost,
      EditPost,
      EditPostNotifications,
      HidePost,
      MarkManyPostsAsRead,
      MarkPostAsRead,
      SavePost,
      moderation::{FeaturePost, ListPostLikes, LockPost, ModEditPost, RemovePost},
    },
  },
  report::{CreatePostReport, PostReportResponse, ResolvePostReport},
};

impl LemmyClient {
  /// Gets a post.
  ///
  /// HTTP GET /post
  pub async fn get_post(&self, data: GetPost) -> LemmyResult<GetPostResponse> {
    self.make_request(Method::GET, "post", data).await
  }

  /// Creates a post.
  ///
  /// HTTP POST /post
  pub async fn create_post(&self, data: CreatePost) -> LemmyResult<PostResponse> {
    self.make_request(Method::POST, "post", data).await
  }

  /// Edits a post you have already created.
  ///
  /// HTTP PUT /post
  pub async fn edit_post(&self, data: EditPost) -> LemmyResult<PostResponse> {
    self.make_request(Method::PUT, "post", data).await
  }

  /// Gets the content type and opengraph data of site linked in a post.
  ///
  /// HTTP GET /post/site_metadata
  pub async fn get_linked_site_metadata(
    &self,
    data: GetSiteMetadata,
  ) -> LemmyResult<GetSiteMetadataResponse> {
    self
      .make_request(Method::GET, "post/site_metadata", data)
      .await
  }

  /// Deletes a post you have made.
  ///
  /// HTTP POST /post/delete
  pub async fn delete_post(&self, data: DeletePost) -> LemmyResult<PostResponse> {
    self.make_request(Method::POST, "post/delete", data).await
  }

  /// Removes a post (moderator action).
  ///
  /// HTTP POST /post/remove
  pub async fn remove_post(&self, data: RemovePost) -> LemmyResult<PostResponse> {
    self.make_request(Method::POST, "post/remove", data).await
  }

  /// Marks a post as read.
  ///
  /// HTTP POST /post/mark_as_read
  pub async fn mark_post_as_read(&self, data: MarkPostAsRead) -> LemmyResult<PostResponse> {
    self
      .make_request(Method::POST, "post/mark_as_read", data)
      .await
  }

  /// Marks several posts as read.
  ///
  /// HTTP POST /post/mark_as_read/many
  pub async fn mark_many_posts_as_read(
    &self,
    data: MarkManyPostsAsRead,
  ) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "post/mark_as_read/many", data)
      .await
  }

  /// Hide a post from list views.
  ///
  /// HTTP POST /post/hide
  pub async fn hide_post(&self, data: HidePost) -> LemmyResult<SuccessResponse> {
    self.make_request(Method::POST, "post/hide", data).await
  }

  /// Prevents users from commenting on the post (moderator action).
  ///
  /// HTTP POST /post/lock
  pub async fn lock_post(&self, data: LockPost) -> LemmyResult<PostResponse> {
    self.make_request(Method::POST, "post/lock", data).await
  }

  /// Pins a post to the top of the community page (moderator action).
  ///
  /// HTTP POST /post/feature
  pub async fn feature_post(&self, data: FeaturePost) -> LemmyResult<PostResponse> {
    self.make_request(Method::POST, "post/feature", data).await
  }

  /// Gets posts with a variety of filters.
  ///
  /// HTTP GET /post/list
  pub async fn list_posts(&self, data: GetPosts) -> LemmyResult<PagedResponse<PostView>> {
    self.make_request(Method::GET, "post/list", data).await
  }

  /// Votes on a post.
  ///
  /// HTTP POST /post/like
  pub async fn like_post(&self, data: CreatePostLike) -> LemmyResult<PostResponse> {
    self.make_request(Method::POST, "post/like", data).await
  }

  /// Lists the likes for a post.
  ///
  /// HTTP GET /post/like/list
  pub async fn list_post_likes(&self, data: ListPostLikes) -> LemmyResult<PagedResponse<VoteView>> {
    self.make_request(Method::GET, "post/like/list", data).await
  }

  /// Saves a post to your favorites list.
  ///
  /// HTTP PUT /post/save
  pub async fn save_post(&self, data: SavePost) -> LemmyResult<PostResponse> {
    self.make_request(Method::PUT, "post/save", data).await
  }

  /// Reports a post to the moderator team of the community the post is in, the admin team of your
  /// instance, and the admin team of the poster's instance.
  ///
  /// HTTP POST /post/report
  pub async fn report_post(&self, data: CreatePostReport) -> LemmyResult<PostReportResponse> {
    self.make_request(Method::POST, "post/report", data).await
  }

  /// Resolves a post report (moderator action).
  ///
  /// HTTP PUT /post/report/resolve
  pub async fn resolve_post_report(
    &self,
    data: ResolvePostReport,
  ) -> LemmyResult<PostReportResponse> {
    self
      .make_request(Method::PUT, "post/report/resolve", data)
      .await
  }

  /// Edits post notifications.
  ///
  /// HTTP PUT /post/notifications
  pub async fn edit_post_notifications(
    &self,
    data: EditPostNotifications,
  ) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::PUT, "post/notifications", data)
      .await
  }

  /// Edit a post as a mod.
  ///
  /// HTTP PUT /post/mod_edit
  pub async fn mod_edit_post(&self, data: ModEditPost) -> LemmyResult<PostResponse> {
    self.make_request(Method::PUT, "post/mod_edit", data).await
  }

  /// Create a warning for a post.
  ///
  /// HTTP POST /post/warn
  pub async fn create_post_warning(&self, data: CreatePostWarning) -> LemmyResult<PostResponse> {
    self.make_request(Method::POST, "post/warn", data).await
  }
}
