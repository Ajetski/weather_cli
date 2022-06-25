use serde::Deserialize;
use std::error::Error;
use std::time::Duration;

use crate::geo;

pub async fn get_weather_data(
    client: &reqwest::Client,
    geo_data: geo::Geo,
    api_key: &str,
) -> Result<Response, Box<dyn Error>> {
    let data = client
        .get(format!("https://api.openweathermap.org/data/2.5/onecall?lon={}&lat={}&units=metric&exclude=minutely,hourly,alerts&appid={}", geo_data.lon, geo_data.lat, api_key))
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .json::<Response>()
        .await?;
    Ok(data)
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub current: CurrentReport,
    pub daily: Vec<DailyReport>,
}

#[derive(Deserialize, Debug)]
pub struct CurrentReport {
    pub dt: u64,
    pub temp: f32,
    pub feels_like: f32,
    pub weather: Vec<Weather>,
}

#[derive(Deserialize, Debug)]
pub struct DailyReport {
    pub sunrise: u64,
    pub sunset: u64,
    pub temp: DailyTemp,
    pub weather: Vec<Weather>,
}

#[derive(Deserialize, Debug)]
pub struct DailyTemp {
    pub min: f32,
    pub max: f32,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub description: String,
}

