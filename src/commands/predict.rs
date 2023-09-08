use chrono::{Datelike, Duration, NaiveDate, Utc};

use crate::api::{GeocodingResponse, HistoricalResponse};

pub async fn predict(city: String, date: String) {
    let now_date = Utc::now().date_naive();
    let target_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").expect("Invalid date format");

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
                ),
        )
        .filter_map(|(date, (((max, min), app_max), app_min))| {
            let diff =
                date.signed_duration_since(target_date.with_year(date.year_ce().1 as i32).unwrap());
            if diff.num_days() < -6 || diff.num_days() > 6 {
                return None;
            }

            Some([max, min, app_max, app_min])
        })
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
    println!("Low: {}°C", data[1]);
    println!("High: {}°C", data[0]);
    println!("Feels Like (Low): {}°C", data[3]);
    println!("Feels Like (High): {}°C", data[2]);
}
