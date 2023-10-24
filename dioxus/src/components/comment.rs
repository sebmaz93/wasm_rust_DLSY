#![allow(non_snake_case)]
use chrono::{DateTime, Utc};
// use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub id: i64,
    /// there will be no by field if the comment was deleted
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub text: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub date: DateTime<Utc>,
    #[serde(default)]
    pub kids: Vec<i64>,
    #[serde(default)]
    pub sub_comments: Vec<Comment>,
    pub r#type: String,
}

// pub fn CommentItem(cx: Scope) -> Element {
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
