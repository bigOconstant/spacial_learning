use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,)]
pub struct FieldError{
    pub valid:bool,
    pub error_message:String,
}