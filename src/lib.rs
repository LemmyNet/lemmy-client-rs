use crate::{error::Error, forms::LemmyForm, response::LemmyResponse};
use cfg_if::cfg_if;
use http::method::Method;
use std::fmt;
use lemmy_api_common::{
    comment::*,
    community::*,
    custom_emoji::*,
    person::*,
    post::*,
    private_message::*,
    site::*
};

mod error;
mod forms;
mod response;
mod utils;

type LemmyResult<R> = Result<R, Error>;

struct LemmyRequest<R: LemmyForm> {
    pub body: Option<R>,
    pub jwt: Option<String>,
}

// impl<R: LemmyForm> LemmyRequest<R> {
//     pub fn from_jwt(jwt: Option<String>) -> Self {
//         Self {
//             body: None::<R>,
//             jwt,
//         }
//     }
// }

impl<R: LemmyForm> From<R> for LemmyRequest<R> {
    fn from(body: R) -> Self {
        LemmyRequest {
            body: Some(body),
            jwt: None,
        }
    }
}

mod private_trait {
    use crate::LemmyResult;

    use super::{LemmyForm, LemmyRequest, LemmyResponse, Method};

    pub trait LemmyClient {
        async fn make_request<Response, Form, Request>(
            &self,
            method: Method,
            path: &str,
            form: Request,
        ) -> LemmyResult<Response>
        where
            Response: LemmyResponse,
            Form: LemmyForm,
            Request: Into<LemmyRequest<Form>>;
    }
}

macro_rules! client_fn {
    ($name:ident, $method:expr, $path:expr, $form:ty, $response:ty) => {
        async fn $name<T>(&self, form: T) -> LemmyResult<$response>
            where
                T: Into<LemmyRequest<$form>>
        {
            self.make_request($method, $path, form).await
        }
    };
}

trait LemmyClient: private_trait::LemmyClient {
    async fn get_site(&self) -> LemmyResult<GetSiteResponse>
        {
            // The type of the request form doesn't matter here because this endpoint doesn't take arguments
            self.make_request(Method::GET, "site", LemmyRequest::<GetSiteMetadata>{
                body: None,
                jwt: None
            }).await
        }
    client_fn!(create_site, Method::POST, "site", CreateSite, GetSiteResponse);
    client_fn!(edit_site, Method::PUT, "site", EditSite, GetSiteResponse);
    client_fn!(get_modlog, Method::GET, "modlog", GetModlog, GetModlogResponse);
    client_fn!(search, Method::GET, "search", Search, SearchResponse);
    client_fn!(resolve_object, Method::GET, "resolve_object", ResolveObject, ResolveObjectResponse);
    client_fn!(get_community, Method::GET, "community", GetCommunity, GetCommunityResponse);
    client_fn!(create_community, Method::POST, "community", CreateCommunity, GetCommunityResponse);
    client_fn!(edit_community, Method::PUT, "community", EditCommunity, GetCommunityResponse);
    client_fn!(hide_community, Method::PUT, "community/hide", HideCommunity, GetCommunityResponse);
    client_fn!(list_communities, Method::GET, "community/list", ListCommunities, ListCommunitiesResponse);
    client_fn!(follow_community, Method::POST, "community/follow", FollowCommunity, GetCommunityResponse);
    client_fn!(block_community, Method::POST, "community/block", BlockCommunity, GetCommunityResponse);
    client_fn!(delete_community, Method::POST, "community/delete", DeleteCommunity, GetCommunityResponse);
    client_fn!(remove_community, Method::POST, "community/remove", RemoveCommunity, GetCommunityResponse);
    client_fn!(transfer_community, Method::POST, "community/transfer", TransferCommunity, GetCommunityResponse);
    client_fn!(ban_from_community, Method::POST, "community/ban_user", BanFromCommunity, BanFromCommunityResponse);
    client_fn!(add_mod_to_community, Method::POST, "community/mod", AddModToCommunity, AddModToCommunityResponse);
    client_fn!(get_federated_instances, Method::GET, "federated_instances", FederatedInstances, GetFederatedInstancesResponse);
    client_fn!(get_post, Method::GET, "post", GetPost, GetPostResponse);
    client_fn!(create_post, Method::POST, "post", CreatePost, GetPostResponse);
    client_fn!(edit_post, Method::PUT, "post", EditPost, GetPostResponse);
    client_fn!(delete_post, Method::POST, "post/delete", DeletePost, GetPostResponse);
    client_fn!(remove_post, Method::POST, "post/remove", RemovePost, GetPostResponse);
    client_fn!(mark_post_as_read, Method::POST, "post/mark_as_read", MarkPostAsRead, GetPostResponse);
    client_fn!(lock_post, Method::POST, "post/lock", LockPost, GetPostResponse);
    client_fn!(feature_post, Method::POST, "post/feature", FeaturePost, GetPostResponse);
    client_fn!(list_posts, Method::GET, "post/list", GetPosts, GetPostsResponse);
    client_fn!(like_post, Method::POST, "post/like", CreatePostLike, GetPostResponse);
    client_fn!(list_post_likes, Method::GET, "post/like/list", ListPostLikes, ListPostLikesResponse);
    client_fn!(save_post, Method::PUT, "post/save", SavePost, GetPostResponse);
    client_fn!(report_post, Method::POST, "post/report", CreatePostReport, PostReportResponse);
    client_fn!(resolve_post_report, Method::PUT, "post/report/resolve", ResolvePostReport, PostReportResponse);
    client_fn!(list_post_reports, Method::GET, "post/report/list", ListPostReports, ListPostReportsResponse);
    client_fn!(get_post_url_metadata, Method::GET, "post/site_metadata", GetSiteMetadata, GetSiteMetadataResponse);
}

