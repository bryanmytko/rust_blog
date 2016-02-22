use db;
use std::fmt;

#[derive(RustcEncodable, RustcDecodable)]
pub struct Post {
    id: i64,
    pub author: String,
    pub date: String,
    pub title: String,
    pub content: String
}

impl fmt::Display for Post {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Post {
    pub fn find(id: i64) -> Post {
        let conn = db::connection();
        conn.query_row("SELECT * FROM post WHERE id=1", &[], |row| {
            Post {
                id: row.get(0),
                author: row.get(1),
                date: row.get(2),
                content: row.get(3),
                title: row.get(4)
            }
        }).unwrap()
    }

    pub fn all() -> Post {
        // let mut person_iter = stmt.query_map(&[], |row| {
        //     Person {
        //         id: row.get(0),
        //         name: row.get(1),
        //         time_created: row.get(2),
        //         data: row.get(3)
        //     }
        // }).unwrap();

      let conn = db::connection();
      let posts = conn.prepare("SELECT * FROM post").unwrap();
      Post { id: 1, author: "a".to_string(), date: "a".to_string(), title: "asdf".to_string(), content: "asdf".to_string() }

    }
}
