use rusqlite::{Connection};
use serde::{Serialize, Deserialize};
use whirlpool::{Whirlpool, Digest};
use base64::encode;

use medallion::{
    Header,
    Payload,
    Token,
};

const SELECT_ID:       &str = "SELECT * FROM users WHERE id = ?1";
const SELECT_ROWID:    &str = "SELECT * FROM users WHERE rowid = ?1";
const SELECT_USERNAME: &str = "SELECT * FROM users WHERE username = ?1";
const INSERT_USER:     &str = "INSERT INTO users (username, passhash) VALUES(?1,?2);";
const DELETE_USER:     &str = "DELETE FROM users WHERE username = ?1 AND passhash = ?2";

const DATABASE: &str = "data/bowtie.db";
const ISSUER:   &str = "bowtie.com";
const SUBJECT:  &str = "user";

const SERVER_KEY: &[u8;10] = b"secret_key";

macro_rules! hash {
    ( $s:expr ) => { Whirlpool::new().chain(&$s).result(); }
}

macro_rules! logs {
    ( $s:expr ) => { |e| { error!("{}",e); Err($s) } }
}

macro_rules! impl_from {
    ( $n:ident, $q:ident, $t:ty ) => {
        pub fn $n( t_value:$t ) -> Result<User,DatabaseError> {
            Connection::open(DATABASE)
            .or_else(logs!(DatabaseError::NoConnection))
            .and_then(|c|{
                c.query_row($q,params![t_value],
                    |row| Ok(User {
                            id:       row.get(0)?,
                            username: row.get(1)?,
                            passhash: row.get(2)?,
                        })
                ).or_else(logs!(DatabaseError::QueryFailed))
            })
        }
    };
}

#[derive(Debug)]
pub enum DatabaseError {
    NoConnection,
    QueryFailed
}

#[derive(Debug)]
pub enum TokenError {
    FailedToSign,
    FailedToParse,
    TokenNotVerifies
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub passhash: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UserClaims {
    pub id: i64,
    pub username: String
}

impl User {

    impl_from!(from_rowid,SELECT_ROWID,i64);
    impl_from!(from_id,SELECT_ID,i64);
    impl_from!(from_username,SELECT_USERNAME,&str);

    pub fn validate( &self, t_password:&str ) -> bool {
        let given_hash = encode(&hash!(t_password));
        self.passhash == given_hash
    }

    pub fn create( t_username:&str, t_password:&str ) -> Result<User,DatabaseError> {
        Connection::open(DATABASE)
        .or_else(logs!(DatabaseError::NoConnection))
        .and_then(|c|{
            let hash = encode(&hash!(t_password));
            c.execute(INSERT_USER, params![t_username,&hash])
                      .or_else(logs!(DatabaseError::QueryFailed))
                      .and_then(|_|{
                          let id = c.last_insert_rowid();
                          User::from_rowid(id)
                      })
        })
    }

    pub fn destroy( t_username:&str, t_password:&str ) -> Result<User,DatabaseError> {
        Connection::open(DATABASE)
        .or_else(logs!(DatabaseError::NoConnection))
        .and_then(|c|{
            let hash = encode(&hash!(t_password));
            c.execute(DELETE_USER, params![t_username,&hash])
                      .or_else(logs!(DatabaseError::QueryFailed))
                      .and_then(|_|{
                          Ok(User {
                              id: -1,
                              username: t_username.into(),
                              passhash: hash.into()
                          })
                      })
        })
    }

    pub fn to_token( &self ) -> Option<String> {
        let header: Header<()> = Default::default();

        let payload = Payload {
            iss: Some(ISSUER.into()),
            sub: Some(SUBJECT.into()),
            claims: Some(self.to_claims()),
            ..Payload::default()
        };

        Token::new(header, payload)
            .sign(SERVER_KEY)
            .or_else(logs!(TokenError::FailedToSign))
            .ok()
    }

    pub fn from_token( &self, t_token:String ) -> Option<User> {
        Token::<(), SessionClaims>::parse(t_token)
        .or_else(logs!(TokenError::FailedToParse))
        .and_then(|t|{
            t.verify(SERVER_KEY)
            .or_else(logs!(TokenError::TokenNotVerifies))
            .and_then(|r|{
                if r && token.payload.claims.is_some() {
                    self.from_claims(token.payload.claims.unwrap())
                }
                else {
                    None
                }
            })
        })
    }

    pub fn to_claims( &self ) -> UserClaims {
        UserClaims {
            id:       self.id.clone()
            username: self.username.clone()
        }
    }

    pub fn from_claims( t_claims: &UserClaims ) -> User {

    }

}
