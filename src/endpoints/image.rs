use crate::{LemmyClient, LemmyResult};
use http::Method;
use lemmy_api_common::{
  PagedResponse,
  SuccessResponse,
  media::{DeleteImageParams, ListMedia, LocalImageView, UploadImageResponse},
};
use reqwest::Body;

impl LemmyClient {
  /// Upload an image to the instance.
  ///
  /// HTTP POST /image
  pub async fn upload_image(&self, data: impl Into<Body>) -> LemmyResult<UploadImageResponse> {
    self.make_file_request("image", (), data).await
  }

  /// Deletes an image from the instance.
  ///
  /// **Can only be used by instance admins**
  ///
  /// HTTP DELETE /image
  pub async fn delete_image_admin(&self, data: DeleteImageParams) -> LemmyResult<SuccessResponse> {
    self.make_request(Method::DELETE, "image", data).await
  }

  /// Gets the pictrs image service health.
  ///
  /// HTTP GET /image/health
  pub async fn pictrs_health(&self) -> LemmyResult<SuccessResponse> {
    self.make_request(Method::GET, "image/health", ()).await
  }

  /// Gets all media posted on an instance. Only usable by the instance's admins.
  ///
  /// HTTP GET /image/list
  pub async fn list_all_media(
    &self,
    data: ListMedia,
  ) -> LemmyResult<PagedResponse<LocalImageView>> {
    self.make_request(Method::GET, "image/list", data).await
  }
}
