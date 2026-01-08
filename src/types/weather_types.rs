#![allow(unused)]
use std::io;
use serde::Deserialize;
use colored::*;


//struct to desierialize the json response from api 

#[derive(Debug, Deserialize, Clone)]
pub struct WeatherResponse {
   pub wind_speed : f64,
   pub wind_degrees : f64,
   pub temp : f64,
   pub humidity : f64,
   pub sunset: f64,
   pub min_temp : f64,
   pub cloud_pct : f64,
   pub feels_like : f64,
   pub sunrise : f64,
   pub max_temp : f64,
}

// ///{10 items
// "wind_speed":5.66
// "wind_degrees":210
// "temp":7
// "humidity":87
// "sunset":1615658463
// "min_temp":7
// "cloud_pct":75
// "feels_like":2
// "sunrise":1615616341
// "max_temp":8
// }