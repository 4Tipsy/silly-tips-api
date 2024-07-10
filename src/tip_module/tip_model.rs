
use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum EntityTypeEnum {
  Tip,
  Folder,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct TipLikeEntityModel {

  pub entity_id: String,
  pub owner_id: String, // user's id
  pub parent_id: Option<String>,

  pub name: String,
  pub entity_type: EntityTypeEnum,


  pub content: Option<String>, // only if "Tip"
  pub last_created: Option<String>, // only if "Tip"
  pub last_modified: Option<String>, // only if "Tip"
}
