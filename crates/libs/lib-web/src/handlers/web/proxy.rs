use crate::{error::Result, tera::render};
use axum::response::IntoResponse;
use tera::Context;
use tracing::debug;

pub async fn render_proxy() -> Result<impl IntoResponse> {
    debug!("{:<12} - web_proxy_handler", "HANDLER");

    let context = Context::new();
    render("routes/proxy.html", &context).map(IntoResponse::into_response)
}
