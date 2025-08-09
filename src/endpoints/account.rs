use crate::{LemmyClient, LemmyResult};
use http::{Method, header::AUTHORIZATION};
use lemmy_api_common::{
  SuccessResponse,
  account::{
    DeleteAccount,
    ListPersonHidden,
    ListPersonHiddenResponse,
    ListPersonLiked,
    ListPersonLikedResponse,
    ListPersonRead,
    ListPersonReadResponse,
    ListPersonSaved,
    ListPersonSavedResponse,
    MyUserInfo,
    SaveUserSettings,
    auth::{
      ChangePassword,
      GenerateTotpSecretResponse,
      GetCaptchaResponse,
      ListLoginsResponse,
      Login,
      LoginResponse,
      PasswordChangeAfterReset,
      PasswordReset,
      Register,
      ResendVerificationEmail,
      UpdateTotp,
      UpdateTotpResponse,
      UserSettingsBackup,
      VerifyEmail,
    },
  },
  community::actions::{BlockCommunity, BlockCommunityResponse},
  federation::{UserBlockInstanceCommunitiesParams, UserBlockInstancePersonsParams},
  media::{DeleteImageParams, ListMedia, ListMediaResponse},
  person::actions::{BlockPerson, BlockPersonResponse},
  report::{GetReportCount, GetReportCountResponse},
};

// TODO: Handle Account avatar and banner

impl LemmyClient {
  /// Registers a new account on an instance.
  ///
  /// HTTP POST /account/auth/register
  pub async fn register_account(&self, data: Register) -> LemmyResult<LoginResponse> {
    self
      .make_request(Method::POST, "account/auth/register", data)
      .await
  }

  /// Logs into the instance, giving you a JWT to use to make authorized requests.
  ///
  /// HTTP POST /account/auth/login
  pub async fn login(&self, data: Login) -> LemmyResult<LoginResponse> {
    self
      .make_request(Method::POST, "account/auth/login", data)
      .await
  }

  /// Deletes the active session associated with the JWT.
  /// If the response is successful, the JWT from the headers the client
  /// sends with each request is also removed.
  ///
  /// HTTP POST /account/auth/logout
  pub async fn logout(&mut self) -> LemmyResult<SuccessResponse> {
    let response = self
      .make_request(Method::POST, "account/auth/logout", ())
      .await;

    if response.is_ok() {
      let headers = self.headers_mut();
      headers.remove(AUTHORIZATION);
    }

    response
  }

