use crate::http::StatusCode;
use crate::http::{Method, Request, Response};
use crate::server::Handler;
use std::fs;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if !path.starts_with(&self.public_path) {
                    return None;
                }
                fs::read_to_string(path).ok()
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/search" => Response::new(StatusCode::Ok, self.read_file("search.html")),
                path => match self.read_file(path) {
                    Some(body) => Response::new(StatusCode::Ok, Some(body)),
                    _ => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
