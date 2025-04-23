# Rust Axum Middleware - Extract Version from Header

Custom extractor for [Rust](https://www.rust-lang.org/) [Axum](https://docs.rs/axum/latest/axum/) to extract the version from an HTTP header `X-Version`.
Works **ONLY** with [Rust](https://www.rust-lang.org/) [Axum](https://docs.rs/axum/latest/axum/).

## Usage

```rust
use axum::{routing::get, Router};
use version_middleware::ExtractVersion;

async fn handler(ExtractVersion(version): ExtractVersion) {
    println!("Version: {:?}", version);
}

let app = Router::<()>::new().route("/foo", get(handler));
```

The extracted value is :

- `trim` to clean extra spaces, before and after ;
- `lowercase` to standardize and make it more resilient to implementation errors.

## Samples

### Extract version if the header is explicitly set

```shell
curl -H "X-Version: v1.0.0" http://api.nebeto.xyz/foo
curl -H "x-version: preView" http://api.nebeto.xyz/foo
curl -H "X-VeRSion: latest" http://api.nebeto.xyz/foo
```

Will give respectively `v1.0.0`, `preview` and `latest`.

### Extract version if the header is missing

```shell
curl http://api.nebeto.xyz/foo
```

Will give by default `latest`.

## Contact

For any question or feature suggestion, you can take a look and open, if necessary, a new [discussion](https://github.com/nebetoxyz/rust-version-middleware--lib/discussions).

For any bug, you can take a look to our active issues and open, if necessary, a new [issue](https://github.com/nebetoxyz/rust-version-middleware--lib/issues).
