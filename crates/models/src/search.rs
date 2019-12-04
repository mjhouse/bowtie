use diesel::prelude::*;
use diesel::pg::Pg;
use std::env;

use serde::{Serialize};
use rocket::request::{FromForm, FormItems};

pub use bowtie_data::schema::*;
use crate::{Post,User,UserModel,PostModel};

macro_rules! conn_or {
    ( $v:expr ) => {
        match db!() {
            Some(c) => c,
            _ => return $v
        };
    }
}

macro_rules! unpack {
    ( $i:ident ) => {
        (
            $i.key.as_str(),
            $i.value
            .url_decode()
            .map_err(|e| {
                warn!("{}",e); 
                SearchError::BadDecode
            })?.as_str() 
        )
    }
}

macro_rules! into {
    ( $r:ident ) => { $r.into_iter().map(|m| m.into()).collect() }
}

macro_rules! apply {
    ( $q:ident, $c:ident, $v:ident ) => { $q.or_filter($c.like(format!("%{}%",&$v.value))) }
}

macro_rules! impl_search {
    (   
        name:   $n:ident,
        table:  $t:expr,
        fields: [ $( $f:path ),* ],
        result: $m:ident -> $o:ident
    ) => {
        pub fn $n(t_conn: &PgConnection, t_query: &SearchQuery) -> Vec<$o> {
            let mut query = $t.into_boxed::<Pg>();
            let mut applicable = false;
            for field in t_query.fields
                .iter()
                .map(|f| f.into())
                .collect::<Vec<FieldType>>() {
                match field {
                    $( $f(c) => { query = apply!(query,c,t_query); applicable = true; }, )*
                    _ => ()
                };
            }
            if applicable {
                match query.load::<$m>(t_conn) {
                    Ok(r)  => into!(r),
                    Err(e) => {
                        warn!("{}",e);
                        vec![]
                    }
                }
            } else {
                vec![]
            }
        } 
    }
}

#[derive(Debug)]
pub enum SearchError {
    Unknown,
    BadDecode,
    UnknownFields
}

#[derive(Serialize,Debug,Clone)]
pub struct SearchQuery {
    value:   String,
    fields:  Vec<Field>,
    targets: Vec<Target>,
    display: Display
}

#[derive(Serialize,Debug)]
pub struct Search {
    pub users: Vec<User>,
    pub posts: Vec<Post>,
    pub query: SearchQuery
}

impl Search {

    pub fn from( t_query: &SearchQuery ) -> Option<Self> {
        let conn = conn_or!(None);
        Some(Self {
            users: Search::for_users(&conn,t_query),
            posts: Search::for_posts(&conn,t_query),
            query: t_query.clone()
        })
    }

    impl_search!(
        name:   for_users,
        table:  users::table,
        fields: [ FieldType::Name,
                  FieldType::Email ],
        result: UserModel -> User
    );

    impl_search!(
        name:   for_posts,
        table:  posts::table,
        fields: [ FieldType::Title,
                  FieldType::Body ],
        result: PostModel -> Post
    );

    pub fn execute<'a,M:'a,O>(t_conn: &PgConnection,t_query: &SearchQuery) -> Vec<O>
        where 
            O: From<&'a M> {
        vec![]
    }

}

#[derive(Serialize,Debug,Clone)]
pub enum Display {
    Grid,
    Rows
}

#[derive(Serialize,Debug,Clone)]
pub enum Field {
    Name,
    Email,
    Title,
    Body
}

#[derive(Serialize,Debug,Clone)]
pub enum Target {
    People,
    Posts,
    // Groups
}

#[derive(Debug)]
pub enum FieldType {
    Name(users::username),
    Email(users::email),
    Title(posts::title),
    Body(posts::body)
}

#[derive(Debug)]
pub enum TargetType {
    People(users::table),
    Posts(posts::table)
//    Groups(groups::table)
}

impl From<&Field> for FieldType {
    fn from(t_field: &Field) -> Self {
        match t_field {
            Field::Name  => FieldType::Name(users::username),
            Field::Email => FieldType::Email(users::email),
            Field::Title => FieldType::Title(posts::title),
            Field::Body  => FieldType::Body(posts::body)
        }
    }
}

impl From<&Target> for TargetType {
    fn from(t_target: &Target) -> Self {
        match t_target {
            Target::People => TargetType::People(users::table),
            Target::Posts  => TargetType::Posts(posts::table),
        }
    }
}

// parse a request into a search query that holds the 
// search string and info about which fields to search. 
impl<'a,'f> FromForm<'f> for SearchQuery {
    type Error = SearchError;

    fn from_form(items: &mut FormItems<'f>, strict: bool) -> Result<SearchQuery, SearchError> {
        let mut value   = String::from("%");
        let mut fields  = vec![];
        let mut targets = vec![];
        let mut display = Display::Grid;

        for item in items {
            match unpack!(item) {
                ("value" ,s) => value = s.to_string(),
                ("name"  ,_) => fields.push(Field::Name),
                ("email" ,_) => fields.push(Field::Email),
                ("title" ,_) => fields.push(Field::Title),
                ("body"  ,_) => fields.push(Field::Body),
                ("people",_) => targets.push(Target::People),
                ("posts" ,_) => targets.push(Target::Posts),
                // ("groups",_) => targets.push(Target::Groups),
                ("display","grid") => display = Display::Grid,
                ("display","rows") => display = Display::Rows,
                _ if strict => return Err(SearchError::UnknownFields),
                _ => ()
            }
        }

        Ok(SearchQuery {
            value:   value,
            fields:  fields,
            targets: targets,
            display: display
        })
    }
}