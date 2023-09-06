use chrono::{Datelike, Duration, NaiveDate, Utc};
use clap::{Parser, Subcommand};
use serde::Deserialize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Predict the weather for a given city
    Predict {
        /// The city to predict the weather for
        city: String,
        /// The date to predict the weather for (format: YYYY-MM-DD)
        date: String,
    },
}

#[derive(Deserialize, Debug)]
struct GeocodingResponse {
    results: Option<Vec<GeocodingResult>>,
}

#[derive(Deserialize, Debug)]
struct GeocodingResult {
    name: String,
    country: String,
    latitude: f32,
    longitude: f32,
}

#[derive(Deserialize, Debug)]
struct HistoricalResponse {
    daily: HistoricalDaily,
}

#[derive(Deserialize, Debug)]
struct HistoricalDaily {
    time: Vec<NaiveDate>,
    temperature_2m_max: Vec<Option<f32>>,
    temperature_2m_min: Vec<Option<f32>>,
    apparent_temperature_max: Vec<Option<f32>>,
    apparent_temperature_min: Vec<Option<f32>>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Predict { city, date } => {
            let now_date = Utc::now().date_naive();
            let target_date =
                NaiveDate::parse_from_str(&date, "%Y-%m-%d").expect("Invalid date format");

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

            let historical_url = format!("https://archive-api.open-meteo.com/v1/archive?latitude={}&longitude={}&start_date={}&end_date={}&daily=temperature_2m_max,temperature_2m_min,apparent_temperature_max,apparent_temperature_min&timezone=auto", city_info.latitude, city_info.longitude, now_date - Duration::days(365 * 3), now_date);
            let historical_response = reqwest::get(historical_url)
                .await
                .unwrap()
                .json::<HistoricalResponse>()
                .await
                .unwrap();

            let data = historical_response
                .daily
                .time
                .into_iter()
                .zip(
                    historical_response
                        .daily
                        .temperature_2m_max
                        .into_iter()
                        .zip(historical_response.daily.temperature_2m_min.into_iter())
                        .zip(
                            historical_response
                                .daily
                                .apparent_temperature_max
                                .into_iter(),
                        )
                        .zip(
                            historical_response
                                .daily
                                .apparent_temperature_min
                                .into_iter(),
                        )
                )
                .filter_map(
                    |(date, (((max, min), app_max), app_min))| {
                        let diff = date.signed_duration_since(
                            target_date.with_year(date.year_ce().1 as i32).unwrap(),
                        );
                        if diff.num_days() < -6 || diff.num_days() > 6 {
                            return None;
                        }

                        Some([max, min, app_max, app_min])
                    },
                )
                .fold(vec![], |acc: Vec<Vec<f32>>, e| {
                    let mut acc = acc.clone();
                    e.into_iter().enumerate().for_each(|(i, v)| {
                        if let Some(v) = v {
                            match acc.get(i) {
                                Some(sub) => {
                                    let mut sub = sub.clone();
                                    sub.push(v);
                                    acc[i] = sub;
                                }
                                None => {
                                    acc.push(vec![v]);
                                }
                            }
                        }
                    });

                    return acc;
                })
                .iter()
                .map(|e| {
                    let mut e = e.clone();
                    e.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    e[(e.len() as f32 / 2.0).round() as usize]
                })
                .collect::<Vec<f32>>();

            println!("Location: {}, {}", city_info.name, city_info.country);
            println!("Date: {}", target_date);
            println!("Max: {}°C", data[0]);
            println!("Min: {}°C", data[1]);
            println!("Feels like max: {}°C", data[2]);
            println!("Feels like min: {}°C", data[3]);
        }
    }
}
