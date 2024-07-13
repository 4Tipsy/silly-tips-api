
use http::Status;
use rocket::*;
use ::serde::Deserialize;
use toml;
use once_cell::sync::{Lazy, OnceCell};
use mongodb::{Client, Database};
use mongodb::bson::doc;
use std::fs;


// modules
mod user_module;
mod tip_module;
mod guards;
mod utils;
use crate::user_module::user_controller::{handle_login, handle_register, handle_get_current_user, handle_update_profile_img};
use crate::tip_module::controllers::tip_controller::{handle_create_tip, handle_modify_tip, handle_rename_tip, handle_delete_tip, handle_get_tip_content};
use crate::tip_module::controllers::folder_controller::{handle_create_new_folder, handle_rename_folder, handle_get_folder_contents};






// config model
#[derive(Debug, Deserialize)]
struct Config {
    port: i32,
    mongodb_uri_string: String,
    path_to_img_repo: String,
    jwt_secret: String,
    psw_secret: String,
}


static CONFIG: Lazy<Config> = Lazy::new(
    || {
        #[derive(Deserialize)] struct _C { Config: Config } // config = config['Config']
        let raw_config_string = fs::read_to_string("Config.toml").expect("Error reading Config.toml");
        let _config: _C = toml::from_str(&raw_config_string).unwrap();
        _config.Config
    }
);



// database
static DB: OnceCell<Database> = OnceCell::new();









// some catchers
#[catch(404)]
async fn handle_400() -> Status {
    Status::NotFound
}
#[catch(500)]
async fn handle_500() -> Status {
    Status::InternalServerError
}






#[launch]
async fn rocket() -> _ {

    // set DB
    let db = Client::with_uri_str( &CONFIG.mongodb_uri_string ).await.expect("Error connecting to MongoDB");
    DB.set(db.database("silly-tips")).unwrap();

    // rocket
    rocket::build()
        .configure(rocket::Config::figment()
            .merge(("port", &CONFIG.port))
        )
        .mount("/api/", routes![
            handle_login, handle_register, handle_get_current_user, handle_update_profile_img,
            handle_create_tip, handle_modify_tip, handle_rename_tip, handle_delete_tip, handle_get_tip_content,
            handle_create_new_folder, handle_rename_folder, handle_get_folder_contents,
            ])
        .register("/", catchers![handle_400, handle_500])

}