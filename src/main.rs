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

fn get_code_snippets(url: &str) {
    let full_url = "http://www.rosettacode.org".to_owned() + url;
    let mut resp = reqwest::get(full_url.as_str()).unwrap();

    let html = resp.text().unwrap();

    let doc = Html::parse_document(&html);
    let selector = Selector::parse(".ruby.highlighted_source").unwrap();
    // TODO: This gets code for "Crystal" as well...

    for node in doc.select(&selector) {
        let text = node.text().collect::<Vec<_>>().join("");
        println!("ruby code: {:?}", text);
    }
}

fn main() {
    for task in get_task_names() {
        get_code_snippets(&task.href);
        //println!("{}", );
    }
    //println!("task names: {:?}", get_task_names());
}
