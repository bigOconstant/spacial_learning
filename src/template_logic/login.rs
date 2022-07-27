use actix_web::{error, web,  Error, HttpResponse, Result};

use std::collections::HashMap;
use argon2::Config;
use rand::Rng;
use diesel::{
    r2d2::{ConnectionManager},
    PgConnection,
  };
type DbPool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;


use crate::models::*;
use crate::view_models::*;