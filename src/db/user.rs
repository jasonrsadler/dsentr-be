use diesel::prelude::*;

use super::db_connection::*;
use crate::db::models::UserEntity;

pub fn fetch_user_by_email(database_url: &String, input_email: &String) -> Option<UserEntity> {
  use crate::schema::users::dsl::*;
  let connection = db_connection(&database_url);
  let mut users_by_id: Vec<UserEntity> = users
    .filter(email.eq(input_email))
    .load::<UserEntity>(&connection)
    .expect("Error loading user");
  if users_by_id.len() == 0 {
    None
  } else {
    Some(users_by_id.remove(0))
  }
}

pub fn fetch_user_by_id(database_url: &String, uid: i32) -> Option<UserEntity> {
  use crate::schema::users::dsl::*;
  let connection = db_connection(&database_url);
  let mut users_by_id: Vec<UserEntity> = users
    .filter(id.eq(uid))
    .load::<UserEntity>(&connection)
    .expect("Error loading user");
  if users_by_id.len() == 0 {
    None
  } else {
    Some(users_by_id.remove(0))
  }
}
