use std::sync::{Arc, Mutex};

use axum::{extract::State, response::IntoResponse, Json};

use crate::{json::JsonResponse, AppState};

pub mod artist;

pub async fn status(State(state): State<Arc<Mutex<AppState>>>) -> impl IntoResponse {
    let status = state.lock().unwrap().ready.clone();

    match status {
        crate::ReadyState::Ready => {
            let resp = JsonResponse {
                success: true,
                message: String::from("Its up"),
            };

            Json(resp)
        }
        crate::ReadyState::NotConnected => {
            let resp = JsonResponse {
                success: false,
                message: String::from("Its not up"),
            };

            Json(resp)
        }
    }
}
