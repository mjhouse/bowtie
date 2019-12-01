use diesel::prelude::*;
use diesel::pg::Pg;
use std::env;

use serde::{Serialize};
use rocket::request::{FromForm, FormItems};

pub use bowtie_data::schema::*;
use crate::{Post,User,UserModel,PostModel};

#[derive(Debug)]
pub enum SearchError {
    Unknown,
    BadDecode,
    UnknownFields
}

#[derive(Debug)]
pub enum Field {
    Name,
    Email,
    Title,
    Body
}

#[derive(Debug)]
pub struct SearchQuery {
    value:  String,
    fields: Vec<Field>,
}

#[derive(Serialize,Debug)]
pub struct Search {
    pub users: Vec<User>,
    pub posts: Vec<Post>
}

macro_rules! conn_or {
    ( $v:expr ) => {
        match db!() {
            Some(c) => c,
            _ => return $v
        };
    }
}

impl SearchQuery {

    pub fn has_fields(&self) -> bool {
        self.fields.len() > 0
    }

}

impl Search {

    pub fn from( t_query: &SearchQuery ) -> Option<Self> {
        let conn = conn_or!(None);
        Some(Self {
            users: Search::for_users(&conn,t_query),
            posts: Search::for_posts(&conn,t_query)
        })
    }

    pub fn for_users( t_conn: &PgConnection, t_query: &SearchQuery) -> Vec<User> {
        let mut query = users::table.into_boxed::<Pg>();

        if t_query.has_fields() {
            for v in t_query.fields.iter() {
                match v {
                    Field::Name  => { query = query.or_filter(users::username.like(format!("%{}%",&t_query.value))); },
                    Field::Email => { query = query.or_filter(users::email.like(format!("%{}%",&t_query.value))); },
                    _ => ()
                };
            }
        }
        else {
            query = query.or_filter(users::username.like(format!("%{}%",&t_query.value)))
                         .or_filter(users::email.like(format!("%{}%",&t_query.value)));
        }

        match query.load::<UserModel>(t_conn) {
            Ok(r) => {
                r.into_iter()
                 .map(|m| m.into())
                 .collect()                
            },
            Err(e) => {
                warn!("{}",e);
                vec![]}
        }
    }

    pub fn for_posts( t_conn: &PgConnection, t_query: &SearchQuery) -> Vec<Post> {
        let mut query = posts::table.into_boxed::<Pg>();

        if t_query.has_fields() {
            for v in t_query.fields.iter() {
                match v {
                    Field::Title => { query = query.or_filter(posts::title.like(format!("%{}%",&t_query.value))); },
                    Field::Body => { query = query.or_filter(posts::body.like(format!("%{}%",&t_query.value))); },
                    _ => ()
                };
            }
        }
        else {
            query = query.or_filter(posts::title.like(format!("%{}%",&t_query.value)))
                         .or_filter(posts::body.like(format!("%{}%",&t_query.value)));
        }

        match query.load::<PostModel>(t_conn) {
            Ok(r) => {
                r.into_iter()
                 .map(|m| m.into())
                 .collect()                
            },
            Err(_) => vec![]
        }
    }

}

macro_rules! unpack_item {
    ( $i:ident ) => {
        (
            $i.key.as_str(),
            $i.value
            .url_decode()
            .map_err(|e| {
                warn!("{}",e); 
                SearchError::BadDecode
            })? 
        )
    }
}

impl<'a,'f> FromForm<'f> for SearchQuery {
    type Error = SearchError;

    fn from_form(items: &mut FormItems<'f>, strict: bool) -> Result<SearchQuery, SearchError> {
        let mut value  = String::from("%");
        let mut fields = vec![];

        for item in items {
            match unpack_item!(item) {
                ("value",s) => value = s,
                ("name" ,_) => fields.push(Field::Name),
                ("email",_) => fields.push(Field::Email),
                ("title",_) => fields.push(Field::Title),
                ("body" ,_) => fields.push(Field::Body),
                _ if strict => return Err(SearchError::UnknownFields),
                _ => ()
            }
        }

        Ok(SearchQuery {
            value:  value,
            fields: fields
        })
    }
}