
use rocket::request::{Request, FromRequest, Outcome};
use rocket::async_trait;
use rocket::http::{Status, Cookie};
use jsonwebtoken::{self, DecodingKey, Validation};

use serde::{Serialize, Deserialize};
use mongodb::bson::doc;


// modules
use crate::CONFIG;
use crate::DB;
use crate::user_module::user_model::UserModel;





#[derive(Serialize, Deserialize, Debug)]
pub struct ATokenModel {
  pub user_id: String,
}







pub struct Auth {
  pub user_id: String,
}



#[async_trait]
impl<'r> FromRequest<'r> for Auth {
  type Error = ();



  async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ()> {

    let db = DB.get().unwrap();
    let users_collection = db.collection::<UserModel>("users");
    


    let a_cookie: Option<&Cookie<'_>> = req.cookies().get("a-token");

    // if no such cookie
    if a_cookie.is_none() {
      return Outcome::Forward(Status::Unauthorized);
    }


    let token: &str = a_cookie.unwrap().value();

    // if cookie
    match jsonwebtoken::decode::<ATokenModel>(token, &DecodingKey::from_secret(CONFIG.jwt_secret.as_bytes()), &Validation::default()) {

      // if valid token
      Ok(parsed_token) => {

        let user: Option<UserModel> = users_collection.find_one(doc! {"user_id": &parsed_token.claims.user_id}, None).await.expect("Error connecting to DB");
        if user.is_none() {
          // if no such user
          Outcome::Forward(Status::Unauthorized)
        } else {
          // if ok
          Outcome::Success(Auth {user_id: parsed_token.claims.user_id})
        }

      
      },


      // if invalid token
      Err(_) => {
        Outcome::Forward(Status::Unauthorized)
      }

    }


  }



}