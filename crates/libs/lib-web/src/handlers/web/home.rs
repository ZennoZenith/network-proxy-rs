use crate::{error::Result, tera::render};
use axum::response::IntoResponse;
use tera::Context;
use tracing::debug;

pub async fn render_home() -> Result<impl IntoResponse> {
    debug!("{:<12} - web_home_handler", "HANDLER");

    let context = Context::new();
    render("home.html", &context).map(IntoResponse::into_response)
}
