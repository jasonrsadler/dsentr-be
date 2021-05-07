#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate serde_derive;

use dotenv::dotenv;
use rocket::http::{Status, Cookies, Cookie};
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket_contrib::json::Json;
use rocket::State;
use std::env;

mod db;
mod auth;
use db::models::*;

fn local_conn_string() -> String {
  dotenv().ok();
  env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

#[derive(Debug)]
enum LoginError {
  InvalidData,
  UsernameDoesNotExist,
  WrongPassword
}

struct AuthenticatedUser {
  user_id: i32
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedUser {
  type Error = LoginError;
  fn from_request(request: &'a Request<'r>) -> Outcome<AuthenticatedUser, LoginError> {
    let username = request.headers().get_one("username");
    let password = request.headers().get_one("password");
    match (username, password) {
      (Some(u), Some(p)) => {
        let conn_str = local_conn_string();
        let maybe_user = fetch_user_by_email(&conn_str, &String::from(u));
        match maybe_user {
          Some(user) => {
            let maybe_auth_info = fetch_auth_info_by_user_id(&conn_str, user.id);
            match maybe_auth_info {
              Some(auth_info) => {
                let hash = hash_password(&String::from(p));
                if hash == auth_info.password_hash {
                  Outcome::Success(AuthenticatedUser{user_id: user.id})
                } else {
                  Outcome::Failure((Status::Forbidden, LoginError::WrongPassword))
                }
              }
              None => {
                Outcome::Failure((Status::MovedPermanently, LoginError::WrongPassword))
              }
            }
          }
          None => {
            Outcome::Failure((Status::NotFound, LoginError::UsernameDoesNotExist))
          }
        }
      },
      _ => Outcome::Failure((Status::BadRequest, LoginError::InvalidData))
    }
  }
}

fn fetch_auth_info_by_user_id(database_url: &String, uid: i32) -> Option<AuthInfoEntity> {
  use dsentr_be::schema::auth_infos::dsl::*;
  let connection = PgConnection::establish(&database_url).expect("Error connecting to database");
  let mut auth_info_by_uid: Vec<AuthInfoEntity> = auth_infos.filter(user_id.eq(uid))
    .load::<AuthInfoEntity>(&connection).expect("Error loading auth info");
  if auth_info_by_uid.len() == 0 {
    None
  } else {
    Some(auth_info_by_uid.remove(0))
  }
}

fn fetch_user_by_email(database_url: &String, input_email: &String) -> Option<UserEntity> {
  use dsentr_be::schema::users::dsl::*;
  let connection = PgConnection::establish(&database_url).expect("Error connecting to database");
  let mut users_by_id: Vec<UserEntity> = users.filter(email.eq(input_email))
    .load::<UserEntity>(&connection).expect("Error loading user");
  if users_by_id.len() == 0 {
    None
  } else {
    Some(users_by_id.remove(0))
  }
}

fn fetch_user_by_id(database_url: &String, uid: i32) -> Option<UserEntity> {
  use dsentr_be::schema::users::dsl::*;
  let connection = PgConnection::establish(&database_url).expect("Error connecting to database");
  let mut users_by_id: Vec<UserEntity> = users.filter(id.eq(uid))
    .load::<UserEntity>(&connection).expect("Error loading user");
    if users_by_id.len() == 0 {
      None
    } else {
      Some(users_by_id.remove(0))
    }
}

fn main() {
  rocket::ignite().mount("/",
    routes![
      create,
      fetch_special,
      login_post,
      logout
    ]).manage(local_conn_string()).launch();
}

#[post("/api/users/create", format="json", data="<create_info>")]
fn create(database_url: State<String>, create_info: Json<CreateInfo>) -> Json<i32> {
  let user_id: i32 = auth::signup::create_signup(database_url, create_info);
  Json(user_id)
}

#[post("/api/users/login", format="json", data="<login_info>")]
fn login_post(db: State<String>, login_info: Json<LoginInfo>, mut cookies: Cookies) -> Json<Option<LoginResult>> {
  let maybe_user = fetch_user_by_email(&db, &login_info.username);
  match maybe_user {
    Some(user) => {
      let maybe_auth = fetch_auth_info_by_user_id(&db, user.id);
      match maybe_auth {
        Some(auth_info) => {
          let hash = hash_password(&login_info.password);
          if hash == auth_info.password_hash {
            cookies.add_private(Cookie::new("user_id", user.id.to_string()));
            let login_result: LoginResult = LoginResult {
              uid: user.id,
              mfa_enabled: auth_info.mfa_enabled
            };
            Json(Some(login_result))
          } else {
            Json(None)
          }
        }
        None => Json(None)
      }
    }
    None => Json(None)
  }
}

#[post("/api/users/logout", format="json")]
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
        Json(fetch_user_by_id(&db, uid))
      } else {
        Json(None)
      }
    },
    None => Json(None)
  }
}
