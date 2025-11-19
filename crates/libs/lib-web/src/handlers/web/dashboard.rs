use crate::{error::Result, tera::render};
use axum::response::IntoResponse;
use tera::Context;
use tracing::debug;

pub async fn render_dashboard() -> Result<impl IntoResponse> {
    debug!("{:<12} - web_dashboard_handler", "HANDLER");

    let context = Context::new();
    render("routes/dashboard.html", &context).map(IntoResponse::into_response)
}
