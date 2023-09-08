use tabled::settings::Style;

use crate::api::{ForecastResponse, GeocodingResponse, WeatherCode};

pub async fn today(city: String) {
    let geocoding_url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1&language=en&format=json",
        city
    );
    let geocoding_response = reqwest::get(geocoding_url)
        .await
        .unwrap()
        .json::<GeocodingResponse>()
        .await
        .unwrap();
    let city_info = geocoding_response.results.expect("No results found");
    let city_info = city_info.first().unwrap();

    let weather_url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m,relativehumidity_2m,apparent_temperature,precipitation_probability,precipitation&daily=weathercode,temperature_2m_max,temperature_2m_min&timezone=auto&forecast_days=1",
        city_info.latitude, city_info.longitude
    );
    let weather_response = reqwest::get(weather_url)
        .await
        .unwrap()
        .json::<ForecastResponse>()
        .await
        .unwrap();

    let weather_code = weather_response.daily.weathercode.first().unwrap();
    let weather_code = match weather_code {
        WeatherCode::ThunderstormWithSlightHail | WeatherCode::ThunderstormWithHeavyHail
            if !weather_response.timezone.starts_with("Europe") =>
        {
            &WeatherCode::Thunderstorm
        }
        _ => weather_code,
    };

    println!(
        "Weather for today in {}, {}:",
        city_info.name, city_info.country
    );
    println!("Status: {}", weather_code);
    println!(
        "Low: {}°C",
        weather_response.daily.temperature_2m_min.first().unwrap().round()
    );
    println!(
        "High: {}°C",
        weather_response.daily.temperature_2m_max.first().unwrap().round()
    );

    println!();
    println!("Hourly forecast:");
    let mut builder = tabled::builder::Builder::default();
    builder.set_header(["Time", "Temperature", "Feels Like", "Humidity", "Precipitation Chance", "Precipitation"]);

    weather_response.hourly.time[0..24]
        .iter()
        .zip(
            weather_response.hourly.temperature_2m[0..24]
                .iter()
                .zip(weather_response.hourly.apparent_temperature[0..24].iter())
                .zip(weather_response.hourly.relativehumidity_2m[0..24].iter())
                .zip(weather_response.hourly.precipitation_probability[0..24].iter())
                .zip(weather_response.hourly.precipitation[0..24].iter()),
        )
        .for_each(
            |(
                time,
                ((((temp, feels_like), humidity), precipitation_probability), precipitation),
            )| {
                let time = time.split("T").collect::<Vec<_>>()[1]
                    .split(":")
                    .collect::<Vec<_>>()[0]
                    .parse()
                    .unwrap();
                let time = match time {
                    0 => "12 AM".to_string(),
                    1..12 => format!("{} AM", time),
                    12 => "12 PM".to_string(),
                    13..24 => format!("{} PM", time - 12),
                    _ => "Unknown".to_string(),
                };

                builder.push_record([
                    time,
                    format!("{}°C", temp.round()),
                    format!("{}°C", feels_like.round()),
                    format!("{}%", humidity),
                    format!("{}%", precipitation_probability),
                    format!("{}mm", precipitation.round()),
                ]);
            },
        );

    let builder = builder.index().column(0).name(None);
    let mut table = builder.build();
    table.with(Style::rounded());
    println!("{}", table);
}
