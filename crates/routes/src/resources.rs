use std::fs;
use std::path::{PathBuf,Path};
use std::collections::HashMap;
use walkdir::{WalkDir};
use tera::{Tera,Context};
use chrono::{Utc};

use rsass::{
    OutputStyle, 
    compile_scss_file
};

use serde::Serialize;

use std::io::Cursor;
use rocket::{
    request::{FromRequest,Request,Outcome,FlashMessage},
    response::{self,Response, Responder},
    http::{ContentType,Status}
};

use bowtie_models::session::*;

macro_rules! files {
    ( $r:expr, $f:expr ) => {
        WalkDir::new($r)
            .into_iter()                        // iterate over all paths
            .filter(|f| f.is_ok())              // filter out Err entries
            .map(Result::unwrap)                // unwrap the Ok values
            .filter(|d| d.file_type()           // filter out non-files
                        .is_file())
            .map(|p| p.path().to_path_buf())
            .filter(|s| !s.ends_with($f))       // filter non-scss files
            .collect::<Vec<PathBuf>>()          // collect into a Vec        
    }
}

macro_rules! key {
    ( $p:ident, $r:expr ) => {
        $p.strip_prefix($r)
            .and_then(|p| Ok(p.with_extension("")))
            .and_then(|b|
                Ok(b.to_string_lossy()
                .to_string()
                .replace('\\',"/"))
            )
    }
}

#[derive(Debug,Serialize)]
pub enum PageType {
    Real,
    None,
    Plain(String)
}

#[derive(Debug,Serialize)]
pub struct Page {

    // the name of the page file (e.g. 'index.html')
    name:  String,

    // combined/compiled and minified css 
    // for this page
    styles: String,

    // combined scripts for this page
    scripts: String,

    // page requires an active login
    secure: bool,

    // a tera instance containing templated html
    #[serde(skip_serializing)]
    resources: Option<tera::Tera>,

    // additional endpoint-specific context
    #[serde(skip_serializing)]
    context: Option<Context>,

    #[serde(skip_serializing)]
    response: PageType

}

#[derive(Debug,Clone)]
pub struct ResourceFile {
    name: String,
    path: PathBuf,
    body: String
}

#[derive(Debug)]
pub struct Resources {

    // the root path (i.e. '<root>/html' or '<root>/css')
    root: String,

    // enable to turn on live reload
    pub live: bool,

    // the chained Tera instance containing templated
    // html files from 'root/html'
    pub html: Option<Tera>,

    // compiled sass where the key is the path from 
    // 'root/css' and value is the compiled content.
    pub css:  HashMap<String,ResourceFile>,

    // minified js where the key is the path from
    // 'root/js' and the value is the minified content.
    pub js:  HashMap<String,ResourceFile>,
}

impl ResourceFile {

    pub fn empty() -> Self {
        Self {
            name: String::new(),
            path: PathBuf::new(),
            body: String::new()
        }
    }

}

impl Resources {

    pub fn new( t_root: &str, t_live: bool ) -> Self {
        Resources {
            root: t_root.to_string(),
            live: t_live,
            html: None,
            css:  HashMap::new(),
            js:   HashMap::new()
        }        
    }

    pub fn from( t_root: &str, t_live: bool ) -> Self {
        let mut resources = Resources::new(t_root,t_live);
        resources.compile();
        resources
    }

    pub fn compile( &mut self ) {
        self.compile_html();
        self.compile_css();
        self.compile_js();
    }

    fn compile_css_file( &self, t_root: &Path, t_path: &Path ) -> Option<ResourceFile> {
        match compile_scss_file(&t_path, OutputStyle::Compressed) {
            Ok(c) =>  {
                match (String::from_utf8(c),key!(t_path,t_root)) {
                    (Ok(s),Ok(name)) => {
                        Some(ResourceFile {
                            name: name,
                            path: t_path.to_path_buf(),
                            body: s
                        })
                    },
                    (Err(e),_) => {
                        dbg!(e);
                        None
                    },
                    _ => None
                }
            },
            Err(e) => {
                dbg!(e);
                None
            }
        }
    }

    fn compile_js_file( &self, t_root: &Path, t_path: &Path ) -> Option<ResourceFile> {
        match (fs::read_to_string(t_path),key!(t_path,t_root)) {
            (Ok(s),Ok(name)) => {
                Some(ResourceFile {
                    name: name,
                    path: t_path.to_path_buf(),
                    body: s
                })                
            },
            (Err(e),_) => {
                dbg!(e);
                None
            },
            _ => None
        }
    }

    pub fn compile_css( &mut self ) {
        let css_root = Path::new(&self.root).join("css");
        if css_root.exists() {
            for path in files!(&css_root,".scss") {
                if let Some(file) = self.compile_css_file(&css_root,&path) {
                    self.css.insert(file.name.clone(),file);
                }
            }
        }
    }

