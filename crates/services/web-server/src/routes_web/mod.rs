use axum::{Router, routing::get};
use lib_core::model::ModelManager;
use lib_web::handlers::web::{auth, dashboard, home, proxy};

// region:    --- Modules
mod routes_fragmant;
pub mod routes_static;

// endregion: --- Modules

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/", get(home::render_home))
        .route("/login", get(auth::render_login))
        .route("/register", get(auth::render_register))
        .route("/dashboard", get(dashboard::render_dashboard))
        .route("/proxy", get(proxy::render_proxy))
        .nest_service("/fragmant", routes_fragmant::routes(mm.clone()))
        .with_state(mm)
}
