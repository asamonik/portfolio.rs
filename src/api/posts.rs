use leptos::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use leptos::prelude::*;

use walkdir::WalkDir;
use std::fs;   
use std::path::Path;   
use pulldown_cmark::html::*;
use pulldown_cmark::Parser;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostMetadata {
    pub date: String,
    pub title: String,
    pub tags: Option<Vec<String>>,
    pub draft: Option<bool>,
    pub summary: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub date: NaiveDate,
    pub title: String,
    pub description: String,
    pub filename: String,
}

impl Post {
    pub fn create_href(&self) -> String {
        self.title.replace(' ', "-").to_lowercase()
    }
}

pub type PostContent = String;

#[server(GetAllPosts, "/getposts", "GetJson")]
pub async fn get_all_posts(_search_string: String) -> Result<Vec<Post>, ServerFnError> {
    let mut posts = Vec::new();

    for entry in WalkDir::new("./posts").into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            println!("Reading file: {:?}", entry.path());
            let content = fs::read_to_string(entry.path()).expect("Failed to read file");
            if let Some((metadata, content)) = parse_markdown(&content) {
                let date = NaiveDate::parse_from_str(&metadata.date, "%Y-%m-%d").expect("Failed to parse date");
                posts.push(Post {
                    date,
                    title: metadata.title,
                    description: metadata.summary.unwrap_or_default(),
                    filename: entry.file_name().to_string_lossy().to_string(), 
                });
            }
        }
    }

    // Sort posts by date in descending order
    posts.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(posts)
}

// TODO: PATH TRAVERSAL VULN!!!!
#[server(GetPost, "/getpost", "GetJson")]
pub async fn get_post(filename: String) -> Result<String, ServerFnError> {
    // Sanitize the filename to prevent path traversal
    let sanitized_filename = Path::new(&filename)
        .file_name()
        .ok_or(ServerFnError::new("Invalid filename".to_string()))?
        .to_string_lossy()
        .to_string();

    let filepath = format!("./posts/{}", sanitized_filename);
    let content = fs::read_to_string(filepath).expect("Failed to read file");
    
    // parse markdown
    let (_, content) = parse_markdown(&content).ok_or(ServerFnError::new("Failed to parse markdown".to_string()))?;
    
    let parser = Parser::new(&content);
    let mut html_output = String::new();
    push_html(&mut html_output, parser);
    Ok(html_output)
}

fn parse_markdown(content: &str) -> Option<(PostMetadata, String)> {
    // Replace literal "\n" with actual newlines
    let content = content.replace("\\n", "\n");
    let mut lines = content.lines();

    let first_line = lines.next()?.trim();
    if first_line != "---" {
        println!("First line is not '---': {:?}", first_line);
        return None;
    }

    let mut frontmatter = String::new();
    for line in lines.by_ref() {
        let trimmed_line = line.trim();
        if trimmed_line == "---" {
            break;
        }
        frontmatter.push_str(trimmed_line);
        frontmatter.push('\n');
    }

    let metadata: PostMetadata = serde_yaml::from_str(&frontmatter).ok()?;
    let content = lines.collect::<Vec<&str>>().join("\n");

    Some((metadata, content))
}