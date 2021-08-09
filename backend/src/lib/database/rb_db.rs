use chrono::Utc;
use rbatis::crud::*;
use rbatis::{core::db::DBExecResult, crud::Skip, rbatis::Rbatis, Error};

use super::entities::{Token, User};
pub struct RbDb {
    pub rb: Rbatis,
}

impl RbDb {
    pub fn new() -> RbDb {
        RbDb { rb: Rbatis::new() }
    }
    pub async fn SaveObj<T>(&self, obj: &T, skips: &[Skip<'_>]) -> Result<DBExecResult, String>
    where
        T: CRUDTable,
    {
        let db_result = self.rb.save(obj, skips).await;

        match db_result {
            Err(err) => Err(match err {
                Error::E(err) => err,
                Error::Deserialize(err) => err,
                Error::Database(err) => err,
                _ => "Uknown error".to_owned(),
            }),
            Ok(res) => Ok(res),
        }
    }

    pub async fn get_token(&self, token: &String) -> Result<Token, rbatis::Error> {
        self.rb
            .fetch_by_column::<Token, String>("token", token)
            .await
    }

    pub async fn user_by_id(&self, id: &u32) -> Result<User, rbatis::Error> {
        self.rb.fetch_by_column::<User, u32>("id", id).await
    }

    pub async fn user_by_login(&self, login: &String) -> Result<User, rbatis::Error> {
        self.rb
            .fetch_by_column::<User, String>("login", login)
            .await
    }

    pub async fn delete_expired_tokens_by_userid(&self, id: &u32) -> Result<u64, rbatis::Error> {
        let wrapper = self
            .rb
            .new_wrapper()
            .eq("user_id", *id)
            .lt("exp_date", Utc::now().naive_utc());
        self.rb.remove_by_wrapper::<Token>(&wrapper).await
    }
}