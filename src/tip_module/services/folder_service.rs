

use futures::TryStreamExt;
use mongodb::bson::doc;

// modules
use crate::tip_module::tip_model::{TipLikeEntityModel, EntityTypeEnum};
use crate::utils::simple_hashers::gen_simple_hash;
use crate::DB;







pub async fn create_new_folder(new_folder_name: &str, parent_id: Option<String>, user_id: &str) -> Result<(), &'static str> {

  let db = DB.get().unwrap();
  let tips_collection = db.collection::<TipLikeEntityModel>("tips");


    // check if folder with such name exists
    let existing_one = tips_collection.find_one(
      doc! {
        "name": &new_folder_name,
        "parent_id": &parent_id,
        "owner_id": &user_id,
      },
      None
    ).await.expect("Error while connecting to DB");
  
    if existing_one.is_some() {
      return Err("Folder with such name already exists");
    }



    // create new folder
    // get next free id
    let next_free_id: String;
    loop {
      let _generated_id = gen_simple_hash(11);
      let existing_id = tips_collection.find_one(doc! {"entity_id": &_generated_id}, None).await.expect("Error connecting to DB");
      if existing_id.is_none() {
        next_free_id = _generated_id;
        break;
      }
    }

    // new tip entity
    let new_folder = TipLikeEntityModel {
      entity_id: next_free_id,
      owner_id: user_id.to_string(),
      parent_id: parent_id, // Option<String>
      name: new_folder_name.to_string(),
      entity_type: EntityTypeEnum::Folder,
      content: None,
      last_created: None,
      last_modified: None,
    };

    // insert
    match tips_collection.insert_one(new_folder, None).await {

      Ok(_) => return Ok(()),

      Err(_) => return Err("Failed to create new folder")
    };


}






pub async fn rename_folder(new_name: &str, entity_id: &str, user_id: &str) -> Result<(), &'static str> {

  let db = DB.get().unwrap();
  let tips_collection = db.collection::<TipLikeEntityModel>("tips");


  // check if folder with such name exists
  let existing_folder = tips_collection.find_one_and_delete(
    doc! {
      "entity_id": &entity_id,
      "owner_id": &user_id,
    },
    None
  ).await.expect("Error while connecting to DB");

  if existing_folder.is_none() {
    return Err("No such folder");
  }


  // update
  let mut folder_entity = existing_folder.unwrap();
  folder_entity.name = new_name.to_string();
  match tips_collection.insert_one(folder_entity, None).await {

    Ok(_) => return Ok(()),

    Err(_) => return Err("Failed to rename folder")
  };

}









pub async fn delete_folder(entity_id: &str, user_id: &str) {

  // TODO: kmp...
}









pub async fn get_folder_contents(entity_id: Option<String>, user_id: &str) -> Result<Vec<TipLikeEntityModel>, &'static str> {
  let db = DB.get().unwrap();
  let tips_collection = db.collection::<TipLikeEntityModel>("tips");


  // check if folder exists
  if entity_id.clone().is_some() {
    let existing_one = tips_collection.find_one(doc! {"owner_id": &user_id, "entity_id": &entity_id}, None).await.expect("Failed connecting to DB");
    if existing_one.is_none() {
      return Err("No such folder");
    }

    // if it's not a folder??
    if existing_one.unwrap().entity_type != EntityTypeEnum::Folder {
      return Err("This is not a folder");
    }
  }



  // find folder contents
  let folder_contents = tips_collection.find(
    doc! {
      "parent_id": entity_id,
      "owner_id": user_id,
    },
    None
  ).await.expect("Error while connecting to DB");

  let result: Vec<TipLikeEntityModel> = folder_contents.try_collect().await.unwrap();
  return Ok(result);
}