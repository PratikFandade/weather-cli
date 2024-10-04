use std::io;
use serde::Deserialize;
use colored::*;

use std::env;
use dotenv::dotenv;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f32,
    humidity: f32,
    pressure: f32,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f32,
}

fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city,
        country_code,
        api_key
    );

    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

fn display_weather_info(weather: &WeatherResponse) {
    let description = &weather.weather[0].description;
    let temperature = weather.main.temp;
    let humidity = weather.main.humidity;
    let pressure = weather.main.pressure;
    let wind_speed = weather.wind.speed;

    let weather_text = format!(
        "Weather in {}: {} {}
        > Temperature: {:.1}°C,
        > Humidity: {:.1}%,
        > Pressure: {:.1} hPa,
        > Wind Speed: {:.1} m/s",
        weather.name,
        description,
        get_temp_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed
    );

    let weather_text_coloured = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

    println!("{}", weather_text_coloured);
}

fn get_temp_emoji(temperature: f32) -> &'static str{
    if temperature < 0.0 {
        "🥶"
    } else if temperature >= 0.0 && temperature < 15.0 {
        "☁️"
    } else if temperature >= 15.0 && temperature < 25.0 {
        "⛅️"
    } else if temperature >= 25.0 && temperature < 35.0 {
        "🌤️"
    } else {
        "🔥"
    }
}

fn main() {
    dotenv().ok();

    println!("{}", "Welcome to Weather Station!".bright_yellow());
    loop {
        println!("{}", "Please enter city name: ".bright_green());
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read line");
        let city: &str = city.trim();

        println!("{}", "Please enter country code: ".bright_green());
        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read line");
        let country_code: &str = country_code.trim();

        let api_key: &str = &env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY must be set");

        match get_weather_info(&city, &country_code, api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }

        println!("{}", "Do you want to search for weather in another city? (y/n):".bright_green()); // Prompting user to continue or exit
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input"); // Reading user input for continuation
        let input = input.trim().to_lowercase();

        if input != "y" {
            println!("Thank you for using our software!");
            break; // Exiting the loop if user doesn't want to continue
        }
    }
}

