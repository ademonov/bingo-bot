use hyper::server::{Server, Request, Response};
//use hyper::server::Listening;

pub fn listening(url: &str) {
   let listener = Server::http(&url).unwrap();
   listener.handle(handle_proc).unwrap();
}

fn handle_proc(_: Request, res: Response) {
    let s_buf = String::from("Helo, World!");
    let buf = s_buf.as_bytes();
    res.send(buf).unwrap();
}
