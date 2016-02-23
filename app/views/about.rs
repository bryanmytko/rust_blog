use views::layouts;

pub fn index() -> String {
  let mut buffer = String::new();

  html!(buffer, {
      h2 { "About" }
      p { "Obligatory page that describes myself." }
  }).unwrap();

  layouts::default(buffer)
}

