use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use rust_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_conn(stream);
        });
    }
}

fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, html_file) = match &req_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        // simulate slow request processing so we can play with
        // the single-threaded server a bit.
        "GET /sleep HTTP/1.1" => {
            println!("slow request started...");
            thread::sleep(Duration::from_secs(5));
            println!("slow request done...");
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 400 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(html_file).unwrap();
    let resp = render_html_resp(status_line, &contents);
    stream.write_all(resp.as_bytes()).unwrap();
}

fn render_html_resp(status_line: &str, contents: &str) -> String {
    let content_length = contents.len();
    format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, content_length, contents
    )
}
