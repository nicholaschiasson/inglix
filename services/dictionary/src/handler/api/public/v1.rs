use askama::Template;
use axum::{
    extract::{Path, Query, State},
    http::{header::ACCEPT, HeaderMap, StatusCode},
    response::Html,
    routing::get,
    Json, Router,
};
use axum_extra::either::Either;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    dal::{self, SearchWord},
    model::Word,
    AppState,
};

pub fn router<S: AppState>() -> Router<S> {
    Router::new()
        .route("/words", get(get_words::<S>))
        .route("/words/:id", get(get_word_by_id::<S>))
}

async fn get_word_by_id<S: AppState>(
    Path(word_id): Path<Uuid>,
    State(state): State<S>,
) -> Result<Json<Word>, (StatusCode, String)> {
    dal::get_word_by_id(state.pool(), word_id.into())
        .await
        .map(Json)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum SearchWordFrom {
    English,
    Inglix,
}

#[derive(Clone, Debug, Deserialize)]
struct SearchWordQuery {
    from: Option<SearchWordFrom>,
    search: Option<String>,
}

impl TryFrom<SearchWordQuery> for SearchWord {
    type Error = ();

    fn try_from(value: SearchWordQuery) -> Result<Self, Self::Error> {
        match (value.from, value.search) {
            (Some(SearchWordFrom::English), Some(s)) => Ok(SearchWord::English(s)),
            (Some(SearchWordFrom::Inglix), Some(s)) => Ok(SearchWord::Inglix(s)),
            _ => Err(()),
        }
    }
}

#[derive(Template)]
#[template(path = "search-results.html")]
struct SearchResults {
    words: Vec<Word>,
}

impl From<Vec<Word>> for SearchResults {
    fn from(value: Vec<Word>) -> Self {
        Self { words: value }
    }
}

async fn get_words<S: AppState>(
    headers: HeaderMap,
    Query(params): Query<SearchWordQuery>,
    State(state): State<S>,
) -> Result<Either<Json<Vec<Word>>, Html<String>>, (StatusCode, String)> {
    dal::get_words(state.pool(), params.try_into().ok())
        .await
        .map(|words| match headers[ACCEPT].to_str() {
            Ok("application/json") => Either::E1(Json(words)),
            _ => Either::E2(Html(SearchResults::from(words).render().unwrap())),
        })
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
}
