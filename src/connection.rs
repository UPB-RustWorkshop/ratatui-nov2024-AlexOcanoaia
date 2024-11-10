use core::time;
use std::{ffi::c_float, ptr::null, time::Duration};

use chrono::{format::Parsed, DateTime, Local, TimeZone, Utc};
use reqwest::{Request, StatusCode};
use serde_json::{self, Value};

pub struct CityInfo {
    // TODO: define elements in the structure
    pub temperature: f64,
    pub time: DateTime<Local>,
    pub humidity: f64,
    pub description: String,
    pub wind_speed: f64,
}

#[derive(Debug)]
pub enum MyError {
    SerdeErr(serde_json::Error),
    ReqwestErr(reqwest::Error),
}

/// Method that is handling the request to the OpenWeather api
/// and parsing the response
///
/// Returns weather details about a certain city
pub async fn get_data(city: String) -> Result<CityInfo, MyError>{
    let apy_key = "4d92d92f10f78581948b04916b86c42a";
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, apy_key);
    let tmp = reqwest::get(url).await;
    match tmp {
        Ok(response) => {
            let url_status = response.status();
        
            if url_status == StatusCode::OK {
                println!("Request was a success");

                let body = response.text().await.map_err(|err| MyError::ReqwestErr(err))?;
                let parsed: Value = serde_json::from_str(&body).map_err(|err| MyError::SerdeErr(err))?;
                let description = &parsed["weather"][0]["description"].to_string();
                let temp = &parsed["main"]["temp"].as_f64();
                let humidity = &parsed["main"]["humidity"].as_f64();
                let wind_speed = &parsed["wind"]["speed"].as_f64();
                let timezone = &parsed["timezone"].as_i64();
                let local_time = Local::now();
                let time = local_time + chrono::Duration::hours(timezone.map_or(0, |timezone| timezone) / 3600);
                let city_info = CityInfo {
                    temperature: temp.map_or(0.0,|temp| temp),
                    humidity: humidity.map_or(0.0, |humidity| humidity),
                    description: String::from(description),
                    wind_speed: wind_speed.map_or(0.0, |wind_speed| wind_speed),
                    time: time,
                };
                return Ok(city_info)
            } else {
                panic!("Request finish with an error that has the status: {}", url_status);
            }
        },
        Err(error) => {
            panic!("Error is: {}", error);
        }
    }
}