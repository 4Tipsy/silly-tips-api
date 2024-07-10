

use mongodb::bson::doc;

use chrono::Utc;

// modules
use crate::tip_module::tip_model::{TipLikeEntityModel, EntityTypeEnum};
use crate::utils::gen_simple_hash;
use crate::DB;






pub async fn create_tip(new_tip_name: &str, parent_id: Option<String>, user_id: &str) -> Result<(), &'static str> {

  let db = DB.get().unwrap();
  let tips_collection = db.collection::<TipLikeEntityModel>("tips");


    // check if tip with such name exists
    let existing_tip = tips_collection.find_one(
      doc! {
        "tip_name": &new_tip_name,
        "parent_id": &parent_id,
        "owner_id": &user_id,
      },
      None
    ).await.expect("Error while connecting to DB");
  
    if existing_tip.is_some() {
      return Err("Tip with such name already exists");
    }



    // create new tip
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

    let initial_tip_content = concat!(
      "<!---",
      "New tip!",
      "-->",
    );

    // new tip entity
    let new_tip = TipLikeEntityModel {
      entity_id: next_free_id,
      owner_id: user_id.to_string(),
      parent_id: parent_id, // Option<String>
      name: new_tip_name.to_string(),
      entity_type: EntityTypeEnum::Tip,
      content: Some( initial_tip_content.to_string() ),
      last_created: Some( Utc::now().to_rfc3339() ),
      last_modified: Some( Utc::now().to_rfc3339() ),
    };

    // insert
    match tips_collection.insert_one(new_tip, None).await {

      Ok(_) => return Ok(()),

      Err(_) => return Err("Failed to create new tip")
    };


}












pub async fn modify_tip(new_content: &str, entity_id: &str, user_id: &str) -> Result<(), &'static str> {

  let db = DB.get().unwrap();
  let tips_collection = db.collection::<TipLikeEntityModel>("tips");

  // options
  let filter = doc! {"entity_id": entity_id, "owner_id": user_id};
  let update = doc! {
    "$set": 
    {
      "content": new_content,
      "last_modified": Utc::now().to_rfc3339()
    }
  };

  // update
  match tips_collection.update_one(filter, update, None).await {

    Ok(_) => return Ok(()),
    Err(_) => return Err("Failed to modify tip")
  };

}







pub async fn delete_tip(entity_id: &str, user_id: &str) -> Result<(), &'static str> {

  let db = DB.get().unwrap();
  let tips_collection = db.collection::<TipLikeEntityModel>("tips");


  // check if exists
  let existing_tip = tips_collection.find_one(doc! {"entity_id": &entity_id, "owner_id": &user_id}, None).await.expect("Failed to connect to DB");
  if existing_tip.is_none() {
    return Err("No such tip");
  }

  // delete
  match tips_collection.delete_one(doc! {"entity_id": entity_id, "owner_id": user_id}, None).await {

    Ok(_) => return Ok(()),
    Err(_) => return Err("Failed to delete tip")
  };
}









pub async fn get_tip_content(entity_id: &str, user_id: &str) -> Result<String, &'static str> {

  let db = DB.get().unwrap();
  let tips_collection = db.collection::<TipLikeEntityModel>("tips");


  match tips_collection.find_one(doc! {"entity_id": entity_id, "owner_id": user_id}, None).await.expect("Failed to connect to DB") {

    Some(tip) => {

      // if it's Folder
      if tip.entity_type == EntityTypeEnum::Folder {
        return Err("This id is related to Folder");
      }

      // if ok
      let content_unwrapped = tip.content.unwrap();
      return Ok(content_unwrapped);
    }

    None => return Err("There is no such tip")

  }

}