use super::entities::*;
use super::validation::Validation;

impl Validation for LoginRequest {
    fn validate(&self) -> Result<(), &str> {
        Ok(())
    }
}

impl Validation for RegistrationRequest {
    fn validate(&self) -> Result<(), &str> {
        if !(3..=32).contains(&self.login.len()) {
            return Err("Login length can be from 3 to 32");
        }
        if !self.login.chars().all(char::is_alphanumeric) {
            return Err("Login is not aplhanumeric");
        }

        if !(3..=32).contains(&self.password.len()) {
            return Err("Password length can be from 3 to 32");
        }
        if !self.password.chars().all(char::is_alphanumeric) {
            return Err("Password is not aplhanumeric");
        }

        Ok(())
    }
}

impl Validation for ChangeAvatarRequest {
    fn validate(&self) -> Result<(), &str> {
        todo!()
    }
}

impl Validation for ChangeAboutRequest {
    fn validate(&self) -> Result<(), &str> {
        todo!()
    }
}
