use diesel::{Insertable, Queryable, Selectable};

use crate::schema::home;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = home)]
pub struct Home {
    pub id: i32,
    pub street: String,
    pub number: String,
    pub floor: String,
    pub zipcode: String,
    pub squaremeters: String,
    pub number_of_bathrooms: String,
    pub number_of_rooms: String,
    pub home_type: String,
}

impl Home {
  pub fn to_table(&self) -> String {
    format!(
      "id: {}, street: {}, number: {}, floor: {}, zipcode: {}, squaremeters: {}, number_of_bathrooms: {}, number_of_rooms: {}, home_type: {}",
      self.id, self.street, self.number, self.floor, self.zipcode, self.squaremeters, self.number_of_bathrooms, self.number_of_rooms, self.home_type
    )
  }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = home)]
pub struct NewHome {
  pub street: String,
  pub number: String,
  pub floor: String,
  pub zipcode: String,
  pub squaremeters: String,
  pub number_of_bathrooms: String,
  pub number_of_rooms: String,
  pub home_type: String,
}
