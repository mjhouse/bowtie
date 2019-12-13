
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