#![allow(unused)]
pub mod types;
pub mod adapter;

use std::io;
use serde::Deserialize;
use colored::*;

use crate::adapter::{ api_request::get_weather_info, display_info::display_weather_info };

#[tokio::main]
async fn main() {
    println!("{}", "Welcome toWeather station!".bright_yellow());
    loop {
        println!("{}", "Enter the name of city: ".bright_cyan());
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input");
        let city = city.trim();
        let api_key = "your_api_key";
        match get_weather_info(city, api_key).await {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(e) => {
                eprintln!("Error {}", e);
            }
        }
        println!("Want to Search for another city: (yes/no)");
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).expect("Error reading answer");
        let ans = ans.trim();
        if ans == "no" || ans == "No" || ans == "NO" {
            break;
        }
    }
}
