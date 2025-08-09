use crate::{LemmyClient, LemmyResult};
use http::Method;
use lemmy_api_common::custom_emoji::{
  CreateCustomEmoji,
  CustomEmojiResponse,
  DeleteCustomEmoji,
  EditCustomEmoji,
  ListCustomEmojis,
  ListCustomEmojisResponse,
};

impl LemmyClient {
  /// Creates a custom emoji.
  ///
  /// HTTP POST /custom_emoji
  pub async fn create_custom_emoji(
    &self,
    data: CreateCustomEmoji,
  ) -> LemmyResult<CustomEmojiResponse> {
    self
      .make_request(Method::POST, "custom_emoji", data.into())
      .await
  }

  /// Edits an existing custom emoji.
  ///
  /// HTTP PUT /custom_emoji
  pub async fn edit_custom_emoji(&self, data: EditCustomEmoji) -> LemmyResult<CustomEmojiResponse> {
    self
      .make_request(Method::PUT, "custom_emoji", data.into())
      .await
  }

  /// Deletes an existing custom emoji.
  ///
  /// HTTP POST /custom_emoji/delete
  pub async fn delete_custom_emoji(
    &self,
    data: DeleteCustomEmoji,
  ) -> LemmyResult<CustomEmojiResponse> {
    self
      .make_request(Method::POST, "custom_emoji/delete", data.into())
      .await
  }

  /// List all custom emojis on the instance.
  ///
  /// HTTP GET /custom_emoji/list
  pub async fn list_custom_emojis(
    &self,
    data: ListCustomEmojis,
  ) -> LemmyResult<ListCustomEmojisResponse> {
    self
      .make_request(Method::GET, "custom_emoji/list", data.into())
      .await
  }
}
