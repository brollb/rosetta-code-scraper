extern crate reqwest;
extern crate scraper;

use scraper::{Html,Selector};

use std::vec::Vec;
use std::fs;
use std::str;
use std::io::prelude::*;

struct Task {
    title: String,
    href: String
}

struct CodeSnippet {
    task: String,
    lang: String,
    code: String
}

fn get_task_names() -> Vec<Task>{
   let task_index_url = "http://www.rosettacode.org/wiki/Category:Programming_Tasks";
   let mut resp = reqwest::get(task_index_url).unwrap();

   let doc = Html::parse_document(&resp.text().unwrap());
   let selector = Selector::parse(".mw-category-group a").unwrap();

   doc.select(&selector).map(|node| {
       let title = node.value().attr("title").unwrap();
       let href = node.value().attr("href").unwrap();
       Task{title: String::from(title), href: String::from(href)}
   }).collect()
}

fn get_code_snippets(task: &Task) -> Vec<CodeSnippet> {
    let full_url = "http://www.rosettacode.org".to_owned() + &task.href;
    let mut resp = reqwest::get(full_url.as_str()).unwrap();

    let html = resp.text().unwrap();

    let doc = Html::parse_document(&html);
    let selector = Selector::parse(".highlighted_source").unwrap();

    let mut snippets = Vec::new();
    eprintln!("found {} matches", doc.select(&selector).count());
    for code_segment in doc.select(&selector) {
        // Look up the language for the given snippet
        if let Some(title) = find_preceding_title(code_segment) {
            eprintln!("Found title: {}", title);
            if let Some(code) = parse_code_snippet(code_segment) {
                let snippet = CodeSnippet{
                        task: task.title.clone(),
                        lang: title.to_string(),
                        code
                };
                snippets.push(snippet);
            } else {
                eprintln!("Skipping {} (nested span)", title);
            }
        }
    }

    eprintln!("found {} snippets", snippets.len());
    snippets
}

fn find_preceding_title(element: scraper::ElementRef) -> Option<&str> {
    let mut prev = element.prev_sibling();
    while let Some(prev_node) = prev {
        // check if it the h2 header
        if let Some(prev_element) = prev_node.value().as_element() {
            if prev_element.name() == "h2" {
                // Can I get the "span" child and the "id" attribute?
                if let Some(title_ctnr_node) = prev_node.first_child() {
                    // Check for a span child
                    // TODO
                    if let Some(title_container) = title_ctnr_node.value().as_element() {
                        return title_container.attr("id");
                    }
                }
            }
        }
        prev = prev_node.prev_sibling();
    }

    return None;
}

/// Given a element containing a code snippet, parse the code from the html
/// elements
fn parse_code_snippet(element: scraper::ElementRef) -> Option<String> {
    eprintln!("about to parse code snippet");
    let mut code = String::new();
    for child in element.children() {
        let node = child.value();

        if node.is_element() {
            let element = node.as_element().unwrap();

            match element.name() {
                "span" => {
                    let span_child_node = child.first_child().unwrap();
                    let child_node = span_child_node.value();
                    // This section kinda sucks. I wanted to refactor this into a method for
                    // getting the descendent text but couldn't figure out how to reference
                    // ego_tree::NodeRef in the method signature as it is a dependency of
                    // scraper...
                    if child_node.is_text() {
                        if let Some(text) = child_node.as_text() {
                            code += text;
                        } else {
                            eprintln!("found span with no text... weird {:?}", child_node);
                        }
                    } else {  // Ignore doubly nested spans for now
                        return None;
                    }
                },
                "br" => {
                    code += "\n";
                },
                _ => {
                    eprintln!("element tag not supported");
                }
            }

        } else if node.is_text() {
            let text = node.as_text().unwrap();
            code += text;
        }
    }

    Some(code)
}

fn main() {
    let base_data_dir = String::from("data/");
    let mut example_count = 0;

    for task in get_task_names() {
        let snippets = get_code_snippets(&task);

        println!("About to print the detected languages");
        for snippet in snippets {
            // Make a directory for each language, task
            let dir_path = base_data_dir.clone() + &snippet.lang + "/";  // + &snippet.task + "/";
            eprintln!("about to mkdir -p {}", dir_path.clone());
            fs::create_dir_all(dir_path.clone()).unwrap();

            // Write the solution for that language in that directory
            let file_path = dir_path + &str::replace(&snippet.task, "/", "-");
            eprintln!("about to write file {}", file_path.clone());
            let mut file = fs::File::create(file_path.clone()).unwrap();
            match file.write_all(snippet.code.as_bytes()) {
                Ok(_) => {
                    example_count += 1;
                    println!("created {}", file_path);
                },
                Err(err) =>  println!("could not write {}: {}", file_path, err)
            }
        }
    }
    println!("Saved {} code snippets", example_count);
}
