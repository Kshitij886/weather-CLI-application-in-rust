

pub fn get_temp_emoji (temperature: f64) -> &'static str {
    if temperature <= 0.0 {
        "❄"
    } else if temperature > 0.0 && temperature <= 10.0{
        "☁"
    }else if temperature > 10.0 && temperature <= 20.0 {
        "🌥"
    } else if temperature > 20.0 && temperature <= 30.0{
        "⛅"
    }else {
        "🌤"
    }
}