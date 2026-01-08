use std::fmt::format;

use crate::{adapter::get_emoji::get_temp_emoji, types::weather_types::WeatherResponse};

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

pub async fn display_weather_info(response: &WeatherResponse) {
    let weather_text: String = format!(
        "
        Weather is {} : 
        > Temperature: {:.1} C
        > Humidity: {:.1}%
        > Wind Speed : {:.1}m/s
        > Wind_degrees : {:.1} degree
        > Sunset : {:.1}
        > Cloud PCT : {:.1}
        > Feels Like : {:.1}C
        > Sunrise : {:.1}
    ",
        get_temp_emoji(response.temp),
        response.temp,
        response.humidity,
        response.wind_speed,
        response.wind_degrees,
        response.sunset,
        response.cloud_pct,
        response.feels_like,
        response.sunrise
    );
}