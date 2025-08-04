use crate::{LemmyClient, LemmyResult};
use http::Method;
use lemmy_api_common::{
  SuccessResponse,
  account::auth::LoginResponse,
  oauth::{
    AuthenticateWithOauth,
    CreateOAuthProvider,
    DeleteOAuthProvider,
    EditOAuthProvider,
    OAuthProvider,
  },
};

impl LemmyClient {
  /// Add an OAuth provider your users can use to register for and log into your instance.
  ///
  /// HTTP POST /oauth_provider
  pub async fn add_oauth_provider(&self, data: CreateOAuthProvider) -> LemmyResult<OAuthProvider> {
    self
      .make_request(Method::POST, "oauth_provider", data.into())
      .await
  }

  /// Edit one of your instance's OAuth providers.
  ///
  /// HTTP PUT /oauth_provider
  pub async fn edit_oauth_provider(&self, data: EditOAuthProvider) -> LemmyResult<OAuthProvider> {
    self
      .make_request(Method::PUT, "oauth_provider", data.into())
      .await
  }

  /// Remove an OAuth provider from your instance.
  ///
  /// HTTP POST /oauth_provider/delete
  pub async fn delete_oauth_provider(
    &self,
    data: DeleteOAuthProvider,
  ) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "oauth_provider/delete", data.into())
      .await
  }

  /// Register/login to an instance using one of the OAuth providers that instance supports.
  ///
  /// HTTP POST /oauth/authenticate
  pub async fn authenticate_with_oauth(
    &self,
    data: AuthenticateWithOauth,
  ) -> LemmyResult<LoginResponse> {
    self
      .make_request(Method::POST, "oauth/authenticate", data.into())
      .await
  }
}
