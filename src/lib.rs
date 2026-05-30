#![warn(missing_docs)]
#![doc(
  html_favicon_url = "https://raw.githubusercontent.com/LemmyNet/lemmy-ui/main/src/assets/icons/favicon.svg"
)]
#![doc(
  html_logo_url = "https://raw.githubusercontent.com/LemmyNet/lemmy-ui/main/src/assets/icons/favicon.svg"
)]
#![doc(issue_tracker_base_url = "https://github.com/LemmyNet/lemmy-client-rs/issues/")]
//! A Rust HTTP client for Lemmy.
//! Also supports WASM, including use in the browser.
//! ## Example
//! ```
//! use lemmy_client::{LemmyClient, ClientOptions};
//!
//! async fn get_site_test() {
//!   let client = LemmyClient::new(ClientOptions {
//!     domain: "lemmy.ml",
//!     secure: true
//!   });
//!   let res = client.get_site().await;
//!
//!   assert!(res.is_ok());
//!
//!   // Login
//!   let login = Login {
//!     username_or_email: "user",
//!     password: "password",
//!     stay_logged_in: None,
//!     totp_2fa_token: None,
//!   };
//!   let jwt = client.login(login).await?.jwt;
//!   if let Some(jwt) = jwt {
//!     client.set_jwt(&jwt.into_inner());
//!   };
//! }
//! ```

mod client_options;
mod endpoints;
mod lemmy_client;

pub use client_options::ClientOptions;
pub use lemmy_api_common;
pub use lemmy_client::{LemmyClient, LemmyResult};
