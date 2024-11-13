use actix_web::{web, HttpResponse, Responder};
use reqwest::Client;
use std::collections::HashMap;
use dotenv::dotenv;
use std::env;
use crate::character;
use crate::util::query_params::QueryParams;

pub async fn get_blizzard_auth_code() -> impl Responder {
    match start_blizzard_auth_flow().await {
        Ok(api_response) => HttpResponse::Ok().json(api_response),
        Err(_) => HttpResponse::InternalServerError().body("Error while obtaining Blizzard auth code"),
    }
}

pub async fn get_character_data_from_blizzard(query: web::Query<QueryParams>) -> impl Responder {
    match get_blizzard_character_data(&query.code).await {
        Ok(api_response) => HttpResponse::Ok().json(api_response),
        Err(_) => HttpResponse::InternalServerError().body("Error while obtaining Blizzard character data"),
    }
}

pub async fn get_character_media_from_blizzard(query: web::Query<QueryParams>) -> impl Responder {
    match get_blizzard_character_media(&query.code).await {
        Ok(api_response) => HttpResponse::Ok().json(api_response),
        Err(_) => HttpResponse::InternalServerError().body("Error while obtaining Blizzard character media"),
    }
}

async fn start_blizzard_auth_flow() -> Result<String, reqwest::Error> {
    dotenv().ok();
    let client_id = env::var("API_KEY").expect("API_KEY not set");
    let redirect_uri = "http://localhost:8080/";
    let scope = "wow.profile";

    let auth_url = format!(
        "https://eu.battle.net/oauth/authorize?response_type=code&client_id={}&redirect_uri={}&scope={}&state=''",
        client_id, redirect_uri, scope
    );

    Ok(auth_url)
}

async fn get_blizzard_character_data(code: &String) -> Result<character::character_data_blizzard::CharacterDataFromBlizzard, reqwest::Error> {

    let response_character_data = call_blizzard_api(String::from("https://eu.api.blizzard.com/profile/wow/character/silvermoon/freedye?namespace=profile-eu&locale=it_IT"), code).await?;

    println!("{:?}", response_character_data);
    
    let character = response_character_data.json::<character::character_data_blizzard::CharacterDataFromBlizzard>().await?;
    Ok(character)
}

async fn get_blizzard_character_media(code: &String) -> Result<character::character_media_blizzard::CharacterMediaBlizzard, reqwest::Error> {

    let response_character_media = call_blizzard_api(String::from("https://eu.api.blizzard.com/profile/wow/character/silvermoon/freedye/character-media?namespace=profile-eu&locale=it_IT"), code).await?;

    println!("{:?}", response_character_media);
    
    let character_media = response_character_media.json::<character::character_media_blizzard::CharacterMediaBlizzard>().await?;
    Ok(character_media)
}

async fn call_blizzard_api(url: String, auth_code: &String) -> Result<reqwest::Response, reqwest::Error> {
    let blizzard_token = get_blizzard_token(auth_code).await?;

    let client = Client::new();
    let response = client
    .get(url)
    .header("Authorization", format!("Bearer {}", blizzard_token))
    .send()
    .await?;

    println!("{:?}", response);

    Ok(response)

}

async fn get_blizzard_token(code: &String) -> Result<String, reqwest::Error> {
    dotenv().ok();
    let api_key = &code;
    let client_id = env::var("API_KEY").expect("API_KEY not set");
    let client_secret = env::var("API_SECRET").expect("API_SECRET not set");

    let client = Client::new();

    let mut params = HashMap::new();
    params.insert("grant_type", "authorization_code");
    params.insert("code", api_key);
    params.insert("redirect_uri", "http://localhost:8080/");
    params.insert("client_id", &client_id);
    params.insert("client_secret", &client_secret);

    let response: serde_json::Value= client
    .post("https://eu.battle.net/oauth/token")
    .form(&params)
    .send()
    .await?
    .json()
    .await?;

    println!("{:?}", serde_json::to_string(&response));

    let mut access_token_unwrapped = "";

    if let Some(access_token) = response.get("access_token") {
        access_token_unwrapped = access_token.as_str().unwrap();
    } else {
        println!("Cannot find access token");
    }

    Ok(String::from(access_token_unwrapped))
}