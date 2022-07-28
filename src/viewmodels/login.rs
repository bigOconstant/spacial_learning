use serde::{Deserialize, Serialize};

use super::shared::FieldError;

#[derive(Serialize, Deserialize,)]
pub struct LoginCheck{
    pub username:FieldError,
    pub password:FieldError,
}


#[derive(Serialize, Deserialize,)]
pub struct Login {
    pub username: String,
    pub password: String,
}

impl LoginCheck {
    pub fn new()->LoginCheck {
        return LoginCheck { username: FieldError{valid:true,error_message:"".to_string()}, password: FieldError{valid:true,error_message:"".to_string()}};
    }
    pub fn has_error(&self)->bool {
        if !self.username.valid || !self.password.valid  {
            return true;
        }else {
            return false
        }
    }

    pub fn set_username_error(&mut self,err_message:&str) {
        self.username.error_message = err_message.to_string();
        self.username.valid = false;
    }

    pub fn set_password_error(&mut self,err_message:&str) {
        self.password.error_message = err_message.to_string();
        self.password.valid = false;
    }
}

impl Login {
    pub fn new()->Login {
        return Login { username: "".to_string(), password: "".to_string()};
    }

    pub fn set_error(&self)-> LoginCheck {
        let mut ret_val = LoginCheck::new();
        if self.username.is_empty(){
            ret_val.username.valid = false;
            ret_val.username.error_message = "Username required".to_string();
        } else if self.username.chars().count() < 4 || self.username.chars().count() > 20 {
            ret_val.username.valid = false;
            ret_val.username.error_message = "Username should be atleast 4 characters and less than 20".to_string();
        }

        if self.password.is_empty(){
            ret_val.password.valid = false;
            ret_val.password.error_message = "password required".to_string();
        } else if self.password.chars().count() < 4 || self.password.chars().count() > 20 {
            ret_val.password.valid = false;
            ret_val.password.error_message = "password should be atleast 4 characters and less than 20. ".to_string();
        }

        return ret_val;
    }
}