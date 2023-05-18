use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;
use std::env;

use crate::models::{Home, NewHome};
use crate::schema::home::{dsl::*};


pub struct HomeRepository {
  pub conn: SqliteConnection
}

impl HomeRepository {

  pub fn new() -> Self {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
    
    HomeRepository {
      conn
    }
  }

  pub fn find_all(&mut self) -> Result<Vec<Home>, Error> {
    home.load::<Home>(&mut self.conn)
  }

  pub fn create(&mut self, new_home: NewHome) -> Result<Home, Error> {
    diesel::insert_into(home)
      .values(new_home)
      .execute(&mut self.conn)?;

    home.order(id.desc()).first(&mut self.conn)
  }

  pub fn update_one(&mut self, home_id: i32, new_home: NewHome) -> Result<Home, Error> {
    diesel::update(home.filter(id.eq(home_id)))
      .set((
        street.eq(new_home.street),
        number.eq(new_home.number),
        floor.eq(new_home.floor),
        zipcode.eq(new_home.zipcode),
        squaremeters.eq(new_home.squaremeters),
        number_of_bathrooms.eq(new_home.number_of_bathrooms),
        number_of_rooms.eq(new_home.number_of_rooms),
        home_type.eq(new_home.home_type)
      ))
      .execute(&mut self.conn)?;

    home.filter(id.eq(home_id)).first(&mut self.conn)
  }

  pub fn delete_one(&mut self, home_id: i32) -> Result<usize, Error> {
    diesel::delete(home.filter(id.eq(home_id)))
      .execute(&mut self.conn)
  }

} 