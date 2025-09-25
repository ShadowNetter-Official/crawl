use ureq;
use scraper::{Html, Selector};
use std::collections::HashSet;
use std::env;
use colored::*;

fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        println!("crawl | a fun web crawler written in Rust 🦀\n");
        println!("usage: \n");
        println!("crawl <url> | make sure to include https://");
        return;
    }

    let mut visited: HashSet<String> = HashSet::new();
    request(argv[1].clone(), &mut visited);
}

fn http(url: String) -> String {
    let request = match ureq::get(&url).call() {
        Ok(request) => request,
        Err(_) => return String::new(),
    };
    let response: String = match request.into_string() {
        Ok(response) => response,
        Err(_) => return String::new(),
    };
    response
}

// find all <a> elements and return vector of links

fn scrape(response: String) -> Vec<String> {
    let search: String = "a".to_string();
    let document = Html::parse_document(&response);
    let selector = match Selector::parse(&search) {
        Ok(selector) => selector,
        Err(e) => {
            println!("Error parsing reponse: {}", e);
            return vec![];
        }
    };
    let mut matches: Vec<String> = Vec::new();
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            matches.push(href.to_string());
        }
    }
    matches
}

fn request(url: String, visited: &mut HashSet<String>) {
    // colors for styling
    let dark_yellow = Color::TrueColor { r:255, g:242, b:102 };
    let bright_yellow = Color::TrueColor {r:250, g:246, b:202 };
    if visited.contains(&url) {
        // already visited, skip
        return;
    }

    visited.insert(url.clone());

    if url.chars().nth(0) == Some('/') {
        println!("  {}", url.color(bright_yellow));
    } else {
        println!("{}", url.color(dark_yellow));
    }

    let response = http(url.clone());
    if response.is_empty() {
        return;
    }

    let items: Vec<String> = scrape(response);
    if items.is_empty() {
        return;
    }

    for item in items {
        // recursively request each link
        request(item, visited);
    }
}

