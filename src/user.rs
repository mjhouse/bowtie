use rusqlite::{Connection};
use serde::{Serialize, Deserialize};
use base64::{encode, decode};
use whirlpool::{Whirlpool, Digest};

const SELECT_ID:       &str = "SELECT * FROM users WHERE id = ?1";
const SELECT_USERNAME: &str = "SELECT * FROM users WHERE username = ?1";
const INSERT_USER:     &str = "INSERT INTO users (username, passhash) VALUES(?1,?2);";

const DATABASE: &str = "data/bowtie.db";

macro_rules! hash {
    ( $s:expr ) => { Whirlpool::new().chain(&$s).result(); }
}

macro_rules! impl_from {
    ( $n:ident, $q:ident ) => {
        pub fn $n( t_value:&str ) -> Option<User> {
            match Connection::open(DATABASE) {
                Ok(conn) => {
                    conn.query_row($q,params![t_value],
                        |row| Ok(
                            User {
                                id:       row.get(0)?,
                                username: row.get(1)?,
                                passhash: row.get(2)?,
                            })).ok()
                }
                _ => None
            }
        }
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub passhash: String,
}

impl User {

    impl_from!(from_id,SELECT_ID);
    impl_from!(from_username,SELECT_USERNAME);

    pub fn validate( &self, t_password:&str ) -> Option<String> {
        let given_hash = encode(&hash!(t_password));
        if self.passhash == given_hash {
            Some(given_hash)
        }
        else {
            None
        }
    }

    pub fn create( t_username:&str, t_password:&str ) -> Option<User> {
        if let Ok(conn) = Connection::open(DATABASE) {
            let passhash = encode(&hash!(t_password));
            let result = conn.execute(INSERT_USER,params![t_username,&passhash]);
            if result.is_ok() {
                let id = conn.last_insert_rowid();
                if id != 0 {
                    return Some(User {
                        id: id,
                        username: t_username.to_string(),
                        passhash: passhash.to_string()
                    });
                }
            }
        }
        None
    }

}