    pub fn compile_js( &mut self ) {
        let js_root = Path::new(&self.root).join("js");
        if js_root.exists() {
            for path in files!(&js_root,".js") {
                if let Some(file) = self.compile_js_file(&js_root,&path){
                    self.js.insert(file.name.clone(),file);
                }
            }
        }
    }

    pub fn compile_html( &mut self ) {
        let html_root = Path::new(&self.root).join("html/**/*");
        if let Some(root) = html_root.to_str(){
            match Tera::new(root) {
                Ok(t) => {
                    self.html = Some(t)
                },
                Err(e) => {
                    println!("Error parsing template: {}",e);
                    ::std::process::exit(1);
                }
            }
        }
    }

    pub fn for_page( &self, t_name: &str ) 
        -> Result<(tera::Tera,Option<ResourceFile>,Option<ResourceFile>),()> {
        match self.html.as_ref() {
            Some(html) => {
                if self.live {
                    let mut layout = html.clone();
                    if let Err(e) = layout.full_reload() {
                        dbg!(e);
                    }

                    let mut css = self.css.get(t_name).cloned();
                    let mut js  = self.js.get(t_name).cloned();

                    css = match css {
                        Some(r) => self.compile_css_file(Path::new(&self.root),&r.path),
                        None => None
                    };

                    js = match js {
                        Some(r) => self.compile_js_file(Path::new(&self.root),&r.path),
                        None => None
                    };

                    Ok(( layout, css, js ))
                }
                else {
                    Ok(( html.clone(),
                        self.css.get(t_name).cloned(),
                        self.js.get(t_name).cloned() ))
                }
            },
            None => Err(())
        }
    }

}

impl Page {

    pub fn render( t_resources: &Resources, t_name: &str, t_secure: bool ) -> Page {
        let name = t_name.trim_start_matches("/").to_string();
        match t_resources.for_page(&name) {
            Ok((html,css,js)) => {
                Page::real(name,css,js,html,t_secure)
            },
            _ => Page::none()
        }
    }

    pub fn real( t_name:   String, 
                 t_style:  Option<ResourceFile>, 
                 t_script: Option<ResourceFile>, 
                 t_tera:   tera::Tera,
                 t_secure: bool ) -> Self {
        let style  = t_style.unwrap_or(ResourceFile::empty());
        let script = t_script.unwrap_or(ResourceFile::empty());

        Self {
            name:      t_name,
            styles:    style.body,
            scripts:   script.body,
            secure:    t_secure,
            resources: Some(t_tera),
            context:   None,
            response:  PageType::Real
        }
    }

    // this constructor will trigger a 404 when
    // returned from a handler.
    pub fn none() -> Self {
        Self {
            name:      String::new(),
            styles:    String::new(),
            scripts:   String::new(),
            secure:    true,
            resources: None,
            context:   None,
            response:  PageType::None
        }
    }

    // this constructor will render plain text
    // when returned.
    pub fn plain( t_body: &str ) -> Self {
        Self {
            name:      String::new(),
            styles:    String::new(),
            scripts:   String::new(),
            secure:    true,
            resources: None,
            context:   None,
            response:  PageType::Plain(t_body.into())
        }        
    }

    pub fn with_context( mut self, t_context: Context ) -> Self {
        self.context = Some(t_context);
        self
    }

}

impl Responder<'_> for Page {
    fn respond_to(self, t_request: &Request) -> response::Result<'static> {
        match self.response {
            PageType::Real => {

                let resources = match self.resources {
                    Some(r) => r,
                    None => Err(Status::NotFound)?
                };
        
                let session = match Session::from_request(t_request) {
                    Outcome::Success(s) => Some(s),
                    _ if self.secure => Err(Status::NotFound)?,
                    _ => None
                };
        
                let message = match FlashMessage::from_request(t_request) {
                    Outcome::Success(s) => Some(s),
                    _ => None
                };
        
                let date = Utc::now().format("%Y").to_string();
        
                let mut route = t_request.uri().path().to_string();
                if let Some(q) = t_request.uri().query() {
                    route = format!("{}?{}",route,q);
                }
        
                let mut context = context!(
                    "name"     => self.name,
                    "styles"   => self.styles,
                    "scripts"  => self.scripts,
                    "route"    => route,
                    "session"  => session,
                    "message"  => unflash!(message),
                    "datetime" => date
                );
        
                match self.context {
                    Some(c) => context.extend(c),
                    _ => ()
                };
        
                match resources.render(&format!("{}.html",self.name),&context) {
                    Ok(l) => {
                        Response::build()
                            .status(Status::Ok)
                            .header(ContentType::HTML)
                            .sized_body(Cursor::new(l))
                            .ok()
                    }
                    Err(e) => {
                        dbg!(e);
                        Err(Status::NotFound)?}
                }                
            },
            PageType::Plain(b) => {
                Response::build()
                    .status(Status::Ok)
                    .header(ContentType::HTML)
                    .sized_body(Cursor::new(b))
                    .ok()
            },
            PageType::None => Err(Status::NotFound)?
        }
    }
}