use crate::{LemmyClient, lemmy_client::LemmyResult};
use lemmy_api_common::{
  federation::{GetFederatedInstancesResponse, ResolveObject},
  media::UploadImageResponse,
  modlog::{GetModlog, GetModlogResponse},
  report::{ListReports, ListReportsResponse},
  search::{Search, SearchResponse},
  site::{
    GetSiteResponse,
    SiteResponse,
    administration::{CreateSite, EditSite},
  },
};
use reqwest::Method;

// TODO: Add stuff for icon and banner

impl LemmyClient {
  /// Gets the site and, if you pass an authorized JWT, information about the logged in user.
  ///
  /// HTTP GET /site
  pub async fn get_site(&self) -> LemmyResult<GetSiteResponse> {
    self.make_request(Method::GET, "site", ().into()).await
  }

  /// Creates site during initial setup.
  ///
  /// HTTP POST /site
  pub async fn create_site(&self, data: CreateSite) -> LemmyResult<SiteResponse> {
    self.make_request(Method::POST, "site", data.into()).await
  }

  ///Edit settings for the site you administer.
  ///
  /// HTTP PUT /site
  pub async fn edit_site(&self, data: EditSite) -> LemmyResult<SiteResponse> {
    self.make_request(Method::PUT, "site", data.into()).await
  }

  /// Gets the modlog.
  ///
  ///HTTP GET /modlog
  pub async fn get_modlog(&self, data: GetModlog) -> LemmyResult<GetModlogResponse> {
    self.make_request(Method::GET, "modlog", data.into()).await
  }

  /// Searches for content.
  ///
  /// HTTP GET /search
  pub async fn search(&self, data: Search) -> LemmyResult<SearchResponse> {
    self.make_request(Method::GET, "search", data.into()).await
  }

  /// Fetches an object from a non-local instance.
  ///
  /// HTTP GET /resolve_object
  pub async fn resolve_object(&self, data: ResolveObject) -> LemmyResult<SearchResponse> {
    self
      .make_request(Method::GET, "resolve_object", data.into())
      .await
  }
  /// Gets the instances that are federated with your instance.
  ///
  /// HTTP GET /federated_instances
  pub async fn get_federated_instances(&self) -> LemmyResult<GetFederatedInstancesResponse> {
    self
      .make_request(Method::GET, "federated_instances", ().into())
      .await
  }

  /// List all reports.
  ///
  /// HTTP GET /report/list
  pub async fn list_reports(&self, data: ListReports) -> LemmyResult<ListReportsResponse> {
    self
      .make_request(Method::GET, "report/list", data.into())
      .await
  }

  /// Upload an icon for your site. This is shown as the site favicon, in site header, and is used as metadata for external instance listings like [join-lemmy.org](https://join-lemmy.org/instances).
  ///
  /// **Only usable by instance admins**
  ///
  /// HTTP POST /icon
  pub async fn upload_site_icon(&self, request: &'static [u8]) -> LemmyResult<UploadImageResponse> {
    self.make_file_request("site/icon", request.into()).await
  }

  /// Upload a banner for your site. This is shown in the site sidebar and is used as metadata for
  /// external instance listings like [join-lemmy.org](https://join-lemmy.org/instances).
  ///
  /// **Only usable by instance admins**
  ///
  /// HTTP POST /banner
  pub async fn upload_site_banner(
    &self,
    request: &'static [u8],
  ) -> LemmyResult<UploadImageResponse> {
    self.make_file_request("site/banner", request.into()).await
  }
}
