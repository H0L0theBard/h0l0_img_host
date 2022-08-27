use rocket::fs::NamedFile;
use rocket::response::status::NotFound;

#[get("/")]
pub async fn index() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("web/index.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[get("/index.css")]
pub async fn indexcss() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("web/css/index.css")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[get("/index.js")]
pub async fn indexjs() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("web/js/index.js")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[get("/dashboard")]
pub async fn dashboard() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("web/dashboard.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[get("/dashboard.css")]
pub async fn dashcss() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("web/css/dashboard.css")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

#[get("/dashboard.js")]
pub async fn dashjs() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("web/js/dashboard.js")
        .await
        .map_err(|e| NotFound(e.to_string()))
}
