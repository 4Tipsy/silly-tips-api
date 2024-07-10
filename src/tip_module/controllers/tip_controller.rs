

use rocket::{get, post, Request, Response};
use rocket::response::{Responder, Result as ResponseResult};
use rocket::serde::json::{json, Json, Value as JsonValue};
use rocket::http::{Status, ContentType};

use serde::Deserialize;

// modules
use crate::tip_module::services::tip_service;
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
pub struct CreateTipBody {
  new_tip_name: String,
  parent_id: Option<String>,
}

#[post("/create-tip", data="<req>")]
pub async fn handle_create_tip(req: Json<CreateTipBody>, auth: Auth) -> ApiResponse {
  match tip_service::create_tip(&req.new_tip_name, req.parent_id.clone(), &auth.user_id).await {

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
pub struct ModifyTipBody {
  new_content: String,
  entity_id: String,
}

#[post("/modify-tip", data="<req>")]
pub async fn handle_modify_tip(req: Json<ModifyTipBody>, auth: Auth) -> ApiResponse {
  match tip_service::modify_tip(&req.new_content, &req.entity_id, &auth.user_id).await {

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
pub struct DeleteTipBody {
  entity_id: String,
}

#[post("/delete-tip", data="<req>")]
pub async fn handle_delete_tip(req: Json<DeleteTipBody>, auth: Auth) -> ApiResponse {
  match tip_service::delete_tip(&req.entity_id, &auth.user_id).await {

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





#[get("/get-tip-content/<entity_id>")]
pub async fn handle_get_tip_content(entity_id: &str, auth: Auth) -> ApiResponse {

  // if entity_id is not provided
  if entity_id.len() == 0 {
    return ApiResponse {
      json: json!({"result": "err", "error": "entity_id cannot be empty"}),
      status: Status::BadRequest,
    }
  }


  match tip_service::get_tip_content(&entity_id, &auth.user_id).await {

    Ok(content) => return ApiResponse {
      json: json!({"result": "ok", "content": content}),
      status: Status::Ok,
    },

    Err(err) => return ApiResponse {
      json: json!({"result": "err", "error": err}),
      status: Status::BadRequest,
    }

  };

}