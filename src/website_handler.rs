use super::http::{Request, Response, StatusCode};
use super::server::Handler;
use crate::http::Method;
pub struct WebsiteHandler;
impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("this is main page".to_string())),
                "/hello" => Response::new(
                    StatusCode::Ok,
                    Some("<h1>this is a hello page</h1>".to_string()),
                ),
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
