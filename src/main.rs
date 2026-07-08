use axum::{
    extract::State,
    routing::get,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration; 
use tokio::net::TcpListener;
use tokio::time::sleep; 

#[derive(Deserialize)]
struct WeatherResponse {
    current_weather: CurrentWeather,
}

#[derive(Deserialize, Serialize, Clone)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
}

#[derive(Serialize)]
struct AnalyticsReport {
    total_records: usize,
    average_temperature: f64,
    max_windspeed: f64,
}

type SharedState = Arc<Mutex<Vec<CurrentWeather>>>;

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(Mutex::new(Vec::new()));

    let worker_state = Arc::clone(&state);
    
    tokio::spawn(async move {
        loop {
            fetch_and_store(&worker_state).await;
            
            sleep(Duration::from_secs(5)).await;
        }
    });

    let app = Router::new()
        .route("/", get(system_status))
        .route("/vault", get(view_vault))
        .route("/analyze", get(analyze_data))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server pulsing on http://127.0.0.1:3000");
    println!("Automaton online. Ingesting local atmospheric data every 5 seconds...\n");
    
    axum::serve(listener, app).await.unwrap();
}

async fn fetch_and_store(state: &SharedState) {
    let url = "https://api.open-meteo.com/v1/forecast?latitude=25.7464&longitude=82.6837&current_weather=true";
    
    if let Ok(response) = reqwest::get(url).await {
        if let Ok(data) = response.json::<WeatherResponse>().await {
            
            let mut vault = state.lock().unwrap();
            vault.push(data.current_weather.clone());
            
            println!("Automaton heartbeat: Vault now holds {} records.", vault.len());
        }
    }
}

async fn system_status() -> &'static str {
    "The backend is awake."
}

async fn view_vault(State(state): State<SharedState>) -> Json<Vec<CurrentWeather>> {
    let vault = state.lock().unwrap();
    Json(vault.clone())
}

async fn analyze_data(State(state): State<SharedState>) -> Json<Option<AnalyticsReport>> {
    let vault = state.lock().unwrap();
    if vault.is_empty() { return Json(None); }

    let total_records = vault.len();
    let mut temp_sum = 0.0;
    let mut max_wind = 0.0_f64;

    for record in vault.iter() {
        temp_sum += record.temperature;
        if record.windspeed > max_wind { max_wind = record.windspeed; }
    }

    Json(Some(AnalyticsReport {
        total_records,
        average_temperature: temp_sum / (total_records as f64),
        max_windspeed: max_wind,
    }))
}