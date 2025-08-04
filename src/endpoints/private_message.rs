use crate::{LemmyClient, LemmyResult};
use http::Method;
use lemmy_api_common::{
  private_message::{
    PrivateMessageResponse,
    actions::{CreatePrivateMessage, DeletePrivateMessage, EditPrivateMessage},
  },
  report::{CreatePrivateMessageReport, PrivateMessageReportResponse, ResolvePrivateMessageReport},
};

impl LemmyClient {
  /// Creates and send a private message to another user.
  ///
  /// HTTP POST /private_message
  pub async fn create_private_message(
    &self,
    data: CreatePrivateMessage,
  ) -> LemmyResult<PrivateMessageResponse> {
    self
      .make_request(Method::POST, "private_message", data.into())
      .await
  }

  /// Edits a private message you have already sent.
  ///
  /// HTTP PUT /private_message
  pub async fn edit_private_message(
    &self,
    data: EditPrivateMessage,
  ) -> LemmyResult<PrivateMessageResponse> {
    self
      .make_request(Method::PUT, "private_message", data.into())
      .await
  }

  /// Deletes a private that you have already sent.
  ///
  /// HTTP POST /private_message/delete
  pub async fn delete_private_message(
    &self,
    data: DeletePrivateMessage,
  ) -> LemmyResult<PrivateMessageResponse> {
    self
      .make_request(Method::POST, "private_message/delete", data.into())
      .await
  }

  /// Reports a private message that was sent to you to your instance's admin team and the sender's
  /// instance's admin team.
  ///
  /// HTTP POST /private_message/report
  pub async fn report_private_message(
    &self,
    data: CreatePrivateMessageReport,
  ) -> LemmyResult<PrivateMessageReportResponse> {
    self
      .make_request(Method::POST, "private_message/report", data.into())
      .await
  }

  /// Resolves a report of a private message sent to a user on the instance you administrate.
  ///
  /// HTTP PUT /private_message/report/resolve
  pub async fn resolve_private_message_report(
    &self,
    data: ResolvePrivateMessageReport,
  ) -> LemmyResult<PrivateMessageReportResponse> {
    self
      .make_request(Method::PUT, "private_message/report/resolve", data.into())
      .await
  }
}
