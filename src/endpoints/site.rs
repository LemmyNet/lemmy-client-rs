use crate::{LemmyClient, lemmy_client::LemmyResult};
use lemmy_api_common::{
  SuccessResponse,
  federation::{GetFederatedInstancesResponse, ResolveObject},
  media::{DeleteImageParams, UploadImageResponse},
  modlog::{GetModlog, GetModlogResponse},
  search::{Search, SearchResponse},
  site::{
    GetSiteResponse,
    SiteResponse,
    administration::{CreateSite, EditSite},
  },
};
use reqwest::{Body, Method};

// TODO: Add stuff for icon and banner

impl LemmyClient {
  /// Gets the site.
  ///
  /// HTTP GET /site
  pub async fn get_site(&self) -> LemmyResult<GetSiteResponse> {
    self.make_request(Method::GET, "site", ()).await
  }

  /// Creates site during initial setup.
  ///
  /// HTTP POST /site
  pub async fn create_site(&self, data: CreateSite) -> LemmyResult<SiteResponse> {
    self.make_request(Method::POST, "site", data).await
  }

  ///Edit settings for the site you administer.
  ///
  /// HTTP PUT /site
  pub async fn edit_site(&self, data: EditSite) -> LemmyResult<SiteResponse> {
    self.make_request(Method::PUT, "site", data).await
  }

  /// Upload an icon for your site. This is shown as the site favicon, in site header, and is used as metadata for external instance listings like [join-lemmy.org](https://join-lemmy.org/instances).
  ///
  /// **Only usable by instance admins**
  ///
  /// HTTP POST /site/icon
  pub async fn upload_site_icon(
    &self,
    request: impl Into<Body>,
  ) -> LemmyResult<UploadImageResponse> {
    self.make_file_request("site/icon", (), request).await
  }

  /// Delete your site's icon.
  ///
  /// **Only usable by instance admins**
  ///
  /// HTTP DELETE /site/icon
  pub async fn delete_site_icon(&self, request: DeleteImageParams) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::DELETE, "site/icon", request)
      .await
  }

  /// Upload a banner for your site. This is shown in the site sidebar and is used as metadata for
  /// external instance listings like [join-lemmy.org](https://join-lemmy.org/instances).
  ///
  /// **Only usable by instance admins**
  ///
  /// HTTP POST /site/banner
  pub async fn upload_site_banner(
    &self,
    request: impl Into<Body>,
  ) -> LemmyResult<UploadImageResponse> {
    self.make_file_request("site/banner", (), request).await
  }

  /// Delete your site's icon.
  ///
  /// **Only usable by instance admins**
  ///
  /// HTTP DELETE /site/icon
  pub async fn delete_site_banner(
    &self,
    request: DeleteImageParams,
  ) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::DELETE, "site/banner", request)
      .await
  }

  /// Gets the modlog.
  ///
  ///HTTP GET /modlog
  pub async fn get_modlog(&self, data: GetModlog) -> LemmyResult<GetModlogResponse> {
    self.make_request(Method::GET, "modlog", data).await
  }

  /// Searches for content.
  ///
  /// HTTP GET /search
  pub async fn search(&self, data: Search) -> LemmyResult<SearchResponse> {
    self.make_request(Method::GET, "search", data).await
  }

  /// Fetches an object from a non-local instance.
  ///
  /// HTTP GET /resolve_object
  pub async fn resolve_object(&self, data: ResolveObject) -> LemmyResult<SearchResponse> {
    self.make_request(Method::GET, "resolve_object", data).await
  }
  /// Gets the instances that are federated with your instance.
  ///
  /// HTTP GET /federated_instances
  pub async fn get_federated_instances(&self) -> LemmyResult<GetFederatedInstancesResponse> {
    self
      .make_request(Method::GET, "federated_instances", ())
      .await
  }
}
