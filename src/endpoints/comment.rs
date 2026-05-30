use crate::{LemmyClient, LemmyResult};
use http::Method;
use lemmy_api_common::{
  PagedResponse,
  VoteView,
  comment::{
    CommentResponse,
    CommentSlimView,
    CommentView,
    GetComment,
    GetComments,
    actions::{
      CreateComment,
      CreateCommentLike,
      CreateCommentWarning,
      DeleteComment,
      EditComment,
      LockComment,
      SaveComment,
      moderation::{DistinguishComment, ListCommentLikes, RemoveComment},
    },
  },
  report::{CommentReportResponse, CreateCommentReport, ResolveCommentReport},
};

impl LemmyClient {
  /// Gets a comment.
  ///
  /// HTTP GET /comment
  pub async fn get_comment(&self, data: GetComment) -> LemmyResult<CommentResponse> {
    self.make_request(Method::GET, "comment", data).await
  }

  /// Creates a new comment.
  ///
  /// HTTP POST /comment
  pub async fn create_comment(&self, data: CreateComment) -> LemmyResult<CommentResponse> {
    self.make_request(Method::POST, "comment", data).await
  }

  /// Edits one of your already-created comments.
  ///
  /// HTTP PUT /comment
  pub async fn edit_comment(&self, data: EditComment) -> LemmyResult<CommentResponse> {
    self.make_request(Method::PUT, "comment", data).await
  }

  /// Deletes one of your already-existing comments.
  ///
  /// HTTP POST /comment/delete
  pub async fn delete_comment(&self, data: DeleteComment) -> LemmyResult<CommentResponse> {
    self
      .make_request(Method::POST, "comment/delete", data)
      .await
  }

  /// Removes a post (moderator action).
  ///
  /// HTTP POST /comment/remove
  pub async fn remove_comment(&self, data: RemoveComment) -> LemmyResult<CommentResponse> {
    self
      .make_request(Method::POST, "comment/remove", data)
      .await
  }

  /// Pins a comment to the top of a post's comment section (speak as moderator).
  ///
  /// HTTP POST /comment/distinguish
  pub async fn distinguish_comment(
    &self,
    data: DistinguishComment,
  ) -> LemmyResult<CommentResponse> {
    self
      .make_request(Method::POST, "comment/distinguish", data)
      .await
  }

  /// Votes on a comment.
  ///
  /// HTTP POST /comment/like
  pub async fn like_comment(&self, data: CreateCommentLike) -> LemmyResult<CommentResponse> {
    self.make_request(Method::POST, "comment/like", data).await
  }

  /// Gets the votes for a comment.
  ///
  /// HTTP GET /comment/like/list
  pub async fn list_comment_likes(
    &self,
    data: ListCommentLikes,
  ) -> LemmyResult<PagedResponse<VoteView>> {
    self
      .make_request(Method::GET, "comment/like/list", data)
      .await
  }

  /// Saves a comment to your favorites list.
  ///
  /// HTTP PUT /comment/save
  pub async fn save_comment(&self, data: SaveComment) -> LemmyResult<CommentResponse> {
    self.make_request(Method::PUT, "comment/save", data).await
  }

  /// Lock a comment thread.
  ///
  /// HTTP POST /comment/lock
  pub async fn lock_comment(&self, data: LockComment) -> LemmyResult<CommentResponse> {
    self.make_request(Method::POST, "comment/lock", data).await
  }

  /// Gets comments with various filters.
  ///
  /// HTTP GET /comment/list
  pub async fn list_comments(&self, data: GetComments) -> LemmyResult<PagedResponse<CommentView>> {
    self.make_request(Method::GET, "comment/list", data).await
  }

  /// Retrieve a slim representation of comments.
  ///
  /// HTTP GET /comment/list/slim
  pub async fn list_comments_slim(
    &self,
    data: GetComments,
  ) -> LemmyResult<PagedResponse<CommentSlimView>> {
    self
      .make_request(Method::GET, "comment/list/slim", data)
      .await
  }

  /// Warn a comment.
  ///
  /// HTTP POST /comment/warn
  pub async fn create_comment_warning(
    &self,
    data: CreateCommentWarning,
  ) -> LemmyResult<CommentResponse> {
    self.make_request(Method::POST, "comment/warn", data).await
  }

  /// Reports a comment to the moderator team of the community the comment is in, your instance's
  /// admin team, and the commenter's instance's admin team.
  ///
  /// HTTP POST /comment/report
  pub async fn report_comment(&self, data: CreateCommentReport) -> LemmyResult<CommentResponse> {
    self
      .make_request(Method::POST, "comment/report", data)
      .await
  }

  /// Resolves a report on a comment made in a community you moderate or instance you administrate.
  ///
  /// HTTP PUT /comment/report/resolve
  pub async fn resolve_comment_report(
    &self,
    data: ResolveCommentReport,
  ) -> LemmyResult<CommentReportResponse> {
    self
      .make_request(Method::PUT, "comment/report/resolve", data)
      .await
  }
}
