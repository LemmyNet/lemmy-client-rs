use crate::{LemmyClient, LemmyResult};
use http::Method;
use lemmy_api_common::{
  PagedResponse,
  report::{ListReports, ReportCombinedView},
};

impl LemmyClient {
  /// List all reports.
  ///
  /// HTTP GET /report/list
  pub async fn list_reports(
    &self,
    data: ListReports,
  ) -> LemmyResult<PagedResponse<ReportCombinedView>> {
    self.make_request(Method::GET, "report/list", data).await
  }
}
