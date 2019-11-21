use crate::schema::users;
use diesel::dsl;
use diesel::prelude::*;
use crate::models::*;
use failure::Error;

use diesel::result::Error as DieselError;


#[derive(Insertable)]
#[table_name="users"]
pub struct User {
    pub email:    String,
    pub username: String,
    pub passhash: String
}

impl User {
    
    pub fn create(t_conn: &PgConnection, t_email: &str, t_username: &str, t_passhash: &str) -> Result<User,DieselError> {
        let new_user = User {
            email:    t_email.into(),
            username: t_username.into(),
            passhash: t_passhash.into()
        };
    
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(t_conn)
            .or_else(|e|  Err(e))
            .and_then(|m: UserModel| Ok(m.into()))
    }

    pub fn find_by_username(t_conn: &PgConnection, t_username: &str) -> Option<User> {
        None
    }

    pub fn find_by_passhash(t_conn: &PgConnection, t_passhash: &str) -> Option<User> {
        None
    }

    pub fn find_by_id(t_conn: &PgConnection, t_id: i64) -> Option<User> {
        None
    }

    pub fn all(t_conn: &PgConnection) -> Vec<User> {
        match users::table.load::<UserModel>(t_conn) {
            Ok(v)  => v.into_iter().map(|m| m.into()).collect(),
            Err(e) => {
                warn!("Error during query: {}",e);
                vec![]
            }
        }
    }

    pub fn all_slice(t_conn: &PgConnection, t_offset: i64, t_limit: i64) -> Vec<User> {            
        match users::table
            .offset(t_offset)
            .limit(t_limit)
            .load::<UserModel>(t_conn)
        {
            Ok(v)  => v.into_iter().map(|m| m.into()).collect(),
            Err(e) => {
                warn!("Error during query: {}",e);
                vec![]
            }
        }
    }

}




impl From<UserModel> for User {
    fn from(model: UserModel) -> Self {
        User {
            email: model.email,
            username: model.username,
            passhash: model.passhash
        }
    }
}

impl From<User> for UserModel {
    fn from(user: User) -> Self {
        UserModel {
            id: -1,
            email: user.email,
            username: user.username,
            passhash: user.passhash
        }
    }
}