trait MaybeBearerAuth {
    fn maybe_bearer_auth(self, token: Option<impl fmt::Display>) -> Self;
}

cfg_if! {
  if #[cfg(target_arch = "wasm32")] {
        use gloo_net::http::{Request, RequestBuilder};
    pub struct Fetch;

        impl MaybeBearerAuth for RequestBuilder {
           fn maybe_bearer_auth(self, token: Option<impl fmt::Display>) -> Self {
                if let Some(token) = token {
                    self.header("Authorization", format!("Bearer {token}").as_str())
                } else {
                    self
                }
            }
        }

    impl private_trait::LemmyClient for Fetch {
      async fn make_request<Response, Form, Req>(
                &self,
                method: Method,
                path: &str,
                req: Req,
            ) -> LemmyAppResult<Response>
            where
                Response: LemmyResponse,
                Form: LemmyForm,
                Req: Into<LemmyRequest<Form>>
            {
                let LemmyRequest { body, .. } = req.into();
                let route = &build_route(path);
                let jwt = get("jwt").and_then(Result::ok);

                // let abort_controller = AbortController::new().ok();
                // let abort_signal = abort_controller.as_ref().map(AbortController::signal);
                // leptos::on_cleanup( move || {
                //     if let Some(abort_controller) = abort_controller {
                //         abort_controller.abort()
                //     }
                // });

                match method {
                    Method::GET =>
                        Request::get(&build_fetch_query(path, body))
                            .maybe_bearer_auth(jwt.as_deref())
                            .abort_signal(abort_signal.as_ref())
                            .build()
                            .expect_throw("Could not parse query params"),
                    Method::POST =>
                        Request::post(route)
                            .maybe_bearer_auth(jwt.as_deref())
                            .abort_signal(abort_signal.as_ref())
                            .json(&body)
                            .expect_throw("Could not parse json body"),
                    Method::PUT =>
                        Request::put(route)
                            .maybe_bearer_auth(jwt.as_deref())
                            .abort_signal(abort_signal.as_ref())
                            .json(&body)
                            .expect_throw("Could not parse json body"),
                    _ => unreachable!("This crate does not use other HTTP methods.")
                }.send().await?.json::<Response>().await.map_err(Into::into)
            }
    }
  } else {
        impl MaybeBearerAuth for awc::ClientRequest {
            fn maybe_bearer_auth(self, token: Option<impl fmt::Display>) -> Self {
                if let Some(token) = token {
                    self.bearer_auth(token)
                } else {
                    self
                }
            }
        }


        impl private_trait::LemmyClient for awc::Client {
            async fn make_request<Response, Form, Request>(
                &self,
                method: Method,
                path: &str,
                req: Request,
            ) -> LemmyResult<Response>
            where
                Response: LemmyResponse,
                Form: LemmyForm,
                Request: Into<LemmyRequest<Form>>
            {
                let LemmyRequest {body, jwt} = req.into();
                let route = path;

                match method {
                    Method::GET =>
                        self
                            .get(route)
                            .maybe_bearer_auth(jwt)
                            .query(&body)?
                            .send(),
                    Method::POST =>
                        self
                            .post(route)
                            .maybe_bearer_auth(jwt)
                            .send_json(&body),
                    Method::PUT =>
                        self
                            .put(route)
                            .maybe_bearer_auth(jwt)
                            .send_json(&body),
                    _ => unreachable!("This crate does not use other HTTP methods.")
                }.await?.json::<Response>().await.map_err(Into::into)
            }
        }
  }
}
