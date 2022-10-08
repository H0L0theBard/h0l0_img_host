use rusqlite::Connection;

use super::util::hash_key;

pub struct Auth{}

impl Auth{
    pub async fn key_check(key: String) -> bool {
        let conn = Connection::open("./users.db").expect("db fucked");
        if key.is_empty() == false {
            let mut stmt = conn
                .prepare(
                    &(("SELECT ALL APIKey FROM Users WHERE APIKey =").to_owned()
                        + "'"
                        + &hash_key(key.clone())
                        + "'"),
                )
                .unwrap();
            let mut rows = stmt.query([]).unwrap();
            let mut names: Vec<String> = Vec::new();
            while let Some(row) = rows.next().unwrap() {
                names.push(row.get(0).unwrap());
            }
            if names.contains(&hash_key(key)) {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    
    pub async fn is_admin_check(key: String) -> bool {
        let conn = Connection::open("./users.db").expect("db fucked");
        if key.is_empty() == false {
            let mut stmt = conn
                .prepare(&("SELECT ALL ApiKey FROM Users WHERE isAdmin = 0"))
                .unwrap();
            let mut rows = stmt.query([]).unwrap();
            let mut names: Vec<String> = Vec::new();
            while let Some(row) = rows.next().unwrap() {
                names.push(row.get(0).unwrap());
            }
            if names.contains(&hash_key(key)) {
                return false;
            } else {
                return true;
            }
        } else {
            return false;
        }
    }
    
}
