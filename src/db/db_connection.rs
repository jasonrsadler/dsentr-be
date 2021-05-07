use diesel::Connection;
use diesel::pg::PgConnection;

pub fn db_connection(database_url: &String) -> diesel::PgConnection {
  PgConnection::establish(&database_url).expect("Error connecting to database")
}