use db;

pub struct User {
    pub id: i64,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password_digest: String,
        admin: bool,
}

impl User {
  pub fn find(id: i64) -> User {
      let conn = db::connection();

      conn.query_row("SELECT * FROM user WHERE id=$1", &[&id], |row| {
          User {
              id: row.get(0),
              email: row.get(1),
              first_name: row.get(2),
              last_name: row.get(3),
              password_digest: row.get(4),
              admin: row.get(5),
          }
      }).unwrap()
  }

    pub fn create(params: &[String; 5]) -> i32 {
        let conn = db::connection();

        let user = User {
            id: 1,
            email: "bryan@test.com".to_string(),
            first_name: "bryan".to_string(),
            last_name: "mytko".to_string(),
            password_digest: "32r90aweufija4fkljasd,f".to_string(),
            admin: false,
        };

        let result = conn.execute("INSERT INTO user (email, first_name, last_name, password_digest, admin) VALUES ($1, $2, $3, $4, $5)", &[&params[0],&params[1],&params[2],&params[3],&params[4]]);

        result.unwrap()
    }
}
