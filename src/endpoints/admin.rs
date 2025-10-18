use crate::{LemmyClient, LemmyResult};
use http::Method;
use lemmy_api_common::{
  SuccessResponse,
  comment::actions::moderation::PurgeComment,
  community::actions::moderation::PurgeCommunity,
  federation::administration::{AdminAllowInstanceParams, AdminBlockInstanceParams},
  media::{DeleteImageParams, ListMedia, ListMediaResponse},
  person::{
    PersonResponse,
    actions::moderation::{
      BanPerson,
      GetRegistrationApplication,
      PurgePerson,
      RegistrationApplicationResponse,
    },
  },
  post::actions::moderation::PurgePost,
  site::{
    GetSiteResponse,
    administration::{
      AddAdmin,
      AddAdminResponse,
      AdminListUsers,
      AdminListUsersResponse,
      ApproveRegistrationApplication,
      GetUnreadRegistrationApplicationCountResponse,
      ListRegistrationApplications,
      ListRegistrationApplicationsResponse,
    },
  },
  tagline::{
    ListTaglines,
    ListTaglinesResponse,
    TaglineResponse,
    administration::{CreateTagline, DeleteTagline, UpdateTagline},
  },
};

impl LemmyClient {
  /// Adds a user to your instance's admin team.
  ///
  /// HTTP POST admin/add
  pub async fn add_admin(&self, data: AddAdmin) -> LemmyResult<AddAdminResponse> {
    self.make_request(Method::POST, "admin/add", data).await
  }

  /// Gets the number of unread registration applications for the instance you administrate.
  ///
  /// HTTP GET /admin/registration_application/count
  pub async fn unread_registration_application_count(
    &self,
  ) -> LemmyResult<GetUnreadRegistrationApplicationCountResponse> {
    self
      .make_request(Method::GET, "admin/registration_application/count", ())
      .await
  }

  /// Gets applications to register an account on the instance you administrate.
  ///
  /// HTTP GET /admin/registration_application/list
  pub async fn list_registration_applications(
    &self,
    data: ListRegistrationApplications,
  ) -> LemmyResult<ListRegistrationApplicationsResponse> {
    self
      .make_request(Method::GET, "admin/registration_application/list", data)
      .await
  }

  /// Approves a pending registration application.
  ///
  /// HTTP PUT /admin/registration_application/approve
  pub async fn approve_registration_application(
    &self,
    data: ApproveRegistrationApplication,
  ) -> LemmyResult<RegistrationApplicationResponse> {
    self
      .make_request(Method::PUT, "admin/registration_application/approve", data)
      .await
  }

  /// Get the application a user submitted when they first registered their account
  ///
  /// HTTP GET /admin/registration_application
  pub async fn get_registration_application(
    &self,
    data: GetRegistrationApplication,
  ) -> LemmyResult<RegistrationApplicationResponse> {
    self
      .make_request(Method::GET, "admin/registration_application", data)
      .await
  }

  /// Purges a user from the database.
  ///
  /// HTTP POST /admin/purge/person
  pub async fn purge_person(&self, data: PurgePerson) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "admin/purge/person", data)
      .await
  }

  /// Purges a community from the database.
  ///
  /// HTTP POST /admin/purge/community
  pub async fn purge_community(&self, data: PurgeCommunity) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "admin/purge/community", data)
      .await
  }

  /// Purges a post from the database.
  ///
  /// HTTP POST /admin/purge/post
  pub async fn purge_post(&self, data: PurgePost) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "admin/purge/post", data)
      .await
  }

  /// Purges a comment from the database.
  ///
  /// HTTP POST /admin/purge/comment
  pub async fn purge_comment(&self, data: PurgeComment) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "admin/purge/Comment", data)
      .await
  }

  /// Adds a new tagline to the site.
  ///
  /// HTTP POST /admin/tagline
  pub async fn create_tagline(&self, data: CreateTagline) -> LemmyResult<TaglineResponse> {
    self.make_request(Method::POST, "admin/tagline", data).await
  }

  /// Updates an existing tagline.
  ///
  /// HTTP PUT /admin/tagline
  pub async fn update_tagline(&self, data: UpdateTagline) -> LemmyResult<TaglineResponse> {
    self.make_request(Method::PUT, "admin/tagline", data).await
  }

  /// Deletes an existing tagline.
  ///
  /// HTTP DELETE /admin/tagline/delete
  pub async fn delete_tagline(&self, data: DeleteTagline) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "admin/tagline/delete", data)
      .await
  }

  /// Gets the site's taglines.
  ///
  /// HTTP GET /admin/tagline/list
  pub async fn list_taglines(&self, data: ListTaglines) -> LemmyResult<ListTaglinesResponse> {
    self
      .make_request(Method::GET, "admin/tagline/list", data)
      .await
  }

  /// Bans a person from your instance.
  ///
  /// HTTP POST /admin/ban
  pub async fn ban_from_site(&self, data: BanPerson) -> LemmyResult<PersonResponse> {
    self.make_request(Method::POST, "admin/ban", data).await
  }

  /// Lists users of your site.
  ///
  /// HTTP GET /admin/users
  pub async fn list_users(&self, data: AdminListUsers) -> LemmyResult<AdminListUsersResponse> {
    self.make_request(Method::GET, "admin/users", data).await
  }

  /// Leave your instance's admin team.
  ///
  /// HTTP POST /admin/leave
  pub async fn leave_admin(&self) -> LemmyResult<GetSiteResponse> {
    self.make_request(Method::POST, "admin/leave", ()).await
  }

  /// Defederates an instance from the current instance.
  ///
  /// HTTP POST /admin/instance/block
  pub async fn admin_block_instance(
    &self,
    data: AdminBlockInstanceParams,
  ) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "admin/instance/block", data)
      .await
  }

  /// Allows a given instance to interact with the current instance.
  ///
  /// HTTP POST /admin/instance/allow
  pub async fn admin_allow_instance(
    &self,
    data: AdminAllowInstanceParams,
  ) -> LemmyResult<SuccessResponse> {
    self
      .make_request(Method::POST, "admin/instance/allow", data)
      .await
  }

  /// Gets all media posted on an instance. Only usable by the instance's admins.
  ///
  /// HTTP GET /image/list
  pub async fn list_all_media(&self, data: ListMedia) -> LemmyResult<ListMediaResponse> {
    self.make_request(Method::GET, "image/list", data).await
  }

  /// Deletes an image from the instance.
  ///
  /// **Can only be used by instance admins**
  ///
  /// HTTP DELETE /image
  pub async fn delete_image_admin(&self, data: DeleteImageParams) -> LemmyResult<SuccessResponse> {
    self.make_request(Method::DELETE, "image", data).await
  }
}
