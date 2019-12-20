use std::{
    path::{Path},
    collections::{HashMap}
};

use rsass::{
    OutputStyle, 
    compile_scss_file
};

use walkdir::WalkDir;

#[derive(Debug)]
pub struct Styles {
    root:    String,
    layouts: HashMap<String,String>,
    themes:  HashMap<String,String>
}

impl Styles {

    pub fn from( t_root: &str ) -> Self {
        let theme_path  = format!("{}/themes",t_root);
        let layout_path = format!("{}/layouts",t_root);
        Self {
            root:    t_root.to_string(),
            layouts: Styles::compile(&layout_path),
            themes:  Styles::compile(&theme_path)
        }
    }

    pub fn layout( &self, t_name: &str ) -> String {
        match self.layouts.get(t_name) {
            Some(v) => v.chars().filter(|c| !c.is_whitespace()).collect(),
            None    => String::new()
        }
    }

    pub fn theme( &self, t_name: &str ) -> String {
        match self.themes.get(t_name) {
            Some(v) => v.chars().filter(|c| !c.is_whitespace()).collect(),
            None    => String::new()
        }
    }

    pub fn sheet( &self, t_theme: &str, t_layout: &str ) -> String {
        format!("<style>{}{}</style>",
            self.theme(t_theme),
            self.layout(t_layout))
    }

    fn compile( t_root: &str ) -> HashMap<String,String> {
        let mut files = HashMap::new();
        for record in WalkDir::new(t_root) {
            if let Ok(entry) = record {
                let path = entry.path();
                let key  = entry.path()
                                .strip_prefix(t_root)
                                .unwrap_or(Path::new(""))
                                .to_str()
                                .unwrap_or("")
                                .replace('\\',"/");
                if key.ends_with(".scss") {
                    if let Ok(compiled) = Styles::build(path) {
                        files.insert(key.replace(".scss",""),compiled);
                    }
                }
            }
        }
        files
    }

    fn build( t_path: &Path ) -> Result<String,()> {
        compile_scss_file(t_path,OutputStyle::Compressed)
            .or_else(|_| Err(()))
            .and_then(|c|{
                String::from_utf8(c)
                .or_else(|_| Err(()))
            })
    }

}