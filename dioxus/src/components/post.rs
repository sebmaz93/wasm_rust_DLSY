#![allow(non_snake_case)]
// use dioxus::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub url: Option<String>,
    pub text: Option<String>,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub score: i64,
    #[serde(default)]
    pub descendants: i64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub date: DateTime<Utc>,
    #[serde(default)]
    pub kids: Vec<i64>,
    pub r#type: String,
}

// pub fn PostItem(cx: Scope) -> Element {
//     let title = "title";
//     let author = "author";
//     let mut likes = use_state(cx, || 0);
//     let date = chrono::Utc::now();
//     let comments = "comments";
//
//     render! {
//         div {
//             padding: "16px",
//             "{title} by {author} ({likes}) {date} {comments}"
//         }
//         button {onclick: move |_| likes += 1, "Up!"}
//         button {onclick: move |_| likes -= 1, "Down!"}
//     }
// }
