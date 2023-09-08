use core::fmt;

use chrono::NaiveDate;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

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

#[derive(Deserialize, Debug)]
pub struct ForecastResponse {
    pub timezone: String,
    pub hourly: ForecastHourly,
    pub daily: ForecastDaily,
}

#[derive(Deserialize, Debug)]
pub struct ForecastHourly {
    pub time: Vec<String>,
    pub precipitation_probability: Vec<f32>,
    pub precipitation: Vec<f32>,
    pub temperature_2m: Vec<f32>,
    pub relativehumidity_2m: Vec<f32>,
    pub apparent_temperature: Vec<f32>,
}

#[derive(Deserialize, Debug)]
pub struct ForecastDaily {
    pub time: Vec<NaiveDate>,
    pub weathercode: Vec<WeatherCode>,
    pub temperature_2m_max: Vec<f32>,
    pub temperature_2m_min: Vec<f32>,
}

#[derive(Deserialize_repr, Debug)]
#[repr(u8)]
pub enum WeatherCode {
    ClearSky = 0,
    MainlyClear = 1,
    PartlyCloudy = 2,
    Overcast = 3,
    Fog = 45,
    DepositingRimeFog = 48,
    LightDrizzle = 51,
    Drizzle = 53,
    HeavyDrizzle = 55,
    FreezingLightDrizzle = 56,
    FreezingDenseDrizzle = 57,
    SlightRain = 61,
    Rain = 63,
    HeavyRain = 65,
    FreezingLightRain = 66,
    FreezingHeavyRain = 67,
    SlightSnowFall = 71,
    SnowFall = 73,
    HeavySnowFall = 75,
    SnowGrains = 77,
    SlightRainShowers = 80,
    RainShowers = 81,
    HeavyRainShowers = 82,
    SlightSnowShowers = 85,
    HeavySnowShowers = 86,
    Thunderstorm = 95,
    ThunderstormWithSlightHail = 96,
    ThunderstormWithHeavyHail = 99,
}

impl fmt::Display for WeatherCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            WeatherCode::ClearSky => "Clear Sky",
            WeatherCode::MainlyClear => "Mainly Clear",
            WeatherCode::PartlyCloudy => "Partly Cloudy",
            WeatherCode::Overcast => "Overcast",
            WeatherCode::Fog => "Fog",
            WeatherCode::DepositingRimeFog => "Depositing Rime Fog",
            WeatherCode::LightDrizzle => "Light Drizzle",
            WeatherCode::Drizzle => "Drizzle",
            WeatherCode::HeavyDrizzle => "Heavy Drizzle",
            WeatherCode::FreezingLightDrizzle => "Freezing Light Drizzle",
            WeatherCode::FreezingDenseDrizzle => "Freezing Dense Drizzle",
            WeatherCode::SlightRain => "Slight Rain",
            WeatherCode::Rain => "Rain",
            WeatherCode::HeavyRain => "Heavy Rain",
            WeatherCode::FreezingLightRain => "Freezing Light Rain",
            WeatherCode::FreezingHeavyRain => "Freezing Heavy Rain",
            WeatherCode::SlightSnowFall => "Slight Snow Fall",
            WeatherCode::SnowFall => "Snow Fall",
            WeatherCode::HeavySnowFall => "Heavy Snow Fall",
            WeatherCode::SnowGrains => "Snow Grains",
            WeatherCode::SlightRainShowers => "Slight Rain Showers",
            WeatherCode::RainShowers => "Rain Showers",
            WeatherCode::HeavyRainShowers => "Heavy Rain Showers",
            WeatherCode::SlightSnowShowers => "Slight Snow Showers",
            WeatherCode::HeavySnowShowers => "Heavy Snow Showers",
            WeatherCode::Thunderstorm => "Thunderstorm",
            WeatherCode::ThunderstormWithSlightHail => "Thunderstorm with Slight Hail",
            WeatherCode::ThunderstormWithHeavyHail => "Thunderstorm with Hail",
        };
        write!(f, "{}", name)
    }
}
