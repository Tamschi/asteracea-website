//! TODO_DOCS_DESCRIPTION
//!
//! [![Zulip Chat](https://img.shields.io/endpoint?label=chat&url=https%3A%2F%2Fiteration-square-automation.schichler.dev%2F.netlify%2Ffunctions%2Fstream_subscribers_shield%3Fstream%3Dproject%252Fasteracea-website)](https://iteration-square.schichler.dev/#narrow/stream/project.2Fasteracea-website)

#![doc(html_root_url = "https://docs.rs/asteracea-website/0.0.1")]
#![warn(clippy::pedantic)]
#![allow(clippy::semicolon_if_nothing_returned)]

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme {}

pub mod components;
pub mod pages;
pub mod site;
pub mod snippets;
pub mod templates;
