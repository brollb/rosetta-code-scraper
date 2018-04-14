extern crate reqwest;
extern crate scraper;

use scraper::{Html,Selector};

use std::vec::Vec;

struct Task {
    title: String,
    href: String
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

fn get_code_snippets(url: &str) -> Vec<String> {
    let full_url = "http://www.rosettacode.org".to_owned() + url;
    let mut resp = reqwest::get(full_url.as_str()).unwrap();

    let html = resp.text().unwrap();

    let doc = Html::parse_document(&html);
    let selector = Selector::parse(".ruby.highlighted_source").unwrap();
    // TODO: This gets code for "Crystal" as well...
    // Fi

    // Record the language of the snippet
    // TODO

    let mut snippets = Vec::new();
    for code_segment in doc.select(&selector) {
        let mut code = String::new();
        for child in code_segment.children() {
            let node = child.value();

            if node.is_element() {
                let element = node.as_element().unwrap();

                match element.name() {
                    "span" => {
                        let text_node = child.first_child().unwrap();
                        let text = text_node.value().as_text().unwrap();
                        code += text;
                    },
                    "br" => {
                        code += "\n";
                    },
                    _ => {
                        println!("element tag not supported");
                    }
                }

            } else if node.is_text() {
                let text = node.as_text().unwrap();
                code += text;
            }
        }
        snippets.push(code);
    }

    snippets
}

fn main() {
    let first_task = &get_task_names()[0];
    let snippets = get_code_snippets(&first_task.href);
    let snippet = &snippets[0];
    println!("snippet:\n{}", snippet);

    //for task in get_task_names() {
        //get_code_snippets(&task.href);
        //println!("{}", );
    //}
    //println!("task names: {:?}", get_task_names());
}
