#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

mod components;
use components::{comment, post, posts};

fn main() {
    LaunchBuilder::new(App).launch();
}

fn App(cx: Scope) -> Element {
    // you can use cx.render( rsx! {} ) does the same thing
    render! {
       posts::Posts {
            _post: post::Post {
                id: 0,
                title: "hello Rustaceans".to_string(),
                url: None,
                text: None,
                author: "Author".to_string(),
                score: 0,
                descendants: 0,
                date: chrono::Utc::now(),
                kids: vec![],
                r#type: "".to_string(),
            }
        }
    }
}
