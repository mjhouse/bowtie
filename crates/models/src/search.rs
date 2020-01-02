use diesel::prelude::*;
use diesel::pg::Pg;

use serde::{Serialize};
use rocket::request::{FromForm, FormItems};

pub use bowtie_data::schema::*;
use crate::{View,Post};

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

macro_rules! apply {
    ( $q:ident, $c:path, $v:ident ) => { $q.or_filter($c.like(format!("%{}%",&$v.value))) }
}

macro_rules! impl_search {
    (   
        name:   $n:ident,
        table:  $t:path,
        target: $a:path,
        fields: [ $( $f:path ),* ],
        result: $m:ident
    ) => {
        pub fn $n(t_conn: &PgConnection, t_search: &SearchQuery) -> Vec<$m> {
            if !t_search.targets.contains(&$a) {
                return vec![]; }

            let mut query = $t.into_boxed::<Pg>();
            $( 
                query = apply!(query,$f,t_search); 
            )*

            match query.load::<$m>(t_conn) {
                Ok(r)  => r,
                Err(e) => {
                    warn!("{}",e);
                    vec![]
                }
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
    targets: Vec<Target>
}

#[derive(Serialize,Debug)]
pub struct Search {
    pub views: Vec<View>,
    pub posts: Vec<Post>,
    pub query: SearchQuery
}

impl Search {

    pub fn from( t_conn: &PgConnection, t_query: &SearchQuery ) -> Option<Self> {
        let (v,p) = match t_query.targets.len() {
            0 => (vec![],vec![]),
            _ => {
                (
                    Search::for_views(t_conn,t_query),
                    Search::for_posts(t_conn,t_query) 
                )
            }
        };
        
        Some(Self {
            views: v,
            posts: p,
            query: t_query.clone()
        })
    }

    impl_search!(
        name:   for_views,
        table:  views::table,
        target: Target::People,
        fields: [ views::name ],
        result: View
    );

    impl_search!(
        name:   for_posts,
        table:  posts::table,
        target: Target::Posts,
        fields: [ posts::title,
                  posts::body ],
        result: Post
    );

}

#[derive(Serialize,Debug,PartialEq,Clone)]
pub enum Target {
    People,
    Posts
}

// parse a request into a search query that holds the 
// search string and info about which fields to search. 
impl<'a,'f> FromForm<'f> for SearchQuery {
    type Error = SearchError;

    fn from_form(items: &mut FormItems<'f>, strict: bool) -> Result<SearchQuery, SearchError> {
        let mut value   = String::new();
        let mut targets = vec![];

        for item in items {
            match unpack!(item) {
                ("value" ,s) => value = s.to_string(),
                ("people",_) => targets.push(Target::People),
                ("posts" ,_) => targets.push(Target::Posts),
                _ if strict => return Err(SearchError::UnknownFields),
                _ => ()
            }
        }

        Ok(SearchQuery {
            value:   value,
            targets: targets
        })
    }
}