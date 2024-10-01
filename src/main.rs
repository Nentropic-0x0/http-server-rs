//! HTTP SERVER IN AGEAN-AGENTS
//! 
//! Fetches alerts, sends alerts
//! Uses the Reqwest framework
//! 

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use anyhow::Result;
use dotenv::dotenv;


fn load_env() {
    dotenv().ok();
}

#[derive(Serialize, Deserialize, Debug)]
struct Alert {
    message: String,
    threat_level: u8,
}

pub async fn get_alerts(crowdsec_url: &str) -> Result<Vec<Alert>> {
    let client = Client::new();
    let response = client
        .get(crowdsec_url)
        .send()
        .await?
        .json::<Vec<Alert>>()
        .await?;
    Ok(response)
}

pub async fn send_alert(crowdsec_url: &str, alert: &Alert) -> Result<()> {
    let client = Client::new();
    client
        .post(format!("{}/alerts", crowdsec_url))
        .json(&alert)
        .send()
        .await?;

    Ok(())
}

/// Creating HTTP Server

use warp::http::StatusCode;
use warp::Filter;
use std::convert::Infallible;
use std::io::{stdout, Write};

use curl::easy::{Easy, List};
use std::env;


fn easy_url() {
    let mut easy = Easy::new();
    let crowdsec_api_key = env::var("CROWDSEC_API_KEY").expect("CROWDSEC_API_KEY not set");
    
    let mut headers = List::new();
    headers.append(&format!("x-api-key: {}", crowdsec_api_key)).unwrap();
    easy.http_headers(headers).unwrap();
    
    easy.url("https://cti.api.crowdsec.net/v2/smoke/185.7.214.104").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let alerts = warp::path("alerts");

    //GET
    let get_alerts = alerts
        .and(warp::get())
        .and_then(handle_get_alerts);

    //POST
    let post_alert = alerts
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_post_alerts);

    // Routes
    let routes = get_alerts.or(post_alert);

    //Start Server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

/// Handlers
 

async fn handle_get_alerts() -> Result<impl warp::Reply, Infallible> {
    let crowdsec_url = "https://cti.api.crowdsec.net/v2/alerts"; // Replace with actual URL or configuration
    match crate::get_alerts(crowdsec_url).await {
        Ok(alerts) => Ok(warp::reply::with_status("Alert retrieved", warp::http::StatusCode::OK)),
        Err(_) => Ok(warp::reply::with_status("Failed to fetch alerts", warp::http::StatusCode::INTERNAL_SERVER_ERROR))
    }
}

async fn handle_post_alerts(alert: Alert) -> Result<impl warp::Reply, Infallible> {
    let crowdsec_url = "https://cti.api.crowdsec.net/v2/alerts"; // Replace with actual URL or configuration
    match crate::send_alert(crowdsec_url, &alert ).await {
        Ok(_) => Ok(warp::reply::with_status("Alert Sent", warp::http::StatusCode::OK)),
        Err(_) => Ok(warp::reply::with_status("Failed to send alert", warp::http::StatusCode::INTERNAL_SERVER_ERROR))
    }
}






