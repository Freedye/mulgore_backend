use actix_web::{web, App, HttpServer, Responder};
use actix_cors::Cors;
use mulgore_backend::api::blizzard_api::get_character_media_from_blizzard;
use mulgore_backend::api::raider_io_api::raider_io_character_call;
use mulgore_backend::api::raider_io_api::raider_io_talents_call;
use mulgore_backend::api::blizzard_api::get_blizzard_auth_code;
use mulgore_backend::api::blizzard_api::get_character_data_from_blizzard;

async fn index() -> impl Responder {
    "Endpoints: \n
    /                               : show this page \n
    /getRaiderIOData                : get character data from raider.io (hard coded for testing purposes for now.) \n
    /getBestTalentsBasedOnSpec      : get best talents based on spec from raider.io (hard coded for testing purposes for now.) \n
    /authenticateWithBlizzard       : authenticate with blizzard to get armory data\n
    /getCharacterDataFromBlizzard   : get Blizzard data after authentication (hard coded for testing purposes for now.) \n
    /getCharacterMediaFromBlizzard  : get Blizzard media after authentication (hard coded for testing purposes for now.) \n"
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
        .route("/getCharacterMediaFromBlizzard", web::get().to(get_character_media_from_blizzard))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}