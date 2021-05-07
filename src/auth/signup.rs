use diesel::prelude::*;
use rocket_contrib::json::Json;
use rocket::State;

use super::super::db::models::*;
use super::schema::*;
use super::crypto_auth;

pub fn create_signup(database_url: State<String>, create_info: Json<CreateInfo>) -> i32 {
  let user: User = User {
    name: create_info.name.clone(),
    email: create_info.email.clone(),
    dob: create_info.dob.clone(),
  };

  let connection = PgConnection::establish(&database_url).expect("Cannot connect to database");
  let user_entity: UserEntity = diesel::insert_into(users::table)
    .values(user)
    .get_result(&connection)
    .expect("Error saving user");

  let password_hash = crypto_auth::hash_password(&create_info.password);
  let auth_info: AuthInfo = AuthInfo {
    user_id: user_entity.id,
    password_hash: password_hash,
    mfa_enabled: false,
  };
  let auth_info_entity: AuthInfoEntity = diesel::insert_into(auth_infos::table)
    .values(auth_info)
    .get_result(&connection)
    .expect("Error saving auth info");
  user_entity.id
}
