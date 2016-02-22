use models::post;
use views::layouts;
use maud::PreEscaped;

pub fn index(posts: Vec<post::Post>) -> String {
  let mut buffer = String::new();

  html!(buffer, {
      h2 { "Posts" }
      ul {
          @for post in posts {
              li {
                  ^PreEscaped(format!("<a href=\"/posts/{}\">{}</a>", post.id, post.title))
              }
          }
      }
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
