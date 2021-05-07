use diesel::{Queryable, Insertable};
use super::super::schema::*;
use serde_derive::*;
use rocket::*;

#[derive(Queryable, Serialize)]
pub struct UserEntity {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub dob: String,
}

#[derive(Insertable, Deserialize, Serialize, FromForm)]
#[table_name="users"]
pub struct User {
  pub name: String,
  pub email: String,
  pub dob: String,
}

#[derive(Insertable)]
pub struct AuthInfo {
  pub user_id: i32,
  pub password_hash: String,
  pub mfa_enabled: bool
}

#[derive(Queryable)]
pub struct AuthInfoEntity {
  pub id: i32,
  pub user_id: i32,
  pub password_hash: String,
  pub mfa_enabled: bool
}

#[derive(FromForm, Deserialize)]
pub struct CreateInfo {
  pub name: String,
  pub email: String,
  pub dob: String,
  pub password: String
}

#[derive(FromForm, Deserialize)]
pub struct LoginInfo {
  pub username: String,
  pub password: String
}

#[derive(Serialize)]
pub struct LoginResult {
  pub uid: i32,
  pub mfa_enabled: bool
}
