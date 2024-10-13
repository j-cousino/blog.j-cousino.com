//! A simple static blog website generator
//! 
//! Blog posts are written in commonmark/GFM. the files should be named
//! as follows, YYYY-MM-DD-filename.yaml.
//!  
use std::{fs::{self, create_dir}, path::PathBuf};
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppOptions {
    inputdir: PathBuf,
    outputdir: PathBuf,
}

fn main() -> Result<()>{
    let options_content = std::fs::read_to_string(PathBuf::from("./bloggen.yaml"))?;
    let options: AppOptions = serde_yaml::from_str(&options_content)?;

    let mut posts: Vec<bloggen::Post> = Vec::new();

    let entries = std::fs::read_dir(options.inputdir)?;
    for entry in entries {
        if let Ok(entry) = entry {
            if entry.file_type()?.is_file() {
                // We need to ignore an error but report it.
                let post = bloggen::Post::read(&entry.path());
                match post {
                    Ok(post) => {
                        posts.push(post);
                    }
                    Err(err) => {
                        println!("Error reading {:?}, {err}", entry.path());
                    }
                }
            }
        }
    }   

    if !options.outputdir.exists() {
        create_dir(&options.outputdir)?;
    }

    for post in posts {
        let mut filename = PathBuf::from(&options.outputdir);
        filename.push(post.uri());
        
        fs::write(filename, post.contents())?;
    }

    Ok(())
}
