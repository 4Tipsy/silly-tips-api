
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct UserModel {
  pub user_id: String,

  pub user_name: String,
  pub email: String,
  pub hashed_password: String,

  pub verified: bool,
}