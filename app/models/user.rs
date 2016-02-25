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

    pub fn create(user: User) -> i32 {
        let conn = db::connection();

        /* @TODO remove: Dummy Data */
        let email = "bryan@test.com";
        let first_name = "bryan";
        let last_name = "mytko";
        let password_digest = "32r90aweufija4fkljasd;f";
        let admin = false;

        let result = conn.execute("INSERT INTO user (email, first_name, last_name, password_digest, admin) VALUES ($1, $2, $3, $4, $5)", &[&email, &first_name, &last_name, &password_digest, &admin]);
        result.unwrap()
    }
}
