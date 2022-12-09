use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_conn(stream);
    }
}

fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req_line = buf_reader.lines().next().unwrap().unwrap();

    if req_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("index.html").unwrap();
        let resp = render_html_resp(status_line, &contents);

        stream.write_all(resp.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let resp = render_html_resp(status_line, &contents);
        stream.write_all(resp.as_bytes()).unwrap();
    }
}

fn render_html_resp(status_line: &str, contents: &str) -> String {
    let content_length = contents.len();
    format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, content_length, contents
    )
}
