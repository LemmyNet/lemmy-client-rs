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
//! # use lemmy_client::{LemmyClient, ClientOptions};
//!
//! # async fn get_site_test() {
//!   let client = LemmyClient::new(ClientOptions {
//!     domain: "lemmy.ml",
//!     secure: true
//!   });
//!   let res = client.get_site(()).await;
//!
//!   assert!(res.is_ok());
//! # }
//! ```
//! <div class="warning">
//! This crate now uses a different versioning scheme than before so as not to be too tied down to
//! Lemmy releases. For Lemmy versions 0.19.4 and up, use versions 1.x.x.
//! For Lemmy versions 0.19.3 and under, use versions 0.19.5 and up.
//!
//! This is confusing, but should become a non issue as Lemmy accumulates versions and fewer servers
//! use Lemmy versions use 0.19.3 and lower.
//! </div>

mod client_options;
mod endpoint_methods;
mod form;
mod lemmy_client;
mod macros;
mod response;

pub use client_options::ClientOptions;
pub use form::LemmyRequest;
pub use lemmy_api_common;
pub use lemmy_client::LemmyClient;
