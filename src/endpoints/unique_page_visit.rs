use std::net::SocketAddr;
use crate::{
    models::{AppState},
    utils::get_error,
};
use axum::{
    extract::{Query, State, ConnectInfo},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_auto_routes::route;
use mongodb::bson::doc;
use serde::Deserialize;
use std::sync::Arc;
use chrono::Utc;
use mongodb::Collection;
use mongodb::options::UpdateOptions;
use serde_json::json;
use crate::models::UniquePageVisit;

#[derive(Deserialize)]
pub struct GetQuestsQuery {
    page_id: String,
}

#[route(get, "/unique_page_visit", crate::endpoints::unique_page_visit)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<GetQuestsQuery>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let id = query.page_id;
    let unique_viewers_collection: Collection<UniquePageVisit> =
        state.db.collection("unique_viewers");
    let created_at = Utc::now().timestamp_millis();
    let filter = doc! { "viewer_ip": addr.to_string(), "viewed_page_id": &id };
    let update = doc! { "$setOnInsert": { "viewer_ip": addr.to_string(), "viewed_page_id": &id,"timestamp":created_at } };
    let options = UpdateOptions::builder().upsert(true).build();

    match unique_viewers_collection.update_one(filter, update, options)
        .await {
        Ok(_) => (StatusCode::OK, Json(json!({"res": true}))).into_response(),
        Err(_) => get_error("unable to detect page visit status".to_string()),
    }
}
