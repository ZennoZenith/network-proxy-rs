use axum::Router;
use lib_core::model::ModelManager;

pub fn routes(_mm: ModelManager) -> Router {
    Router::new()
}
