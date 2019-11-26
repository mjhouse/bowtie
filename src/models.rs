
#[derive(Queryable,Debug)]
pub struct UserModel {
    pub id:       i32,
    pub email:    Option<String>,
    pub username: String,
    pub passhash: String
}

#[derive(Queryable,Debug)]
pub struct PostModel {
    pub id:        i32,
    pub title:     String,
    pub body:      String,
    pub published: bool
}