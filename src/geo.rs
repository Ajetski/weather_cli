use serde::Deserialize;
use std::error::Error;
use std::time::Duration;

pub async fn get_geo_data(
    client: &reqwest::Client,
    location: &str,
    api_key: &str,
) -> Result<Geo, Box<dyn Error>> {
    let geo = client
        .get(format!(
            "http://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}",
            location, api_key
        ))
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .json::<Vec<Geo>>()
        .await?;
    Ok(geo[0].clone())
}

#[derive(Deserialize, Debug, Clone)]
pub struct Geo {
    pub name: String,
    pub lat: f32,
    pub lon: f32,
}
