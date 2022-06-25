use reqwest;
use std::error::Error;

pub mod geo;
pub mod weather;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = "PUT_API_KEY_HERE"; // get api key from https://openweathermap.org/
    let location = "Danbury";
    let client = reqwest::Client::new();
    let geo_data = geo::get_geo_data(&client, location, api_key).await?;
    let data = weather::get_weather_data(&client, geo_data, api_key).await?;
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

fn is_daytime(data: &weather::Response) -> bool {
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