  /// Sends an email to your account (if you have one) with a one time link to change your password.
  /// Use this if you forgot your password.
  ///
  /// HTTP POST /account/auth/password_reset
  pub async fn reset_password(&self, data: PasswordReset) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "account/auth/password_reset", data)
      .await
  }

  /// Gets a captcha.
  ///
  /// HTTP GET /account/auth/get_captcha
  pub async fn get_captcha(&self) -> LemmyResult<GetCaptchaResponse> {
    self
      .make_request(Method::GET, "account/auth/get_captcha", ())
      .await
  }

  /// Follows through with one time link password reset request.
  ///
  /// HTTP POST /account/auth/password_change
  pub async fn change_password_after_reset(
    &self,
    data: PasswordChangeAfterReset,
  ) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "account/auth/password_change", data)
      .await
  }

  /// Changes your password if you are already logged in.
  ///
  /// HTTP PUT /account/auth/change_password
  pub async fn change_password(&self, data: ChangePassword) -> LemmyResult<LoginResponse> {
    self
      .make_request(Method::POST, "account/auth/change_password", data)
      .await
  }

  /// Generates a secret to enable time-based one time passwords for two-factor authentication.
  ///
  /// After this, you will need to call /account/auth/totp/update with a valid token to enable it.
  ///
  /// HTTP POST /account/auth/totp/generate
  pub async fn generate_totp_secret(&self) -> LemmyResult<GenerateTotpSecretResponse> {
    self
      .make_request(Method::POST, "account/auth/totp/generate", ())
      .await
  }

  /// Enables/disables two-factor authentication.
  ///
  /// To enable, you must first call /account/auth/totp/generate to generate a token to pass to
  /// this.
  ///
  /// You can only disable this if it is already enabled. Again, you must pass a valid TOTP.
  ///
  /// HTTP POST /account/auth/totp/update
  pub async fn update_totp(&self, data: UpdateTotp) -> LemmyResult<UpdateTotpResponse> {
    self
      .make_request(Method::POST, "account/auth/totp/update", data)
      .await
  }

  /// Verifies your email. Used when the instance you are registering an account on requires email
  /// verification.
  ///
  /// HTTP POST /account/auth/verify_email
  pub async fn verify_email(&self, data: VerifyEmail) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "account/auth/verify_email", data)
      .await
  }

  /// Resend a verification email.
  ///
  /// HTTP POST /account/auth/resend_verification_email
  pub async fn resend_verification_email(
    &self,
    data: ResendVerificationEmail,
  ) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "account/auth/resend_verification_email", data)
      .await
  }

  /// Return the user associated with the JWT token passed.
  ///
  /// HTTP GET /account
  pub async fn get_current_user(&self) -> LemmyResult<MyUserInfo> {
    self.make_request(Method::GET, "account", ()).await
  }

  /// Delete an image that you uploaded.
  ///
  /// HTTP DELETE /account/media
  pub async fn delete_image(&self, data: DeleteImageParams) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::DELETE, "account/media", data)
      .await
  }

  /// Gets all media posted by the logged in user.
  ///
  /// HTTP GET /account/media/list
  pub async fn list_media(&self, data: ListMedia) -> LemmyResult<ListMediaResponse> {
    self
      .make_request(Method::GET, "account/media/list", data)
      .await
  }

  /// Deletes your account.
  ///
  /// HTTP POST /account/delete
  pub async fn delete_account(&self, data: DeleteAccount) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "account/delete", data)
      .await
  }

  /// Marks all notifications (replies, mentions, private messages) as read.
  ///
  /// HTTP POST /account/mark_as_read/all
  pub async fn mark_all_notifications_as_read(&self) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "account/mark_as_read/all", ())
      .await
  }

  /// Gets number of reports you can resolve.
  ///
  /// HTTP GET /account/report_count
  pub async fn report_count(&self, data: GetReportCount) -> LemmyResult<GetReportCountResponse> {
    self
      .make_request(Method::GET, "account/report_count", data)
      .await
  }

  /// Lists login tokens for your user's active sessions.
  ///
  /// HTTP GET /account/list_logins
  pub async fn list_logins(&self) -> LemmyResult<ListLoginsResponse> {
    self.make_request(Method::GET, "list_logins", ()).await
  }

  /// Returns an error message if your auth token is invalid.
  ///
  /// HTTP GET /account/validate_auth
  pub async fn validate_auth(&self) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::GET, "account/validate_auth", ())
      .await
  }

  /// Make donation dialog appear for users of your instance even if a user dismissed it before.
  ///
  /// HTTP POST /account/donation_dialog_shown
  pub async fn donation_dialog_shown(&self) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "account/donation_dialog_shown", ())
      .await
  }

  /// Blocks a person.
  ///
  /// HTTP POST /account/block/person
  pub async fn block_person(&self, data: BlockPerson) -> LemmyResult<BlockPersonResponse> {
    self
      .make_request(Method::POST, "account/block/person", data)
      .await
  }

  /// Blocks a community.
  ///
  /// HTTP POST /account/block/community
  pub async fn block_community(&self, data: BlockCommunity) -> LemmyResult<BlockCommunityResponse> {
    self
      .make_request(Method::POST, "account/block/community", data)
      .await
  }

  /// Prevents posts from communities from the blocked instance from appearing in your feed.
  ///
  /// HTTP POST /account/block/instance/communities
  pub async fn user_block_instance_communities(
    &self,
    data: UserBlockInstanceCommunitiesParams,
  ) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "account/block/instance/communities", data)
      .await
  }

  /// Prevents posts and comments by users from the blocked instance from being fetched.
  ///
  /// HTTP POST /account/block/instance/persons
  pub async fn user_block_instance_users(
    &self,
    data: UserBlockInstancePersonsParams,
  ) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "account/block/instance/persons", data)
      .await
  }

  /// List posts and comments that were saved by the authenticated user.
  ///
  /// HTTP GET /account/saved
  pub async fn list_saved(&self, data: ListPersonSaved) -> LemmyResult<ListPersonSavedResponse> {
    self.make_request(Method::GET, "account/saved", data).await
  }

  /// List posts and comments that were read by the authenticated user in reverse chronological
  /// order.
  ///
  /// HTTP GET /account/read
  pub async fn list_read(&self, data: ListPersonRead) -> LemmyResult<ListPersonReadResponse> {
    self.make_request(Method::GET, "account/read", data).await
  }

  /// List posts and comments that were hidden by the authenticated user, ordered by date hidden.
  ///
  /// HTTP GET /account/hidden
  pub async fn list_hidden(&self, data: ListPersonHidden) -> LemmyResult<ListPersonHiddenResponse> {
    self.make_request(Method::GET, "account/hidden", data).await
  }

  /// List posts and comments that were liked by the authenticated user.
  ///
  /// HTTP GET /account/liked
  pub async fn list_liked(&self, data: ListPersonLiked) -> LemmyResult<ListPersonLikedResponse> {
    self.make_request(Method::GET, "account/liked", data).await
  }

  /// Saves your account settings.
  ///
  /// HTTP PUT /account/settings/save
  pub async fn save_user_settings(&self, data: SaveUserSettings) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::PUT, "account/settings/save", data)
      .await
  }

  /// Exports a backup of your user settings - including your saved content, followed communities,
  /// and blocks - as JSON.
  ///
  /// HTTP GET /account/settings/export
  pub async fn export_settings(&self) -> LemmyResult<UserSettingsBackup> {
    self
      .make_request(Method::GET, "account/settings/export", ())
      .await
  }

  /// Imports a backup of your user settings.
  ///
  /// HTTP POST /account/settings/import
  pub async fn import_settings(&self, data: UserSettingsBackup) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "account/settings/import", data)
      .await
  }
}
