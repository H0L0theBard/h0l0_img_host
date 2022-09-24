use std::{fs, path::Path};

use crate::{
    backend::util::{hash_key, random_string, user_count},
    Upload,
};
use chrono::Utc;
use rocket::{
    form::Form,
    serde::json::{serde_json, Json},
};
use rusqlite::{Connection, Error};
use serde::Serialize;

pub fn make_db() -> Result<(), Error> {
    let conn = Connection::open("./users.db").expect("db fucked");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Users (
            username text not null,
            uid int not null unique,
            isAdmin bool,
            uploadCount int,
            APIKey text not null unique
         )",
        (),
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Images (
            filename text not null unique,
            uid int not null,
            timestamp int not null
         )",
        (),
    )?;
    Ok(())
}
pub struct User {
    username: String,
    uid: usize,
    is_admin: bool,
    upload_count: i64,
    apikey: String,
}

#[derive(Serialize)]
pub struct UserData {
    pub username: String,
    pub uid: usize,
    pub is_admin: usize,
    pub upload_count: usize,
    pub uploaded_bytes: u64,
    pub timestamp: usize,
}

impl User {
    pub fn new(username: &str, admin: bool) {
        let newuser = User {
            username: username.to_string(),
            uid: user_count(),
            is_admin: admin,
            upload_count: 0,
            apikey: random_string(40),
        };
        let conn = Connection::open("./users.db").expect("db fucked");
        println!("{}", newuser.apikey);
        conn.execute(
            "INSERT INTO Users (Username, uid, isAdmin ,uploadCount, APIKey) VALUES (?1, ?2, ?3, ?4,?5)",
            (newuser.username, newuser.uid, newuser.is_admin, newuser.upload_count, hash_key(newuser.apikey)),
        ).unwrap();
    }

    pub fn get_uid_from_key(key: String) -> usize {
        let mut uid = 0;
        let conn = Connection::open("./users.db").expect("db fucked");
        let mut stmt = conn
            .prepare(
                &(("SELECT uid FROM Users WHERE APIKey =").to_owned() + "'" + &hash_key(key) + "'"),
            )
            .unwrap();
        let mut rows = stmt.query([]).unwrap();
        while let Some(row) = rows.next().unwrap() {
            uid = row.get(0).unwrap();
        }
        return uid;
    }

    pub fn get_user_data(key: String) -> Json<UserData> {
        let mut username = "".to_string();
        let mut uid = 0;
        let mut is_admin: usize = 0;

        let conn = Connection::open("./users.db").expect("db fucked");
        let mut stmt = conn
            .prepare(
                &(("SELECT * FROM Users WHERE APIKey =").to_owned() + "'" + &hash_key(key) + "'"),
            )
            .unwrap();
        let mut rows = stmt.query([]).unwrap();
        while let Some(row) = rows.next().unwrap() {
            username = row.get(0).unwrap();
            uid = row.get(1).unwrap();
            is_admin = row.get(2).unwrap();
        }

        let mut stmt = conn
            .prepare(
                &(("SELECT * FROM Images WHERE uid =").to_owned() + "'" + &uid.to_string() + "'"),
            )
            .unwrap();
        let mut rows = stmt.query([]).unwrap();
        let mut images: Vec<Img> = Vec::new();
        let mut size = 0;

        while let Some(row) = rows.next().unwrap() {
            let image = Img {
                filename: row.get(0).unwrap(),
                uid: row.get(1).unwrap(),
                timestamp: row.get(2).unwrap(),
            };
            let filename: String = row.get(0).unwrap();
            size += Path::new("img/").join(&filename).metadata().unwrap().len();
            images.push(image);
        }
        let mut timestamp: usize = 0;

        if images.is_empty() != true {
            let mut stmt = conn
                .prepare(
                    &(("SELECT MAX(timestamp) FROM Images WHERE uid = ").to_owned()
                        + "'"
                        + &uid.to_string()
                        + "'"),
                )
                .unwrap();
            let mut rows = stmt.query([]).unwrap();

            while let Some(row) = rows.next().unwrap() {
                timestamp = row.get(0).unwrap();
            }
        }

        return Json(UserData {
            username: username,
            uid: uid,
            is_admin: is_admin,
            upload_count: images.len(),
            uploaded_bytes: size,
            timestamp: timestamp,
        });
    }

    pub async fn nuke(key: String) {
        let conn = Connection::open("./users.db").expect("db fucked");
        let uid = User::get_uid_from_key(key);
        let mut stmt = conn
            .prepare(
                &(("SELECT * FROM Images WHERE uid =").to_owned() + "'" + &uid.to_string() + "'"),
            )
            .unwrap();
        let mut rows = stmt.query([]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            let filename: String = row.get(0).unwrap();
            fs::remove_file(Path::new("img/").join(&filename)).unwrap();
            conn.execute(
                "DELETE FROM Images WHERE filename = ? AND uid = ?",
                (filename, uid),
            )
            .unwrap();
        }
    }
}

#[derive(Serialize)]
pub struct Img {
    filename: String,
    uid: usize,
    timestamp: i64,
}

impl Img {
    pub async fn new(mut image: Form<Upload<'_>>, uid: usize) {
        let conn = Connection::open("./users.db").expect("db fucked");

        let mut imgname = random_string(8) + "." + &image.extension;
        let mut imgpath = Path::new("img/").join(&imgname);

        while Path::new("img/").join(imgname.clone()).is_file() == true {
            imgname = random_string(8) + "." + &image.extension;
            imgpath = Path::new("img/").join(&imgname);
        }

        let newimg = Img {
            filename: imgname,
            uid: uid,
            timestamp: Utc::now().timestamp(),
        };

        conn.execute(
            "INSERT INTO Images (filename,uid,timestamp) VALUES (?1, ?2,?3)",
            (newimg.filename, newimg.uid, newimg.timestamp),
        )
        .unwrap();

        image.img.persist_to(imgpath).await.unwrap();
    }

    pub async fn delete(uid: usize, filename: String) {
        let conn = Connection::open("./users.db").expect("db fucked");
        fs::remove_file(Path::new("img/").join(&filename)).unwrap();
        conn.execute(
            "DELETE FROM Images WHERE filename = ? AND uid = ?",
            (filename, uid),
        )
        .unwrap();
    }

    pub async fn get_images_from_key(key: String) -> String {
        let conn = Connection::open("./users.db").expect("db fucked");
        let mut stmt = conn
            .prepare(
                &(("SELECT uid FROM Users WHERE APIKey =").to_owned() + "'" + &hash_key(key) + "'"),
            )
            .unwrap();
        let mut rows = stmt.query([]).unwrap();
        let mut uid: Vec<usize> = Vec::new();

        while let Some(row) = rows.next().unwrap() {
            uid.push(row.get(0).unwrap());
        }

        let user_uid = uid[0];

        let mut stmt = conn
            .prepare(
                &(("SELECT * FROM Images WHERE uid =").to_owned()
                    + "'"
                    + &user_uid.to_string()
                    + "'"),
            )
            .unwrap();
        let mut rows = stmt.query([]).unwrap();
        let mut images: Vec<Img> = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            let image = Img {
                filename: row.get(0).unwrap(),
                uid: row.get(1).unwrap(),
                timestamp: row.get(2).unwrap(),
            };
            images.push(image);
        }
        let json = serde_json::to_string(&images).unwrap();
        return json;
    }
}
