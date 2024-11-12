extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;

use actix_web::{web, App, HttpServer, Responder};

mod character;

async fn getRaiderIOData() -> impl Responder {
    let response = reqwest::get("https://raider.io/api/v1/characters/profile?region=eu&realm=Silvermoon&name=Freedye&fields=gear%2Ctalents%2Cguild%2Craid_progression%2Cmythic_plus_scores_by_season%3Acurrent");
    return "Ok";
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(getRaiderIOData))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}