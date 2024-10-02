use std::collections::HashMap;

use axum::{
    extract::State, response::IntoResponse, routing::{get, post}, Json, Router
};

use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct AppState {
    db: HashMap<String, Movie>
}

#[tokio::main]
async fn main() {
    let db: HashMap<String, Movie> = HashMap::new();
    let state = AppState {db};
    let app = Router::new().with_state(state.clone())
        .route("/movie/:id", get(movie_by_id)).with_state(state.clone())
        .route("/movie", post(add_movie)).with_state(state.clone())
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
async fn movie_by_id(State(state): State<AppState>, movie_id: String, ) -> impl IntoResponse {
    if let Some(movie) = state.db.get(&movie_id) {
        return Json(movie.clone());
    }
    Json(Movie::default())
}

/// Gets a movie and inserts in into the db
async fn add_movie(State(mut state): State<AppState>, movie: Json<Movie>) -> () {
    let deserialized_movie = Movie { id: movie.id.clone(), name: movie.name.clone(), year: movie.year, was_good: movie.was_good };
    state.db.insert(movie.id.clone(), deserialized_movie);
}
