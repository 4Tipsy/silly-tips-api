
use rocket::fs::TempFile;
use mongodb::bson::doc;


use std::fs;
use std::path::Path as FsPath;


// modules
use crate::{DB, CONFIG};
use crate::user_module::user_model::UserModel;
use crate::utils::simple_hashers::gen_num_hash;
use crate::utils::hash_password::{verify_password, get_hashed_psw};




pub async fn get_user_by_login(user_email: &str, password: &str) -> Result<UserModel, &'static str> {

  let db = DB.get().unwrap();
  let users_collection = db.collection::<UserModel>("users");

  // find user
  let user = users_collection.find_one(
    doc! {
      "email": user_email
    },
    None
  ).await.expect("Error while connecting to DB");

  // if no user found
  if user.is_none() {
    return Err("Invalid email or password");
  }

  // verify password
  let user_unwrapped = user.unwrap();
  if !verify_password(password, &user_unwrapped.hashed_password) {
    return Err("Invalid email or password");
  }

  // if ok
  Ok(user_unwrapped)
}





pub async fn create_new_user(user_email: &str, password: &str, user_name: &str) -> Result<(), &'static str> {

  let db = DB.get().unwrap();
  let users_collection = db.collection::<UserModel>("users");

  // check if user with given email or name already exists
  let existing_user = users_collection.find_one(
    doc! {
      "$or": [
        {"user_email": user_email},
        {"user_name": user_name}
      ]
    },
    None
  ).await.expect("Error while connecting to DB");

  if existing_user.is_some() {
    return Err("User with such email or name already exists");
  }


  // get next user id
  let next_free_id: String;
  loop {
    let _generated_id = gen_num_hash(5);
    let existing_id = users_collection.find_one(doc! {"user_id": &_generated_id}, None).await.expect("Error connecting to DB");
    if existing_id.is_none() {
      next_free_id = _generated_id;
      break;
    }
  }


  // new user struct
  let new_user = UserModel {
    user_id: next_free_id.clone(),
    user_name: user_name.to_string(),
    email: user_email.to_string(),
    hashed_password: get_hashed_psw(password),
    verified: false,
  };



  // insert new user into db
  let insert_result = users_collection.insert_one(new_user, None).await;

  // if ok
  if insert_result.is_ok() {

    // create new user's img repo
    let path_to_user_img_repo = FsPath::new("").join( &CONFIG.path_to_img_repo ).join( next_free_id );
    fs::create_dir(path_to_user_img_repo).expect("Failed to create user img repo directory");
    return Ok(());


  // if err
  } else {
    return Err("Failed to create user");
  }


}









pub async fn get_current_user(user_id: &str) -> Result<UserModel, &'static str> {

  let db = DB.get().unwrap();
  let users_collection = db.collection::<UserModel>("users");


  // get user from db
  let user: Option<UserModel> = users_collection.find_one(doc! {"user_id": &user_id}, None).await.expect("Error connecting to DB");

  // if user not found
  if user.is_none() {
    return Err("User not found");
  }

  let user_unwrapped = user.unwrap();

  // if user is not verified
  if user_unwrapped.verified == false {
    return Err("User is not verified");
  }

  // if ok
  Ok(user_unwrapped)
}





pub async fn update_profile_img(user_id: &str, mut file: TempFile<'_>)  -> Result<(), &'static str> {

  let save_to = FsPath::new("").join( &CONFIG.path_to_img_repo ).join(&user_id).join("__profile_img");

  
  match file.persist_to(save_to).await {

    Ok(_) => return Ok(()),

    Err(_) => return Err("Failed to update profile image")

  }
  
}