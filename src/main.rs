use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use reqwest::Client;

mod character;
mod talents;

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

async fn index() -> impl Responder {
    "Endpoints: \n
    /                           : show this page \n
    /getRaiderIOData            : get character data from raider.io (hard coded for testing purposes for now.) \n
    /getBestTalentsBasedOnSpec  : get best talents based on spec from raider.io (hard coded for testing purposes for now.)"
}

async fn raider_io_character_call() -> impl Responder {
    match get_raiderio_data().await {
        Ok(api_response) => HttpResponse::Ok().json(api_response),
        Err(_) => HttpResponse::InternalServerError().body("Error while calling Raider.IO"),
    }
}

async fn raider_io_talents_call() -> impl Responder {
    match get_best_talents_based_on_spec().await {
        Ok(api_response) => HttpResponse::Ok().json(api_response),
        Err(_) => HttpResponse::InternalServerError().body("Error while calling Raider.IO"),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(index))
        .route("/getRaiderIOData", web::get().to(raider_io_character_call))
        .route("/getBestTalentsBasedOnSpec", web::get().to(raider_io_talents_call))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}