use rocket::request::{Form};

#[derive(Debug)]
pub enum Action {
    Unknown,
    Create(String),
    Delete(i32),
    Active(i32)
}

#[derive(FromForm,Debug)]
pub struct ViewForm {
    pub value:  String,
    pub action: String
}

#[derive(FromForm)]
pub struct LoginForm {
    pub username: String,
    pub password: String
}

#[derive(FromForm)]
pub struct RegisterForm {
    pub username:  String,
    pub password1: String,
    pub password2: String
}

impl From<Form<ViewForm>> for Action {
    fn from(form: Form<ViewForm>) -> Self {
        match (form.action.as_str(),form.value.clone()) {
            ("create",v) => Action::Create(v),
            ("delete",v) => {
                match v.parse::<i32>() {
                    Ok(i) => Action::Delete(i),
                    _ => Action::Unknown
                }
            },
            ("active",v) => {
                match v.parse::<i32>() {
                    Ok(i) => Action::Active(i),
                    _ => Action::Unknown
                }
            },
            _ => Action::Unknown
        }
    }
}

