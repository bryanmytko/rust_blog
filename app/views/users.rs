use models::user;
use views::layouts;
use maud::PreEscaped;

pub fn new() -> String {
  let mut buffer = String::new();

  html!(buffer, {
      h2 { "Make a new user?" }
  }).unwrap();

  layouts::default(buffer)
}
