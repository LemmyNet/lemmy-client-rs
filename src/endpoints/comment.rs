use crate::{LemmyClient, LemmyResult};
use http::Method;
use lemmy_api_common::{
  comment::{
    CommentResponse,
    GetComment,
    GetComments,
    GetCommentsResponse,
    GetCommentsSlimResponse,
    actions::{
      CreateComment,
      CreateCommentLike,
      DeleteComment,
      EditComment,
      SaveComment,
      moderation::{DistinguishComment, ListCommentLikes, ListCommentLikesResponse, RemoveComment},
    },
  },
  report::{CommentReportResponse, CreateCommentReport, ResolveCommentReport},
};

impl LemmyClient {
  /// Gets a comment.
  ///
  /// HTTP GET /comment
  pub async fn get_comment(&self, data: GetComment) -> LemmyResult<CommentResponse> {
    self.make_request(Method::GET, "comment", data.into()).await
  }

  /// Creates a new comment.
  ///
  /// HTTP POST /comment
  pub async fn create_comment(&self, data: CreateComment) -> LemmyResult<CommentResponse> {
    self
      .make_request(Method::POST, "comment", data.into())
      .await
  }

  /// Edits one of your already-created comments.
  ///
  /// HTTP PUT /comment
  pub async fn edit_comment(&self, data: EditComment) -> LemmyResult<CommentResponse> {
    self.make_request(Method::PUT, "comment", data.into()).await
  }

  /// Deletes one of your already-existing comments.
  ///
  /// HTTP POST /comment/delete
  pub async fn delete_comment(&self, data: DeleteComment) -> LemmyResult<CommentResponse> {
    self
      .make_request(Method::POST, "comment/delete", data.into())
      .await
  }

  /// Removes a post (moderator action).
  ///
  /// HTTP POST /comment/remove
  pub async fn remove_comment(&self, data: RemoveComment) -> LemmyResult<CommentResponse> {
    self
      .make_request(Method::POST, "comment/remove", data.into())
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
      .make_request(Method::POST, "comment/distinguish", data.into())
      .await
  }

  /// Votes on a comment.
  ///
  /// HTTP POST /comment/like
  pub async fn like_comment(&self, data: CreateCommentLike) -> LemmyResult<CommentResponse> {
    self
      .make_request(Method::POST, "comment/like", data.into())
      .await
  }

  /// Gets the votes for a comment.
  ///
  /// HTTP GET /comment/like/list
  pub async fn list_comment_likes(
    &self,
    data: ListCommentLikes,
  ) -> LemmyResult<ListCommentLikesResponse> {
    self
      .make_request(Method::GET, "comment/like/list", data.into())
      .await
  }

  /// Saves a comment to your favorites list.
  ///
  /// HTTP PUT /comment/save
  pub async fn save_comment(&self, data: SaveComment) -> LemmyResult<CommentResponse> {
    self
      .make_request(Method::PUT, "comment/save", data.into())
      .await
  }

  /// Gets comments with various filters.
  ///
  /// HTTP GET /comment/list
  pub async fn list_comments(&self, data: GetComments) -> LemmyResult<GetCommentsResponse> {
    self
      .make_request(Method::GET, "comment/list", data.into())
      .await
  }

  /// Retrieve a slim representation of comments.
  ///
  /// HTTP GET /comment/list/slim
  pub async fn list_comments_slim(
    &self,
    data: GetComments,
  ) -> LemmyResult<GetCommentsSlimResponse> {
    self
      .make_request(Method::GET, "comment/list/slim", data.into())
      .await
  }

  /// Reports a comment to the moderator team of the community the comment is in, your instance's
  /// admin team, and the commenter's instance's admin team.
  ///
  /// HTTP POST /comment/report
  pub async fn report_comment(&self, data: CreateCommentReport) -> LemmyResult<CommentResponse> {
    self
      .make_request(Method::POST, "comment/report", data.into())
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
      .make_request(Method::PUT, "comment/report/resolve", data.into())
      .await
  }
}
