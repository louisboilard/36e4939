use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router
};

use serde::{Deserialize, Serialize};

type SharedState = Arc<Mutex<HashMap<String, Movie>>>;

#[derive(Clone)]
struct AppState {
    db: SharedState,
}

#[tokio::main]
async fn main() {
    let db: HashMap<String, Movie> = HashMap::new();
    let inner_state = Arc::new(Mutex::new(db));
    let state = AppState { db: inner_state };

    let app = Router::new()
        .route("/movie/:id", get(movie_by_id))
        .with_state(state.clone())
        .route("/movie", post(add_movie))
        .with_state(state.clone())
        .route("/test", get(|| async { "test." }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Movie {
    id: String,
    name: String,
    year: u16,
    was_good: bool,
}

/// Find a movie by its id, if it exists.
async fn movie_by_id(State(state): State<AppState>, Path(movie_id): Path<String>) -> axum::response::Response {
    let db = state.db.lock().unwrap();
    eprintln!("looking up id: {movie_id}");
    if let Some(movie) = db.get(&movie_id) {

        return (StatusCode::OK, Json(movie)).into_response();
    }
    (StatusCode::NOT_FOUND, ()).into_response()
}

/// Gets a movie and inserts in into the db
async fn add_movie(State(state): State<AppState>, Json(movie): Json<Movie>) -> StatusCode {
    let deserialized_movie = Movie {
        id: movie.id.clone(),
        name: movie.name.clone(),
        year: movie.year,
        was_good: movie.was_good,
    };

    let mut db = state.db.lock().unwrap();
    db.insert(movie.id.clone(), deserialized_movie);

    StatusCode::CREATED
}
