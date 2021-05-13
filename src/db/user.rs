use diesel::prelude::*;

use super::db_connection::*;
use crate::db::models::UserEntity;
use crate::db::models::User;

use crate::schema::users::dsl::*;

pub fn fetch_user_by_email(database_url: &String, input_email: &String) -> Option<UserEntity> {
  let connection = db_connection(&database_url);
  let mut users_by_id: Vec<UserEntity> = users
    .filter(email.eq(input_email))
    .load::<UserEntity>(&connection)
    .expect("ErrorLoadingUser");
  if users_by_id.len() == 0 {
    None
  } else {
    Some(users_by_id.remove(0))
  }
}

pub fn fetch_user_by_id(database_url: &String, uid: i32) -> Option<UserEntity> {
  let connection = db_connection(&database_url);
  let mut users_by_id: Vec<UserEntity> = users
    .filter(id.eq(uid))
    .load::<UserEntity>(&connection)
    .expect("ErrorLoadingUser");
  if users_by_id.len() == 0 {
    None
  } else {
    Some(users_by_id.remove(0))
  }
}

pub fn insert_user(database_url: &String, user: User) -> UserEntity {
  let connection = db_connection(&database_url);
  use crate::schema::users::dsl::*;
  diesel::insert_into(users)
    .values(user)
    .get_result(&connection)
    .expect("ErrorSavingUser")
}
