use http::httprequest::HttpRequest;
use std::{env, fs};
use http::httpresponse::HttpResponse;

pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;

    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/index.html", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);
        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
}

pub struct StaticPageHandler;
pub struct WebServiceHandler;
pub struct StaticFileHandler;