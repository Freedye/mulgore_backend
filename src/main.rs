use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use reqwest::Client;

mod character;

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

async fn index() -> impl Responder {
    match get_raiderio_data().await {
        Ok(api_response) => HttpResponse::Ok().json(api_response), // Rispondi con i dati ottenuti
        Err(_) => HttpResponse::InternalServerError().body("Errore nel chiamare l'API esterna"),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}