use std::path::{Path, PathBuf};
use anyhow::{bail, Ok, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct YamlContent {
    title: String,
    author: String,
    contents: String,
}

#[derive(Debug, Default)]
pub struct Post {
    title: String,
    author: String,
    contents: String,
    filename: String,
    uri: String,
}

impl Post {
    pub fn read(path: &Path) -> Result<Self> {
        // Make sure it has a yaml file extension. May add support for other
        // formats later.
        let extension = path.extension().unwrap().to_str().unwrap();
        if extension != "yaml" {
            bail!("file does not have the .yaml extension");
        }

        // This shouldn't fail because the path is provided by the system, not user input.
        let filename = path.file_name().unwrap().to_str().unwrap().to_string();

        let mut uri = PathBuf::from(&filename);
        uri.set_extension("html");

        let uri = uri.to_str().unwrap().to_string();

        // Read the yaml file
        let content = std::fs::read_to_string(path).unwrap();

        let YamlContent {
            title,
            author,
            contents,
        } = serde_yaml::from_str(&content)?;

        // convert the md formated data in contents to html
        let options = comrak::Options {
            extension: comrak::ExtensionOptionsBuilder::default()
                .header_ids(Some(String::new()))
                .strikethrough(true)
                .footnotes(true)
                .table(true)
                .build()?,
            ..comrak::Options::default()
        };

        let contents = comrak::markdown_to_html(&contents, &options);

        Ok(Self {
            title,
            author,
            contents,
            filename,
            uri,
        })        
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn author(&self) -> &str { 
        &self.author
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn contents(&self) -> &str {
        &self.contents
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

}