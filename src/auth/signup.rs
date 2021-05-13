extern crate diesel;

use rocket::State;
use rocket_contrib::json::Json;

use crate::auth::crypto_auth;
use crate::db::models::*;
use crate::db::user;
use crate::db::auth_info;

pub fn create_signup(database_url: State<String>, create_info: Json<CreateInfo>) -> Result<CreateResult, &str> {
  let user_to_insert: User = User {
    email: create_info.email.clone(),
    dob: create_info.dob.clone(),
    kyc_level: 0
  };
  let user = user::fetch_user_by_email(&database_url, &user_to_insert.email);
  match user {
    None => {
      let user_entity = user::insert_user(&database_url, user_to_insert);

      let password_hash = crypto_auth::hash_password(&create_info.password);
      let auth_info: AuthInfo = AuthInfo {
        user_id: user_entity.id,
        password_hash: password_hash,
        mfa_enabled: false,
      };
      let auth_info_entity = auth_info::insert_auth_info(&database_url, auth_info);
      Ok(CreateResult { uid: user_entity.id, auth_id: auth_info_entity.id })
    },
    Some(_user) => Err("User already exists")
  }
}
