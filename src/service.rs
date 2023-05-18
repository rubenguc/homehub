use crate::repository::HomeRepository;
use crate::models::{Home, NewHome};
use diesel::result::Error;

pub struct HomeService {
  pub repository: HomeRepository
}

impl HomeService{
  pub fn new() -> Self {
    HomeService {
      repository: HomeRepository::new()
    }
  }

  pub fn create(&mut self, home: NewHome) -> Result<Home, Error> {
    self.repository.create(home)
  }

  pub fn get_all(&mut self) -> Result<Vec<Home>, Error> {
    self.repository.find_all()
  }

  pub fn update_one(&mut self, id: i32, home: NewHome) -> Result<Home, Error> {
    self.repository.update_one(id, home)
  }

  pub fn delete_one(&mut self, id: i32) -> Result<usize, Error> {
    self.repository.delete_one(id)
  }
}