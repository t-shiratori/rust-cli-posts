use colour::{dark_green, yellow};
use serde::Deserialize; 
use std::error::Error;

#[derive(Deserialize)]

struct Post {
    title: String,
    body: String,
}

type Posts = Vec<Post>;

fn get_posts(url: &str) -> Result<Posts, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;
    let posts = serde_json::from_str(&response)?; // Vec<Post>型のデータ構造にデシリアライズ
    Ok(posts)
}

fn render_posts(posts: &Posts) {
    for post in posts {
        dark_green!("[Title]: {}\n", post.title);
        yellow!("[Body]: {}\n", post.body);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let url: &str = "https://jsonplaceholder.typicode.com/posts";
    let posts_result = get_posts(url)?;
    render_posts(&posts_result);
    Ok(())
}
