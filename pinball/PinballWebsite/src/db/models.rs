use chrono::{NaiveDateTime, NaiveTime};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use diesel::dsl::sql;
use crate::db::schema;

use crate::establish_connection;
use super::schema::{users, scores};

#[derive(Queryable,Serialize)]
pub struct User {
    id:         i32,
    name: String,
    image: i32
}

#[derive(Queryable)]
pub struct Image {
    id:         i32,
    image: Vec<u8>
}

impl User{
    pub fn get_id(&self) -> i32{
        self.id
    }
    pub fn get_name(&self)-> String{self.name.clone()}
    pub fn remove_user(&self) -> Option<i32>{
        use schema::users::dsl::*;

        let role_id : Option<i32> = users.select(id).filter(id.eq(self.id)).first(&db).ok();

        let _ = diesel::delete(users.filter(id.eq(self.id))).execute(&db);
        role_id
    }
    pub fn add(id: i32, username:String)->Self{
        use schema::users::dsl::*;
        let db = establish_connection();

        let r = diesel::insert_into(ban).values((
            name.eq(username),
        )).execute(&db);

        match users.filter(id.eq(&id)).first::<User>(&db) {
            Ok(lang) => lang,
            Err(_) => {
                let r = diesel::insert_into(users).values(
                    id.eq(id)).execute(&db);
                match r {
                    Ok(_) => {
                        users.order(id.desc())
                            .first::<User>(&db)
                            .unwrap()
                    },
                    Err(e) => panic!(e),
                }
            },
    }

    pub fn username(&self) -> &str {&self.name}
    // pub fn image(&self)->&Vec<u8>{
    // }
}


#[derive(Queryable,Serialize)]
pub struct Score {
    id:         i32,
    users: i32,
    epoch: String,
}
impl Score{
    pub fn get_id(&self)-> i32{
        self.id
    }
    pub fn user(&self)->Option<User>{
        use schema::users::dsl::*;
        let db = establish_connection();

        match users.find(self.users).get_result::<User>(&db){
            Ok(res) => Some(res),
            Err(_)=>None
        }
    }
    pub fn add_score(user: User, time: NaiveDateTime)-> Self{
        use schema::scores::dsl::*;
        let db = establish_connection();

        let time = format!("{}", time.timestamp());

        let r = diesel::insert_into(scores).values((
            users.eq(user.id),
            epoch.eq(time)
            )
        ).execute(&db);
        match r{
            Ok(_) => {
                score.order(id.desc())
                    .first::<Score>(&db)
                    .unwrap()
            }
            Err(e) => panic!(e),
        }
    }

    pub fn date(&self)->&str{
        &self.epoch
    }
    pub fn remove_score(score: Score)->Option<i32>{
        use schema::scores::dsl::*;
        let db = establish_connection();

        let filter = sql(&format!("user = {}", score.id));

        let ban_id: Option<i32> = users.select(id).filter(&filter).first(&db).ok();

        let _ = diesel::delete(scores)
            .filter(filter)
            .execute(&db);

        ban_id
    }
}
