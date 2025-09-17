use crate::{LemmyClient, LemmyResult};
use http::Method;
use lemmy_api_common::report::{
  CommunityReportResponse,
  CreateCommunityReport,
  ListReports,
  ListReportsResponse,
  ResolveCommunityReport,
};

impl LemmyClient {
  /// List all reports.
  ///
  /// HTTP GET /report/list
  pub async fn list_reports(&self, data: ListReports) -> LemmyResult<ListReportsResponse> {
    self.make_request(Method::GET, "report/list", data).await
  }

  /// Report a community.
  ///
  /// HTTP POST /community/report
  pub async fn report_community(
    &self,
    data: CreateCommunityReport,
  ) -> LemmyResult<CommunityReportResponse> {
    self
      .make_request(Method::POST, "community/report", data)
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
      .make_request(Method::PUT, "community/report/resolve", data)
      .await
  }
}
