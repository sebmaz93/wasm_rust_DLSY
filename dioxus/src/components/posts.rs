#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::comment::Comment;
use crate::components::post::Post;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostPageData {
    #[serde(flatten)]
    pub item: Post,
    #[serde(default)]
    pub comments: Vec<Comment>,
}

#[inline_props]
pub fn Posts(cx: Scope, _post: Post) -> Element {
    let Post {
        title,
        author,
        score,
        date,
        kids,
        ..
    } = _post;

    let score = format!("{score} {}", if *score == 1 { " point" } else { " points" });

    let comments = format!(
        "{} {}",
        kids.len(),
        if kids.len() == 1 {
            " comment"
        } else {
            " comments"
        }
    );

    let time = date.format("%D %l:%M %p");

    render! {
    div {
        padding: "0.5rem",
        position: "relative",
        div {
            font_size: "1.5rem",
            a {
                href: "#",
                "{title}"
            }
            a {
                color: "gray",
                href: "https://news.ycombinator.com/from?site=#",
                text_decoration: "none",
                " (hostname)"
            }
        }
        div {
            display: "flex",
            flex_direction: "row",
            color: "gray",
            div {
                "{score}"
            }
            div {
                padding_left: "0.5rem",
                "by {author}"
            }
            div {
                padding_left: "0.5rem",
                "{date}"
            }
            div {
                padding_left: "0.5rem",
                "{comments}"
            }
        }
    }
    }
}
