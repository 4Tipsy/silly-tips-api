

use rocket::{get, post, Request, Response};
use rocket::response::{Responder, Result as ResponseResult};
use rocket::serde::json::{json, Json, Value as JsonValue};
use rocket::http::{Status, ContentType};
use rocket::http::{Cookie, CookieJar};
use rocket::fs::TempFile;

use serde::Deserialize;
use jsonwebtoken::{self, EncodingKey, Header};
use time::{self, Duration};

// modules
use crate::CONFIG;
use crate::user_module::user_service;
use crate::guards::auth::{Auth, ATokenModel};








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
pub struct LoginReqBody {
  user_email: String,
  password: String,
}

#[post("/login", data="<req>")]
pub async fn handle_login(req: Json<LoginReqBody>, jar: &CookieJar<'_>) -> ApiResponse {

  match user_service::get_user_by_login(&req.user_email, &req.password).await {
      
    Ok(user) => {

      // set a_token cookie
      let claims = ATokenModel {
        user_id: user.user_id
      };
      let token_string = jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(CONFIG.jwt_secret.as_bytes()))
        .expect("Failed to encode jwt");

      let token_cookie = Cookie::build(("a_token", token_string))
        .path("/")
        .max_age( Duration::days(30*3) );

      jar.add(token_cookie);

      // return ok response
      return ApiResponse {
        json: json!({"result": "ok"}),
        status: Status::Ok,
      };
    },



    Err(err) => return ApiResponse {
      json: json!({"result": "err", "detail": err}),
      status: Status::BadRequest,
    },

  }


}










#[derive(Deserialize)]
pub struct RegisterReqBody {
  user_email: String,
  password: String,
  user_name: String,
}

#[post("/register", data="<req>")]
pub async fn handle_register(req: Json<RegisterReqBody>) -> ApiResponse {
  match user_service::create_new_user(&req.user_email, &req.password, &req.user_name).await {

    Ok(_) => return ApiResponse {
      json: json!({"result": "ok"}),
      status: Status::Ok,
    },

    Err(err) => return ApiResponse {
      json: json!({"result": "err", "detail": err}),
      status: Status::BadRequest,
    }

  };

}







#[get("/get-current-user")]
pub async fn handle_get_current_user(auth: Auth) -> ApiResponse {
  match user_service::get_current_user(&auth.user_id).await {

    Ok(user) => return ApiResponse {
      json: json!({"result": "ok", "user": user}),
      status: Status::Ok,
    },

    Err(err) => return ApiResponse {
      json: json!({"result": "err", "detail": err}),
      status: Status::BadRequest,
    }

  };

}





#[post("/update-profile-img", data="<file>")]
pub async fn handle_update_profile_img(auth: Auth, file: TempFile<'_>) -> ApiResponse {

  match user_service::update_profile_img(&auth.user_id, file).await {

    Ok(_) => return ApiResponse {
      json: json!({"result": "ok"}),
      status: Status::Ok,
    },

    Err(err) => return ApiResponse {
      json: json!({"result": "err", "detail": err}),
      status: Status::BadRequest,
    }

  };

}