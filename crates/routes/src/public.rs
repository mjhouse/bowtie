pub mod get {

    use rocket::{
        State,
        request::{LenientForm}
    };
    
    use tera::{Context};
    
    // @todo Make bowtie_models export internal modules
    // @body These imports are getting out of hand
    
    use crate::resources::*;
    use bowtie_models::*;
    use bowtie_data::Conn;

    #[get("/")]
    pub fn index( resources: State<Resources> ) -> Page {
        Page::render(&resources,"/public/index",false)
    }
    
    #[get("/about")]
    pub fn about( resources: State<Resources> ) -> Page {
        Page::render(&resources,"/public/about",false)
    }
    
    #[get("/search?<query..>")]
    pub fn search( conn: Conn, resources: State<Resources>, query: LenientForm<SearchQuery> ) -> Page {
        Page::render(&resources,"/public/search",false)
            .with_context(context!(
                "search" => Search::from(&conn,&query)))
    }
    
    #[get("/user/<name>")]
    pub fn user( conn: Conn, session: Option<Session>, resources: State<Resources>, name: String ) -> Page {
        let (posts,view) = match View::with_posts(&conn,&name) {
            Some((v,p)) => (p,Some(v)),
            None    => (vec![],None)
        };
    
        // @todo Refactor to reduce the number of queries
        // @body There are 4 queries in the worst case at this endpoint
    
        let (followed,friended) = match (session.as_ref(),view.as_ref()) {
            (Some(s),Some(v)) => {
                ( Follow::exists(&conn,s.view,v.id),
                  Friend::exists(&conn,s.view,v.id) )
            },
            _ => (false,false)
        };
    
        Page::render(&resources,"/public/user",false)
            .with_context(context!(
                "posts"    => posts,
                "view"     => view,
                "followed" => followed,
                "friended" => friended))
    }
    
    #[get("/post/<id>?<page>&<count>")]
    pub fn post( conn: Conn, resources: State<Resources>, id: i32, page: Option<i64>, count: Option<i64> ) -> Page {
        let page_number = page.unwrap_or(0);
        let item_count  = count.unwrap_or(50);
        let start = page_number * item_count;

        let comments   = Comment::for_post(&conn,id,start,item_count);
        let submission = Post::for_id(&conn,id).ok();
    
        Page::render(&resources,"/public/post",false)
            .with_context(context!(
                "submission" => submission,
                "comments"   => comments,
                "page_number"=> page_number,
                "item_count" => item_count
            ))
    }
    
    #[get("/comment/<id>?<page>&<count>")]
    pub fn comment( conn: Conn, resources: State<Resources>, id: i32, page: Option<i64>, count: Option<i64> ) -> Page {
        let page_number = page.unwrap_or(0);
        let item_count  = count.unwrap_or(50);
        let start = page_number * item_count;
        
        let comments   = Comment::for_comment(&conn,id,start,item_count);
        let submission = Comment::for_id(&conn,id).ok();
    
        let crumbs = match submission {
            Some((_,ref c)) => c.get_path(),
            _ => vec![]
        };
    
        Page::render(&resources,"/public/comment",false)
            .with_context(context!(
                "submission" => submission,
                "comments"   => comments,
                "crumbs"     => crumbs,
                "page_number"=> page_number,
                "item_count" => item_count
            ))
    }
}
