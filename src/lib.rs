use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};

/// This is a custom extractor for Axum that extracts the version, via the `X-Version` request header.
/// If the `X-Version` header is present, it returns it.
/// If the `X-Version` header is not present, it defaults to `latest`.
///
/// # Links
///
/// https://docs.rs/axum/latest/axum/index.html
/// https://docs.rs/axum/latest/axum/extract/index.html#defining-custom-extractors
///
/// # Author
///
/// François GRUCHALA <francois@nebeto.xyz>
///
/// # Examples
///
/// ```rust
/// use axum::{routing::get, Router};
/// use version_middleware::ExtractVersion;
///
/// async fn handler(ExtractVersion(version): ExtractVersion) {
///     println!("Version: {}", version);
/// }
///
/// let app = Router::<()>::new().route("/foo", get(handler));
/// ```
#[derive(Debug, Clone)]
pub struct ExtractVersion(pub String);

const HEADER_X_VERSION: &str = "x-version";
const HEADER_X_VERSION_DEFAULT: &str = "latest";

impl<S> FromRequestParts<S> for ExtractVersion
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let version = parts.headers.get(HEADER_X_VERSION);

        match version {
            Some(version) => Ok(ExtractVersion(
                version.clone().to_str().unwrap().trim().to_lowercase(),
            )),
            None => Ok(ExtractVersion(HEADER_X_VERSION_DEFAULT.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, extract::FromRequestParts, http::Request};

    use crate::ExtractVersion;

    #[tokio::test]
    async fn test_lib_extract_version_with_header_ok_one() {
        let request = Request::builder()
            .header("x-version", "v1.0.0")
            .body(Body::empty())
            .unwrap();

        let mut parts = request.into_parts();

        let version = ExtractVersion::from_request_parts(&mut parts.0, &()).await;

        match version {
            Ok(version) => assert_eq!(version.0, "v1.0.0"),
            Err(_) => assert!(false, "Expected a valid version"),
        }
    }

    #[tokio::test]
    async fn test_lib_extract_version_with_header_ok_two() {
        let request = Request::builder()
            .header("X-VersIon", " prevIew ")
            .body(Body::empty())
            .unwrap();

        let mut parts = request.into_parts();

        let version = ExtractVersion::from_request_parts(&mut parts.0, &()).await;

        match version {
            Ok(version) => assert_eq!(version.0, "preview"),
            Err(_) => assert!(false, "Expected a valid version"),
        }
    }

    #[tokio::test]
    async fn test_lib_extract_version_without_header() {
        let request = Request::builder().body(Body::empty()).unwrap();

        let mut parts = request.into_parts();

        let version = ExtractVersion::from_request_parts(&mut parts.0, &()).await;

        match version {
            Ok(version) => assert_eq!(version.0, "latest"),
            Err(_) => assert!(false, "Expected a valid version"),
        }
    }
}
