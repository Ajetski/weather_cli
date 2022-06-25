use reqwest;
use serde::Deserialize;
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = "PUT_API_KEY_HERE"; // get api key from https://openweathermap.org/
    let client = reqwest::Client::new();
    let data = client
        .get(format!("https://api.openweathermap.org/data/2.5/onecall?lon=-73.454&lat=41.3948&units=metric&exclude=minutely,hourly,alerts&appid={}", api_key))
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .json::<Response>()
        .await?;
    println!(
        "{} {}°C ({}°C) 🥶 {}°C 🥵 {}°C",
        convert_to_emoji(
            data.current.weather[0].description.as_str(),
            is_daytime(&data)
        ),
        data.current.temp,
        data.current.feels_like,
        data.daily[0].temp.min,
        data.daily[0].temp.max
    );

    Ok(())
}

fn is_daytime(data: &Response) -> bool {
    data.current.dt >= data.daily[0].sunrise || data.current.dt <= data.daily[0].sunset
}

fn convert_to_emoji(weather: &str, is_daytime: bool) -> &str {
    if weather == "clear sky" {
        if is_daytime {
            "☀️"
        } else {
            "🌕"
        }
    } else if weather == "few clouds" {
        if is_daytime {
            "🌤️"
        } else {
            "🌕"
        }
    } else if weather == "scattered clouds" {
        if is_daytime {
            "⛅"
        } else {
            "☁️"
        }
    } else if weather == "broken clouds" {
        "☁️"
    } else if weather == "shower rain" {
        "🌧️"
    } else if weather == "rain" {
        "☔"
    } else if weather == "thunderstorm" {
        "⛈️"
    } else if weather == "snow" {
        "🌨️"
    } else if weather == "mist" {
        "🌈"
    } else {
        "?"
    }
}

#[derive(Deserialize, Debug)]
struct Response {
    current: CurrentReport,
    daily: Vec<DailyReport>,
}

#[derive(Deserialize, Debug)]
struct CurrentReport {
    dt: u64,
    temp: f32,
    feels_like: f32,
    weather: Vec<Weather>,
}

#[derive(Deserialize, Debug)]
struct DailyReport {
    sunrise: u64,
    sunset: u64,
    temp: DailyTemp,
    weather: Vec<Weather>,
}

#[derive(Deserialize, Debug)]
struct DailyTemp {
    min: f32,
    max: f32,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

