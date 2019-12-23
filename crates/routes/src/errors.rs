use rocket::request::{Outcome};
use rocket::Request;
use rocket::State;

use crate::resources::*;

macro_rules! body {
    () => {
        "<html>
            <head>
            <style>
                .wrapper {{
                    max-width: 500px;
                    margin-left:auto;
                    margin-right:auto;
                }}
            </style>
            </head>
            <body>
                <div class=\"wrapper\">
                    <h1>{}: Part Two - Electric Boogaloo</h1>
                    <p>
                        Wooooo boy! The handler couldn't
                        even load the regular html page. This
                        is the backup hardcoded response.
                        <br/><br/>
                        WHAT DID YOU DO?!
                        <br/><br/>
                        Just kidding- please contact me (Michael
                        House) and tell me that everything is 
                        broken so I can try to fix it.
                    </p>
                </div>
            </body>
        </html>"
    }
}

#[catch(404)]
pub fn handler_404(request: &Request) -> Page {
    match request.guard::<State<Resources>>() {
        Outcome::Success(r) => Page::render(&r,"/errors/404",false),
        _ => Page::plain(&format!(body!(),"404")) 
    }
}

#[catch(500)]
pub fn handler_500(request: &Request) -> Page {
    match request.guard::<State<Resources>>() {
        Outcome::Success(r) => Page::render(&r,"/errors/500",false),
        _ => Page::plain(&format!(body!(),"500")) 
    }
}

// @todo Add more error handlers
// @body Needs at least a 500 page.