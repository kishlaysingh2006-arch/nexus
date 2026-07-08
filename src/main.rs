use axum::{routing::get, Router};
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct WeatherResponse {
    current_weather: CurrentWeather,
}

#[derive(Deserialize)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
}

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(system_status))
        .route("/harvest", get(harvest_data));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server pulsing on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}

async fn system_status() -> &'static str {
    "The backend is awake."
}

async fn harvest_data() -> String {
    let url = "https://api.open-meteo.com/v1/forecast?latitude=25.7464&longitude=82.6837&current_weather=true";

    let response = reqwest::get(url).await.unwrap();
    
    let data: WeatherResponse = response.json().await.unwrap();

    format!(
        "Harvest complete. Live target data -> Temp: {}°C, Wind: {} km/h",
        data.current_weather.temperature, 
        data.current_weather.windspeed
    )
}