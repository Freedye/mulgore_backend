use actix_web::{web, HttpResponse, Responder};
use reqwest::Client;
use std::collections::HashMap;
use dotenv::dotenv;
use std::env;
use crate::character::character_data_from_blizzard::CharacterDataFromBlizzard;
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

async fn get_blizzard_character_data(code: &String) -> Result<CharacterDataFromBlizzard, reqwest::Error> {
    dotenv().ok();
    let api_key = &code;
    let client_id = env::var("API_KEY").expect("API_KEY not set");
    let client_secret = env::var("API_SECRET").expect("API_SECRET not set");

    let client = Client::new();

    let mut params = HashMap::new();
    params.insert("grant_type", "authorization_code");
    params.insert("code", api_key);
    params.insert("redirect_uri", "http://localhost:8080/getCharacterDataFromBlizzard");
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

    let response_character_data = client
    .get("https://eu.api.blizzard.com/profile/wow/character/silvermoon/freedye?namespace=profile-eu&locale=it_IT")
    .header("Authorization", format!("Bearer {}", access_token_unwrapped))
    .send()
    .await?;

    println!("{:?}", response_character_data);
    
    let character = response_character_data.json::<CharacterDataFromBlizzard>().await?;
    Ok(character)
}

async fn start_blizzard_auth_flow() -> Result<String, reqwest::Error> {
    dotenv().ok();
    let client_id = env::var("API_KEY").expect("API_KEY not set");
    let redirect_uri = "http://localhost:8080/getCharacterDataFromBlizzard";
    let scope = "wow.profile";

    let auth_url = format!(
        "https://eu.battle.net/oauth/authorize?response_type=code&client_id={}&redirect_uri={}&scope={}&state=''",
        client_id, redirect_uri, scope
    );

    Ok(auth_url)
}