use hyper::method::Method;
use hyper::server::{Server, Request, Response, Listening};
use hyper::status::StatusCode;
use hyper::uri::RequestUri;

pub fn listening(url: &str) {
   let listener = Server::http(&url).unwrap();
   listener.handle(handle_proc).unwrap();
}

fn handle_proc(req: Request, mut res: Response) {
    let mut s_buf:String;
    match (req.method, req.uri) {
        (Method::Get, RequestUri::AbsolutePath(ref s)) => {
            s_buf = format!("{}", s);
        }
        (method, uri) => {
            trace!("Invalid request: {} {}", method, uri);
            s_buf = String::from("Not found!");
            *res.status_mut() = StatusCode::NotFound;
        }
    }
    let buf = s_buf.as_bytes();
    res.send(buf).unwrap();
}
