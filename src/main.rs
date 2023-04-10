#[macro_use]
extern crate rocket;

use rocket::fs::TempFile;
use rocket::serde::json::Json;
use std::env;
use std::path::Path;

use rocket::data::{Limits, ToByteUnit};
use rocket::form::Form;
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};

use backend::util::*;
use backend::auth::*;

use frontend::webpages::*;

use backend::database::*;

mod backend;
mod frontend;

const DEFAULT_USERNAME: &str = "Admin";

#[derive(FromForm)]
pub struct Upload<'r> {
    apikey: String,
    img: TempFile<'r>,
    extension: String,
}

#[derive(FromForm)]
struct NewUser {
    apikey: String,
    username: String,
    admin: bool,
}

#[derive(FromForm)]
struct Key {
    apikey: String,
}

#[derive(Serialize, Deserialize)]
struct Stats {
    bytes: u64,
    users: usize,
    files: u8,
    newest: usize,
}

#[derive(FromForm)]
struct Delete {
    apikey: String,
    filename: String,
}

#[post("/", data = "<request>")]
async fn login(request: Form<Key>) -> Result<Json<UserData>, Status> {
    if Auth::key_check(request.apikey.to_string()).await {
        Ok(User::get_user_data(request.apikey.to_string()))
    } else {
        Err(Status::NotAcceptable)
    }
}

#[post("/img", data = "<request>")]
async fn img(request: Form<Key>) -> Result<String, Status> {
    if Auth::key_check(request.apikey.to_string()).await {
        Ok(Img::get_images_from_key(request.apikey.to_string()).await)
    } else {
        Err(Status::NotAcceptable)
    }
}

#[post("/upload", data = "<image>")]
async fn upload(image: Form<Upload<'_>>) -> Status {
    let valid_extensions = vec!["png", "gif", "jpg", "jpeg", "mp4"];
    if Auth::key_check(image.apikey.to_string()).await
        && valid_extensions.contains(&image.extension.as_str())
    {
        let key = image.apikey.to_string();
        Img::new(image, User::get_uid_from_key(key)).await;
        Status::Accepted
    } else {
        Status::BadRequest
    }
}

#[delete("/delete", data = "<request>")]
async fn del(request: Form<Delete>) -> Status {
    if Auth::key_check(request.apikey.to_string()).await {
        Img::delete(
            User::get_uid_from_key(request.apikey.to_string()),
            request.filename.clone(),
        )
        .await;
        Status::Accepted
    } else {
        Status::BadRequest
    }
}
#[delete("/nuke", data = "<request>")]
async fn nuke(request: Form<Key>) -> Status {
    if Auth::key_check(request.apikey.to_string()).await {
        User::nuke(request.apikey.to_string()).await;
        Status::Accepted
    } else {
        Status::BadRequest
    }
}

#[post("/new", data = "<request>")]
async fn new(request: Form<NewUser>) -> Status {
    if Auth::key_check(request.apikey.to_string()).await
        && Auth::is_admin_check(request.apikey.to_string()).await
    {
        User::new(&request.username, request.admin);
        Status::Accepted
    } else {
        Status::BadRequest
    }
}

#[get("/stats")]
async fn stats() -> Json<Stats> {
    Json(Stats {
        bytes: dir_size("./img"),
        users: user_count(),
        files: count_dir("./img"),
        newest: last_upload(),
    })
}

#[launch]
fn rocket() -> _ {
    //first time setup if users.db doesn't exist
    if !Path::new("./users.db").is_file() {
        generate_salt_string();
        make_db().unwrap();
        User::new(DEFAULT_USERNAME, true);
    }

    if !Path::new("./img").exists() {
        match std::fs::create_dir("./img"){
            Ok(_) => println!("image directory created"),
            Err(_) => println!("image directory fucked up. probably permissions"),
        }
    }
    
    if !Path::new("./tmp").exists() {
        match std::fs::create_dir("./tmp"){
            Ok(_) => println!("tmp directory created"),
            Err(_) => println!("tmp directory fucked up. probably permissions"),
        }    }


    let figment = rocket::Config::figment()
        .merge((
            "temp_dir",
            env::current_dir().unwrap().to_str().unwrap().to_owned() + "/tmp",
        ))
        .merge(("address", "127.0.0.1"))
        .merge(("port", 8080))
        .merge(("limits", Limits::new().limit("file", 100_i32.mebibytes())))
        .merge((
            "limits",
            Limits::new().limit("data-form", 100_i32.mebibytes()),
        ));

    rocket::custom(figment)
        .mount(
            "/",
            routes![index, indexcss, indexjs, dashboard, dashcss, dashjs],
        )
        .mount("/", FileServer::from("img/"))
        .mount("/api", routes![upload, stats, img, del, nuke])
        .mount("/api/usr", routes![new, login])
}
