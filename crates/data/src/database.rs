
extern crate r2d2;
extern crate r2d2_diesel;
extern crate diesel;
use std::env;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

use rocket::{
    State,
    request::{FromRequest,Request,Outcome},
    http::{Status}
};

use std::ops::Deref;

#[macro_export]
macro_rules! db {
    () => {
        match env::var("DATABASE_URL") {
            Ok(p) => {
                match PgConnection::establish(&p) {
                    Ok(c) => Some(c),
                    _ => None
                } 
            },
            Err(e) => {
                warn!("Could not connect to database: {}",e);
                None
            }
        }
    };
    ( $r:expr ) => {
        match db!() {
            Some(c) => c,
            None => return $r
        }
    }
}

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Conn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl Conn {

    pub fn initialize( t_variable: &str ) -> Pool {
        Pool::new(ConnectionManager::<PgConnection>::new(
            env::var(t_variable)
            .expect("No Url Variable")))
            .expect("No Connection")
    }

}

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Conn, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Conn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}