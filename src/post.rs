use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

macro_rules! impl_from {
    ( $n:ident, $q:ident, $t:ty ) => {
        pub fn $n( t_value:$t ) -> Result<User,DatabaseError> {
            Connection::open(DATABASE)
            .or_else(logs!(DatabaseError::NoConnection))
            .and_then(|c|{
                c.query_row($q,params![t_value],
                    |row| Ok(Pose {
                            id:       row.get(0)?,
                            title:    row.get(1)?,
                            body:     row.get(2)?,
                            author:   row.get(3)?,
                            created:  DateTime::parse_from_rfc3339(row.get(4)?),
                        })
                ).or_else(logs!(DatabaseError::QueryFailed))
            })
        }
    };
}

struct Post {
    id:      i64,
    title:   String,
    body:    String,
    author:  i64,
    created: DateTime<Utc>
}

impl Post {

    impl_from!(from_rowid,SELECT_ROWID,i64);

}
