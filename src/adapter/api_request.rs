// api_key = 6TDmDzHnHfrSbOlqNbeThw==1ClpzBmtubSzFeCx
// url = https://api.api-ninjas.com/v1/weather?city={}

use crate::types::weather_types::WeatherResponse;
use reqwest::{self, Client, Response};


pub async  fn get_weather_info(city: &str, api_key : &str) -> Result<WeatherResponse, reqwest::Error>{
    let url = format!("https://api.api-ninjas.com/v1/weather?city={}", city);
    let client = Client::new();
    let response = client
    .get(&url)
    .header("X-Api-Key", api_key)
    .send()
    .await?
    .json::<WeatherResponse>()
    .await?;
    Ok(response)

} 


//blocking refers to sync (one request at a time)