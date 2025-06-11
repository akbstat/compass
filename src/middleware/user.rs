use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use tracing::{error, info};

#[derive(Debug, Clone)]
pub struct CompassUser(pub String);

pub async fn compass_user_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, &'static str)> {
    let method = format!("{:?}", request.method());
    let request_path = request.uri().path();
    let user = headers
        .get("Compass-User")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    match user {
        Some(user) => {
            info!(user = user, method = method, path = request_path);
            request.extensions_mut().insert(CompassUser(user));
            Ok(next.run(request).await)
        }
        None => {
            error!(
                message = "Missing key-pair of Compass-User in request header",
                method = method,
                path = request_path
            );
            Err((StatusCode::BAD_REQUEST, "Invalid User"))
        }
    }
}
