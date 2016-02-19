use models::post;
use std::fmt;
use std::io;

pub fn index() -> String {
  let mut buffer = String::new();

  html!(buffer, {
      p { "Hello Universe" }
  }).unwrap();

  buffer
}

pub fn show(post: post::Post) -> String {
  let mut buffer = String::new();
  let author = "Bryan"; //post.author;

  html!(buffer, {
     // h1 { "Hello !" $author }
      p { "test" }
  }).unwrap();

  buffer
}
