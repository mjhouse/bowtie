use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

use std::ops::Deref;
use std::env;

use rocket::{
    State,
    request::{FromRequest,Request,Outcome},
    http::{Status}
};

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