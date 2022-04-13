use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use crate::db::schema;
use super::schema::{users, scores};

#[derive(Queryable)]
pub struct User {
    id:         i32,
    name: String,
    images: i32,
}

impl User{
    pub fn get_id(&self) -> i32{
        self.id
    }
}


#[derive(Queryable)]
pub struct Score {
    id:         i32,
    epoch: chrono::NaiveDate,
    users: i32,
}
impl Score{
    pub fn get_id(&self)-> i32{
        self.id
    }
    pub fn user(&self,db: &PgConnection)->Option<User>{
        use schema::scores::dsl::*;

        match scores.find(self.users).get_result::<User>(&db){
            Ok(res) => Some(res),
            Err(_)=>None
        }
    }
}

#[derive(Queryable)]
pub struct Image {
    id:         i32,
    image: Vec<u8>
}