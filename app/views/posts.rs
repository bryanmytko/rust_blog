use models::post;
use views::layouts;

pub fn index(posts: post::Post) -> String {
  let mut buffer = String::new();

  html!(buffer, {
      h2 { "Posts" }
      p { "All posts will go here." ^posts }
  }).unwrap();

  layouts::default(buffer)
}

pub fn show(post: post::Post) -> String {
  let mut buffer = String::new();

  html!(buffer, {
      h2 { ^post.title }
      em { ^post.date }
      p  { ^post.content }
  }).unwrap();

  layouts::default(buffer)
}
