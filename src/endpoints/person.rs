use crate::{LemmyClient, LemmyResult};
use http::Method;
use lemmy_api_common::{
  SuccessResponse,
  person::{
    GetPersonDetails,
    GetPersonDetailsResponse,
    actions::{ListPersonContent, ListPersonContentResponse, NotePerson},
  },
};

impl LemmyClient {
  /// Gets the publicly viewable details of a user's account.
  ///
  /// HTTP GET /person
  pub async fn get_person_details(
    &self,
    data: GetPersonDetails,
  ) -> LemmyResult<GetPersonDetailsResponse> {
    self.make_request(Method::GET, "person", data.into()).await
  }

  /// List posts and comments made by a user.
  ///
  /// HTTP GET /person/content
  pub async fn list_person_content(
    &self,
    data: ListPersonContent,
  ) -> LemmyResult<ListPersonContentResponse> {
    self
      .make_request(Method::GET, "person/content", data.into())
      .await
  }

  /// Create a note about another user.
  ///
  /// HTTP POST /person/note
  pub async fn create_person_note(&self, data: NotePerson) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "person/note", data.into())
      .await
  }
}
