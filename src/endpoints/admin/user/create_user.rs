use crate::middleware::auth::auth_middleware;
use crate::models::LoginDetails;
use crate::utils::calculate_hash;
use crate::{models::AppState, utils::get_error};
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use axum_auto_routes::route;
use mongodb::bson::doc;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

pub_struct!(Deserialize; CreateCustom {
    user: String,
    password: String,
});

#[route(post, "/admin/user/create", auth_middleware)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Extension(sub): Extension<String>,
    body: Json<CreateCustom>,
) -> impl IntoResponse {
    if sub != "super_user" {
        return get_error("Operation not allowed with your account".to_string());
    };

    let collection = state.db.collection::<LoginDetails>("login_details");
    let hashed_password = calculate_hash(&body.password);

    let new_document = LoginDetails {
        user: body.user.clone(),
        code: hashed_password.to_string(),
    };

    // insert document to boost collection
    return match collection.insert_one(new_document, None).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({"message": "User added successfully"})).into_response(),
        )
            .into_response(),
        Err(_e) => get_error("Error creating user".to_string()),
    };
}
