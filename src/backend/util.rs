use rand::{distributions::Alphanumeric, Rng};
use rusqlite::Connection;
use sha2::{Digest, Sha512};
use std::fs::{self, read_dir};

pub fn random_string(length: usize) -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    return s;
}

pub fn count_dir(path: &str) -> u8 {
    let paths = read_dir(path).unwrap();
    return paths.count().try_into().unwrap();
}

pub fn dir_size(path: &str) -> u64 {
    let paths = fs::read_dir(path).unwrap();
    let mut size = 0;
    for path in paths {
        size += path.unwrap().metadata().unwrap().len();
    }
    return size;
}

pub fn last_upload() -> usize {
    let imgdir = fs::metadata("./img").unwrap();
    let now_str = format!("{:?}", imgdir.modified().unwrap());
    let now_str_digits_spaces: String = now_str
        .chars()
        .filter(|c| c.is_digit(10) || *c == ',')
        .collect();
    let now_splitted: Vec<&str> = now_str_digits_spaces.split(",").collect();
    let tv_sec: usize = now_splitted[0].parse().unwrap();
    return tv_sec;
}

pub fn user_count() -> usize {
    let conn = Connection::open("./users.db").expect("db fucked");
    let mut stmt = conn.prepare(&("SELECT ALL uid FROM Users")).unwrap();
    let mut rows = stmt.query([]).unwrap();
    let mut names: Vec<String> = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        names.push(row.get(0).unwrap_or(1.to_string()));
    }

    return names.len();
}

pub fn generate_salt_string() {
    fs::write("./salt.txt", random_string(40)).unwrap();
}

pub fn hash_key(key: String) -> String {
    let mut hasher = Sha512::new();
    let salt = fs::read_to_string("./salt.txt").expect("Something went wrong reading the file");
    let key = key + &salt;
    hasher.update(key.as_bytes());
    let result = hasher.finalize();
    let hash_string = format!("{:x}", result);
    println!("{}", hash_string);
    return hash_string;
}
