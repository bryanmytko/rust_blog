use maud::PreEscaped;

pub fn default(template: String) -> String {
    let mut layout = String::new();

    html!(layout, {
        html {
            head {
                title "bryanmytko.com"
                link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Roboto+Condensed:400,700' rel='stylesheet" /
                link rel="stylesheet" href="/assets/css/style.css" /
                script src="/assets/js/main.js" {}
            }
            body {
                nav {
                    div class="container" {
                        ul {
                            li {
                                a href="/posts" "Words"
                            }

                            li {
                                a href="/about" "About"
                            }

                            li {
                                a href="/contact" "Contact"
                            }
                        }
                    }
                }
                div class="main-container" {
                    h1 { a href="/" "bryanmytko.com" }
                    div class="entry" {
                        ^PreEscaped(template)
                    }
                }
                footer {
                    div class="container" {
                        p { ^PreEscaped("&copy; 2016 <span>&hearts;</span> ")  a href="http://github.com/bryanmytko" "Bryan Mytko" }
                    }
                }
            }
        }
    }).unwrap();

    layout
}
