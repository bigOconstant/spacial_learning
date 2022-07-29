

extern crate diesel;

//use diesel::query_dsl::methods::FilterDsl;
use diesel::{PgConnection};
use self::diesel::prelude::*;

use crate::crudmodels::*;

pub fn login<'a> (
    un: &'a String,
    conn: &PgConnection
) -> Result<user::User, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    //use crate::schema;//::users::dsl::*;
    let results = users.filter(username.eq(un))
    .limit(1)
    .first::<user::User>(conn);
    //.expect("Error loading posts");
     //   .first(&connection);//users.filter(username.eq(un)); //filter(username.eq(un)).first::<crate::crudmodels::user::User>(conn);
    return results;
}