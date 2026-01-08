use crate::types::weather_types::WeatherResponse;
use reqwest::{self, Client, Response};


pub async  fn get_weather_info(city: &str, api_key : &str) -> Result<WeatherResponse, reqwest::Error>{
    let url = format!("https://api.api-ninjas.com/v1/weather?city={}", city);
    let client = Client::new();
    let response = client
    .get(&url)
    .header("X-Api-Key", api_key)
    .send()
    .await?;

    println!("response {:?}", response);
    let json_resp = Response::json::<WeatherResponse>(response).await?;
    Ok(json_resp)

} 


//blocking refers to sync (one request at a time)