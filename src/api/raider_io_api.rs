use actix_web::{HttpResponse, Responder};
use reqwest::Client;
use crate::character::character_data_rio::CharacterDataFromRio;
use crate::talents::best_talents_by_spec::BestTalentsBasedOnSpec;

pub async fn raider_io_character_call() -> impl Responder {
    match get_raiderio_data().await {
        Ok(api_response) => HttpResponse::Ok().json(api_response),
        Err(_) => HttpResponse::InternalServerError().body("Error while calling Raider.IO for character data"),
    }
}

pub async fn raider_io_talents_call() -> impl Responder {
    match get_best_talents_based_on_spec().await {
        Ok(api_response) => HttpResponse::Ok().json(api_response),
        Err(_) => HttpResponse::InternalServerError().body("Error while calling Raider.IO for best talents"),
    }
}

async fn get_raiderio_data() -> Result<CharacterDataFromRio, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get("https://raider.io/api/v1/characters/profile?region=eu&realm=Silvermoon&name=Freedye&fields=gear%2Ctalents%2Cguild%2Craid_progression%2Cmythic_plus_scores_by_season%3Acurrent")
        .send()
        .await?
        .json::<CharacterDataFromRio>()
        .await?;
    Ok(response)
}

async fn get_best_talents_based_on_spec() -> Result<BestTalentsBasedOnSpec, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get("https://raider.io/api/mythic-plus/rankings/specs?region=world&season=season-tww-1&class=druid&spec=feral&page=0")
        .send()
        .await?
        .json::<BestTalentsBasedOnSpec>()
        .await?;
    Ok(response)
}