

use rocket::{post, Request, Response};
use rocket::response::{Responder, Result as ResponseResult};
use rocket::serde::json::{json, Json, Value as JsonValue};
use rocket::http::{Status, ContentType};

use serde::Deserialize;

// modules
use crate::tip_module::services::folder_service;
use crate::guards::auth::Auth;







#[derive(Debug)]
pub struct ApiResponse {
  json: JsonValue,
  status: Status,
}
impl<'r> Responder<'r, 'r> for ApiResponse {
  fn respond_to(self, req: &Request) -> ResponseResult<'r> {
    Response::build_from(self.json.respond_to(&req).unwrap())
      .status(self.status)
      .header(ContentType::JSON)
      .ok()
  }
}





#[derive(Deserialize)]
pub struct CreateNewFolderBody {
  new_folder_name: String,
  parent_id: Option<String>,
}

#[post("/create-new-folder", data="<req>")]
pub async fn handle_create_new_folder(req: Json<CreateNewFolderBody>, auth: Auth) -> ApiResponse {
  match folder_service::create_new_folder(&req.new_folder_name, req.parent_id.clone(), &auth.user_id).await {

    Ok(_) => return ApiResponse {
      json: json!({"result": "ok"}),
      status: Status::Ok,
    },

    Err(err) => return ApiResponse {
      json: json!({"result": "err", "error": err}),
      status: Status::BadRequest,
    }

  };

}








#[derive(Deserialize)]
pub struct GetFolderContentsBody {
  parent_id: Option<String>,
}

#[post("/get-folder-contents", data="<req>")]
pub async fn handle_get_folder_contents(req: Json<GetFolderContentsBody>, auth: Auth) -> ApiResponse {

  match folder_service::get_folder_contents(req.parent_id.clone(), &auth.user_id).await {

    Ok(folder_contents) => return ApiResponse {
      json: json!({"result": "ok", "folder_contents": folder_contents}),
      status: Status::Ok,
    },

    Err(err) => return ApiResponse {
      json: json!({"result": "err", "error": err}),
      status: Status::BadRequest,
    }

  }
}