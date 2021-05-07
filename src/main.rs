#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate serde_derive;

extern crate dsentr_be_lib;

use dsentr_be_lib::auth;
use dsentr_be_lib::auth::crypto_auth;
use dsentr_be_lib::db::models::*;
use dsentr_be_lib::db::*;

use dotenv::dotenv;
use rocket::http::{Cookie, Cookies, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket::State;
use rocket_contrib::json::Json;
use std::env;

fn local_conn_string() -> String {
  dotenv().ok();
  env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

#[derive(Debug)]
enum LoginError {
  InvalidData,
  UsernameDoesNotExist,
  WrongPassword,
}

struct AuthenticatedUser {
  user_id: i32,
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedUser {
  type Error = LoginError;
  fn from_request(request: &'a Request<'r>) -> Outcome<AuthenticatedUser, LoginError> {
    let username = request.headers().get_one("username");
    let password = request.headers().get_one("password");
    match (username, password) {
      (Some(u), Some(p)) => {
        let maybe_user = user::fetch_user_by_email(&local_conn_string(), &String::from(u));
        match maybe_user {
          Some(user) => {
            let maybe_auth_info =
              auth_info::fetch_auth_info_by_user_id(&local_conn_string(), user.id);
            match maybe_auth_info {
              Some(auth_info) => {
                let hash = crypto_auth::hash_password(&String::from(p));
                if hash == auth_info.password_hash {
                  Outcome::Success(AuthenticatedUser { user_id: user.id })
                } else {
                  Outcome::Failure((Status::Forbidden, LoginError::WrongPassword))
                }
              }
              None => Outcome::Failure((Status::MovedPermanently, LoginError::WrongPassword)),
            }
          }
          None => Outcome::Failure((Status::NotFound, LoginError::UsernameDoesNotExist)),
        }
      }
      _ => Outcome::Failure((Status::BadRequest, LoginError::InvalidData)),
    }
  }
}

fn main() {
  rocket::ignite()
    .mount("/", routes![create, fetch_special, login_post, logout])
    .manage(local_conn_string())
    .launch();
}

#[post("/api/users/create", format = "json", data = "<create_info>")]
fn create(database_url: State<String>, create_info: Json<CreateInfo>) -> Json<i32> {
  let user_id: i32 = auth::signup::create_signup(database_url, create_info);
  Json(user_id)
}

#[post("/api/users/login", format = "json", data = "<login_info>")]
fn login_post(
  db: State<String>,
  login_info: Json<LoginInfo>,
  mut cookies: Cookies,
) -> Json<Option<LoginResult>> {
  let maybe_user = user::fetch_user_by_email(&db, &login_info.username);
  match maybe_user {
    Some(user) => {
      let maybe_auth = auth_info::fetch_auth_info_by_user_id(&db, user.id);
      match maybe_auth {
        Some(auth_info) => {
          let hash = auth::crypto_auth::hash_password(&login_info.password);
          if hash == auth_info.password_hash {
            cookies.add_private(Cookie::new("user_id", user.id.to_string()));
            let login_result: LoginResult = LoginResult {
              uid: user.id,
              mfa_enabled: auth_info.mfa_enabled,
            };
            Json(Some(login_result))
          } else {
            Json(None)
          }
        }
        None => Json(None),
      }
    }
    None => Json(None),
  }
}

#[post("/api/users/logout", format = "json")]
fn logout(mut cookies: Cookies) -> () {
  cookies.remove_private(Cookie::named("user_id"));
}

#[get("/api/users/cookies/<uid>")]
fn fetch_special(db: State<String>, uid: i32, mut cookies: Cookies) -> Json<Option<UserEntity>> {
  let logged_in_user = cookies.get_private("user_id");
  match logged_in_user {
    Some(c) => {
      let logged_in_uid = c.value().parse::<i32>().unwrap();
      if logged_in_uid == uid {
        Json(user::fetch_user_by_id(&db, uid))
      } else {
        Json(None)
      }
    }
    None => Json(None),
  }
}
