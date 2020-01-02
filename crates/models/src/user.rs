pub use bowtie_data::schema::*;

use base64::encode;
use diesel::prelude::*;
use whirlpool::{Whirlpool, Digest};
use serde::{Serialize};
use failure::*;

use crate::ViewModel;

macro_rules! hash {
    ( $s:expr ) => { Whirlpool::new().chain(&$s).result(); }
}

#[derive(Serialize, Queryable, Debug)]
pub struct User {
    pub id:       i32,
    pub email:    Option<String>,
    pub username: String,
    pub passhash: String
}

#[derive(Identifiable,Insertable,Debug,Serialize)]
#[table_name="users"]
pub struct UserModel {
    pub id:       Option<i32>,
    pub email:    Option<String>,
    pub username: String,
    pub passhash: String
}

impl User {

    pub fn create_from(
        t_conn: &PgConnection, 
        t_name: &str, 
        t_password: &str) -> Result<User,Error> {

        t_conn.transaction::<_, Error, _>(|| {
            // create model
            let result: User = 
            diesel::insert_into(users::table)
                .values(&UserModel {
                    id:       None,
                    email:    None,
                    username: t_name.into(),
                    passhash: encode(&hash!(t_password))
                })
                .get_result(t_conn)?;
        
            // create a default view
            diesel::insert_into(views::table)
                .values(&ViewModel {
                    id: None,
                    user_id: result.id,
                    name: result.username.clone()                    
                })
                .execute(t_conn)?;
            
            Ok(result)
        })

    }

    pub fn delete(t_conn: &PgConnection, t_user: i32) -> Result<User,Error> {
        t_conn.transaction::<_, Error, _>(|| {
            // find all view models
            let ids = views::table
                .filter(views::user_id.eq(t_user))
                .select(views::id)
                .load::<i32>(t_conn)?;

            // delete all posts associated with the user's views
            diesel::delete(posts::table)
                .filter(posts::view_id.eq_any(ids))
                .execute(t_conn)?;

            // delete all views associated with the user
            diesel::delete(views::table)
                .filter(views::user_id.eq(t_user))
                .execute(t_conn)?;

            // delete the user
            let result =
            diesel::delete(users::table)
                .filter(users::id.eq(t_user))
                .get_result(t_conn)?;

            Ok(result)
        })
    }

    pub fn validate( &self, t_password:&str ) -> bool {
        let hash = encode(&hash!(t_password));
        self.passhash == hash
    }

    pub fn for_id(t_conn: &PgConnection, t_id: i32) -> Result<User,Error> {
        let result = users::table
            .filter(users::id.eq(t_id))
            .first::<User>(t_conn)?;
        Ok(result)
    }

    pub fn for_name(t_conn: &PgConnection, t_name: &str) -> Option<User> {
        users::table
            .filter(users::username.eq(t_name))
            .first::<User>(t_conn).ok()
    }

}
