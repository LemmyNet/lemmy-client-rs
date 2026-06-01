[![Crates.io Version](https://img.shields.io/crates/v/lemmy-client)](https://crates.io/crates/lemmy-client)
[![GitHub tag (latest SemVer)](https://img.shields.io/github/tag/LemmyNet/lemmy-client-rs.svg)](https://github.com/LemmyNet/lemmy-client-rs/tags)
[![Build Status](https://woodpecker.join-lemmy.org/api/badges/LemmyNet/lemmy-client-rs/status.svg)](https://woodpecker.join-lemmy.org/LemmyNet/lemmy-client-rs)
[![GitHub issues](https://img.shields.io/github/issues-raw/LemmyNet/lemmy-client-rs.svg)](https://github.com/LemmyNet/lemmy-client-rs/issues)
[![License](https://img.shields.io/github/license/LemmyNet/lemmy-client-rs.svg)](LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/LemmyNet/lemmy-client-rs?style=social)](https://github.com/LemmyNet/lemmy-client-rs/stargazers)
<a href="https://endsoftwarepatents.org/innovating-without-patents"><img style="height: 20px;" src="https://static.fsf.org/nosvn/esp/logos/patent-free.svg"></a>

<div align="center">
  <a href="https://join-lemmy.org" rel="noopener">
      <img src="https://raw.githubusercontent.com/LemmyNet/lemmy-ui/main/src/assets/icons/favicon.svg" alt="Lemmy logo" width="250px" height="250px"/>
  </a>
  <h1 align="center">lemmy-client</h1>
  <p align="center">A Rust HTTP client for <a href="https://github.com/LemmyNet/lemmy">Lemmy</a>. Uses the browser's built-in <a href="https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API">fetch API</a> when targeting WASM to keep the binary size small.</p>
</div>

## Usage

In your `Cargo.toml`:

```toml
[dependencies]
lemmy_client = "X.X.X"
```

An example:

```rust
use lemmy_client::{LemmyClient, ClientOptions};
use lemmy_api_common::account::auth::Login;

async fn get_site_test() {
  let mut client = LemmyClient::new(ClientOptions {
    domain: "lemmy.ml",
    secure: true
  });

  let res = client.get_site().await;
  assert!(res.is_ok());

  // Login
  let login = Login {
    username_or_email: "user".to_string().into(),
    password: "password".to_string().into(),
    stay_logged_in: None,
    totp_2fa_token: None,
  };
  let jwt = client.login(login).await?.jwt;
  if let Some(jwt) = jwt {
    client.set_jwt(&jwt.into_inner());
  };
}
```
