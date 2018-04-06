extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Class;

fn main() {
    let resp = reqwest::get("http://www.rosettacode.org/wiki/Narcissist").unwrap();

    let doc = Document::from_read(resp)
        .unwrap();

    let nodes = doc.find(Class("c"))
        .into_selection()
        .filter(Class("highlighted_source"));

    println!("Received: {}", nodes.len());
    println!("Content: {}", nodes.first().unwrap().text())
}
