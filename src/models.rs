
#[derive(Queryable,Debug)]
pub struct UserModel {
    pub id: i32,
    pub email:    Option<String>,
    pub username: String,
    pub passhash: String
}