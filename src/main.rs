use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use reqwest::Client;
use std::collections::HashMap;
use dotenv::dotenv;
use std::env;

mod character;
mod talents;
mod util;

async fn get_raiderio_data() -> Result<character::character_data_rio::CharacterDataFromRio, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get("https://raider.io/api/v1/characters/profile?region=eu&realm=Silvermoon&name=Freedye&fields=gear%2Ctalents%2Cguild%2Craid_progression%2Cmythic_plus_scores_by_season%3Acurrent")
        .send()
        .await?
        .json::<character::character_data_rio::CharacterDataFromRio>()
        .await?;
    Ok(response)
}

async fn get_best_talents_based_on_spec() -> Result<talents::best_talents_by_spec::BestTalentsBasedOnSpec, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get("https://raider.io/api/mythic-plus/rankings/specs?region=world&season=season-tww-1&class=druid&spec=feral&page=0")
        .send()
        .await?
        .json::<talents::best_talents_by_spec::BestTalentsBasedOnSpec>()
        .await?;
    Ok(response)
}

async fn get_blizzard_character_data(code: &String) -> Result<character::character_data_from_blizzard::CharacterDataFromBlizzard, reqwest::Error> {
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
    
    let character = response_character_data.json::<character::character_data_from_blizzard::CharacterDataFromBlizzard>().await?;
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

async fn index() -> impl Responder {
    "Endpoints: \n
    /                               : show this page \n
    /getRaiderIOData                : get character data from raider.io (hard coded for testing purposes for now.) \n
    /getBestTalentsBasedOnSpec      : get best talents based on spec from raider.io (hard coded for testing purposes for now.) \n
    /authenticateWithBlizzard       : authenticate with blizzard to get armory data\n
    /getCharacterDataFromBlizzard   : get Blizzard data after authentication (hard coded for testing purposes for now.) \n"
}

async fn raider_io_character_call() -> impl Responder {
    match get_raiderio_data().await {
        Ok(api_response) => HttpResponse::Ok().json(api_response),
        Err(_) => HttpResponse::InternalServerError().body("Error while calling Raider.IO for character data"),
    }
}

async fn raider_io_talents_call() -> impl Responder {
    match get_best_talents_based_on_spec().await {
        Ok(api_response) => HttpResponse::Ok().json(api_response),
        Err(_) => HttpResponse::InternalServerError().body("Error while calling Raider.IO for best talents"),
    }
}

async fn get_blizzard_auth_code() -> impl Responder {
    match start_blizzard_auth_flow().await {
        Ok(api_response) => HttpResponse::Ok().json(api_response),
        Err(_) => HttpResponse::InternalServerError().body("Error while obtaining Blizzard auth code"),
    }
}

async fn get_character_data_from_blizzard(query: web::Query<util::query_params::QueryParams>) -> impl Responder {
    match get_blizzard_character_data(&query.code).await {
        Ok(api_response) => HttpResponse::Ok().json(api_response),
        Err(_) => HttpResponse::InternalServerError().body("Error while obtaining Blizzard character data"),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .wrap(Cors::default().allow_any_origin())
        .route("/", web::get().to(index))
        .route("/getRaiderIOData", web::get().to(raider_io_character_call))
        .route("/getBestTalentsBasedOnSpec", web::get().to(raider_io_talents_call))
        .route("/authenticateWithBlizzard", web::get().to(get_blizzard_auth_code))
        .route("/getCharacterDataFromBlizzard", web::get().to(get_character_data_from_blizzard))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}