use request::Client;
use serde_json::json;

async fn send_data(data: serde_json::Value, token: &str) -> Result<(), reqwest::Error> {
    let url_server = "https://micro-red.ucuenca.edu.ec/api/v1/";
    let url_send = format!("{}{}/telemetry", url_server, token);

    let client = Client::new();
    let response = client.post(&url_send)
        .json(&data)
        .header("Content-Type", "application/json")
        .send()
        .await;

    response.map(|_| ())
}

