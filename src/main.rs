// Modules
mod api; 
mod models;
mod repository;

#[macro_use] extern crate rocket;
use api::user_api::create_user;
use repository::mongodb::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build().manage(db).mount("/", routes![create_user])
}