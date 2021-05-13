use diesel::prelude::*;

use super::db_connection::*;
use crate::db::models::AuthInfoEntity;
use crate::db::models::AuthInfo;

use crate::schema::auth_infos::dsl::*;

pub fn fetch_auth_info_by_user_id(database_url: &String, uid: i32) -> Option<AuthInfoEntity> {
  use crate::schema::auth_infos::dsl::*;
  let connection = db_connection(&database_url);
  let mut auth_info_by_uid: Vec<AuthInfoEntity> = auth_infos
    .filter(user_id.eq(uid))
    .load::<AuthInfoEntity>(&connection)
    .expect("ErrorLoadingAuthInfo");
  if auth_info_by_uid.len() == 0 {
    None
  } else {
    Some(auth_info_by_uid.remove(0))
  }
}

pub fn insert_auth_info(database_url: &String, auth: AuthInfo) -> AuthInfoEntity {
  let connection = db_connection(&database_url);
  diesel::insert_into(auth_infos)
    .values(auth)
    .get_result(&connection)
    .expect("ErrorSavingAuthInfo")
}
