//use crate::crudmodels::user::{UserInsertable,User};
use crate::diesel::RunQueryDsl;
use diesel::prelude::*;

use diesel::query_dsl::methods::FilterDsl;
use diesel::{PgConnection};
use diesel::expression_methods::ExpressionMethods;
use diesel::query_dsl::QueryDsl;
use crate::crudmodels;
pub fn insert_new_user_session <'a>(
    nu: &'a crate::crudmodels::user_sessions::UserSessionInsertable,
    conn: &PgConnection
,
) -> Result<crate::crudmodels::user_sessions::UserSession, diesel::result::Error> {
    
    use crate::schema::users;

    let ret_val = diesel::insert_into(users::table).values(nu).get_result(conn);
    return ret_val;
}