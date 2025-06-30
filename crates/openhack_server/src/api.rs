pub use self::jwt::Keys;

use self::context::AppContext;
use self::response::{Response, ResponseWithHeaders};
use ::axum::{Extension, Router};
use ::openhack::OpenHack;

mod context;
mod handler;
mod jwt;
mod response;

pub fn routes(state: OpenHack, jwt_keys: Keys) -> Router<()> {
    use ::axum::routing::{get, post};
    use ::openhack::command::{
        create_event::CreateEvent, create_user::CreateUser, update_event::UpdateEvent,
        upsert_rsvp::UpsertRsvp,
    };
    use ::openhack::report::search_events::SearchEvents;

    Router::new()
        .route("/cmd/CreateUser", post(handler::command::<CreateUser>))
        .route("/cmd/CreateEvent", post(handler::command::<CreateEvent>))
        .route("/cmd/UpdateEvent", post(handler::command::<UpdateEvent>))
        .route("/cmd/UpsertRsvp", post(handler::command::<UpsertRsvp>))
        .route("/cmd/LoginUser", post(handler::sign_in))
        .route("/rpt/SearchEvents", get(handler::report::<SearchEvents>))
        .layer(Extension(jwt_keys))
        .with_state(state)
}
