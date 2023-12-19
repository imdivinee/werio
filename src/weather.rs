// essentially a port of https://github.com/divinewrites/weather-app/blob/master/src/main.rs

pub async fn get_weather(city: &str) -> String {
    // api key | ⚠️ set this in your environment variables
    let api_key =
        std::env::var("OPENWEATHER_API_KEY").expect("Expected a token in the environment");

    // define the base api url
    let url =
        format!("https://api.openweathermap.org/data/2.5/weather?q={city}&appid={api_key}&units=metric");

    // get response
    let response = reqwest::Client::new()
        .get(url)
        .header(
            reqwest::header::USER_AGENT,
            "Mozilla/5.0 (Linux x86_64; rv:115.0) Gecko/20100101 Firefox/115.0",
        )
        .send()
        .await
        .unwrap();

    // check response status
    if response.status().is_success() {
        let body = response.text().await.unwrap();

        // making sure we're working with valid json
        match serde_json::from_str::<serde_json::Value>(&body) {
            Ok(data) => {
                let location = data["name"].as_str().unwrap_or("Unknown");
                let temperature = data["main"]["temp"].as_f64().unwrap_or(0.0);
                let weather_type = data["weather"][0]["description"]
                    .as_str()
                    .unwrap_or("Unknown");

                let response = format!(
                    "Weather in {}:\nTemperature: {} °C\nWeather Type: {}",
                    location, temperature, weather_type
                );

                return response;
            }
            // json wasn't valid return error
            Err(err) => {
                let response = format!("Failed to parse JSON response: {}", err);
                return response;
            }
        }
    } else {
        // city doesn't exist / invalid | api key was invalid / blocked
        return "City could not be found - [api key could also be invalid / blocked]".to_string();
    }
}
