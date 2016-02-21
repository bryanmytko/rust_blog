use models::post;
use views::layouts;

pub fn index() -> String {
  let mut buffer = String::new();

  html!(buffer, {
      h1 { "Posts" }
      p { "All posts will go here." }
  }).unwrap();

  layouts::default(buffer)
}

pub fn show(post: post::Post) -> String {
  let mut buffer = String::new();

  html!(buffer, {
      h1 { ^post.title }
      em { ^post.date }
      p  { ^post.content }
  }).unwrap();

  layouts::default(buffer)
}
