use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GeocodingResponse {
    pub results: Option<Vec<GeocodingResult>>,
}

#[derive(Deserialize, Debug)]
pub struct GeocodingResult {
    pub name: String,
    pub country: String,
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Deserialize, Debug)]
pub struct HistoricalResponse {
    pub daily: HistoricalDaily,
}

#[derive(Deserialize, Debug)]
pub struct HistoricalDaily {
    pub time: Vec<NaiveDate>,
    pub temperature_2m_max: Vec<Option<f32>>,
    pub temperature_2m_min: Vec<Option<f32>>,
    pub apparent_temperature_max: Vec<Option<f32>>,
    pub apparent_temperature_min: Vec<Option<f32>>,
}
