extern crate diesel;

use diesel::{Queryable,Insertable};
use chrono::{NaiveDateTime};
use crate::schema::user_sessions;


#[derive(Debug, Clone, Queryable)]
pub struct UserSession {
    pub id: i32,
    pub user_id:  i32,
    pub uuid: String,
    pub created_on:  NaiveDateTime,
}

#[derive(Debug, Clone,Insertable)]
#[table_name="user_sessions"]
pub struct UserSessionInsertable {
    pub user_id: i32,
    pub uuid: String,
}