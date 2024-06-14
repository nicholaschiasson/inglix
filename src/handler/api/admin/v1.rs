use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, post},
    Json, Router,
};
use serde::Deserialize;
use uuid::{fmt::Hyphenated, Uuid};

use crate::{dal, model::Word, AppRouter, AppState};

#[derive(Deserialize)]
struct CreateWord {
    english_spelling: String,
    inglix_spelling: String,
}

impl From<CreateWord> for Word {
    fn from(val: CreateWord) -> Self {
        Word::new(&val.english_spelling, &val.inglix_spelling)
    }
}

pub fn router() -> AppRouter {
    Router::new()
        .route("/words", post(create_word))
        .route("/words/:id", delete(delete_word_by_id))
}

async fn create_word(
    State(AppState { pool }): State<AppState>,
    Json(payload): Json<CreateWord>,
) -> Result<(StatusCode, Json<Hyphenated>), (StatusCode, String)> {
    dal::create_word(&pool, &payload.into())
        .await
        .map(|id| (StatusCode::CREATED, Json(id)))
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

async fn delete_word_by_id(
    Path(word_id): Path<Uuid>,
    State(AppState { pool }): State<AppState>,
) -> Result<Json<Hyphenated>, (StatusCode, String)> {
    dal::delete_word_by_id(&pool, word_id.into())
        .await
        .map(Json)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}
