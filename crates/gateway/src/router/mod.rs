use axum::Router;
use axum::routing::get;

use crate::configure::AppConfig;
use crate::server::state::AppState;

pub fn create_router(state: AppState) -> Router {
    let router = Router::new();

    let router = router.route("/", get(|| async { "Main page" }));
    let router = router.route("/health", get(|| async { "Ok" }));

    router.with_state(state)
}
