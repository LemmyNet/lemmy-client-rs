<div align="center">
  <img src="https://img.shields.io/github/license/LemmyNet/lemmy-client-rs?style=for-the-badge" alt="License" />
  <img src="https://img.shields.io/github/last-commit/LemmyNet/lemmy-client-rs?style=for-the-badge&logo=GitHub" alt="Last commit" />
  <img src="https://img.shields.io/github/stars/LemmyNet/lemmy-client-rs?style=for-the-badge&logo=GitHub" alt="Github stars" />
  <img src="https://img.shields.io/crates/v/lemmy-client?style=for-the-badge" alt="Latest version" />
  <img src="https://img.shields.io/crates/dv/lemmy-client?style=for-the-badge" alt="Downloads for latest version" />
  <img src="https://img.shields.io/github/languages/code-size/LemmyNet/lemmy-client-rs?style=for-the-badge&logo=Rust" alt="Library size in bytes" />
</div>
<div align="center">
  <a href="https://join-lemmy.org" rel="noopener">
      <img src="https://raw.githubusercontent.com/LemmyNet/lemmy-ui/main/src/assets/icons/favicon.svg" alt="Lemmy logo" width="250px" height="250px"/>
  </a>
  <h1 align="center">lemmy-client</h1>
  <p align="center">A Rust HTTP client for <a href="https://github.com/LemmyNet/lemmy">Lemmy</a>. Uses the browser's built-in <a href="https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API">fetch API</a> when targeting WASM to keep the binary size small.</p>
</div>

## IMPORTANT NOTICE

This crate now uses a different versioning scheme than before so as not to be too tied down to Lemmy releases. For Lemmy versions 0.19.4 and up, use versions 1.x.x. For Lemmy versions 0.19.3 and under, use versions 0.19.5 and up. This is confusing, but should become a non issue as Lemmy accumulates versions and fewer servers use Lemmy versions use 0.19.3 and lower.
