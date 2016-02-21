use maud::PreEscaped;

pub fn default(template: String) -> String {
    let mut layout = String::new();

    html!(layout, {
        html {
            head {
                title "bryanmytko.com"
                link rel="stylesheet" href="/assets/css/style.css" /
                script src="/assets/js/main.js" {}
            }
            body {
                div class="main-container" {
                    ^PreEscaped(template)
                }
            }
        }
    }).unwrap();

    layout
}
