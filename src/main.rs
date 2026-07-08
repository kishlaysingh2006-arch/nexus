use axum::{
    extract::State,
    routing::get,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct WeatherResponse {
    current_weather: CurrentWeather,
}

#[derive(Deserialize, Serialize, Clone)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
}

type SharedState = Arc<Mutex<Vec<CurrentWeather>>>;

#[tokio::main]
async fn main() {

    let state: SharedState = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/", get(system_status))
        .route("/harvest", get(harvest_data))
        .route("/vault", get(view_vault))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server pulsing on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}

async fn system_status() -> &'static str {
    "The backend is awake."
}

async fn harvest_data(State(state): State<SharedState>) -> String {
    let url = "https://api.open-meteo.com/v1/forecast?latitude=25.7464&longitude=82.6837&current_weather=true";
    
    let response = reqwest::get(url).await.unwrap();
    let data: WeatherResponse = response.json().await.unwrap();

    let mut vault = state.lock().unwrap();
    
    vault.push(data.current_weather.clone());

    format!(
        "Data ingested. The vault now holds {} records.",
        vault.len()
    )
}

async fn view_vault(State(state): State<SharedState>) -> Json<Vec<CurrentWeather>> {
    let vault = state.lock().unwrap();
    Json(vault.clone())
}