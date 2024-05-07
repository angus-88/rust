use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(5) {
        let stream = stream.unwrap();

        println!("Connection established!");

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("Request: {:#?}", request_line);

    let response: String;
    let (status, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "public/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "public/hello.html")
        }
        _ => ("HTTP/1.1 404 Not Found", "public/404.html"),
    };

    let body = fs::read_to_string(filename).unwrap();
    let body_len = body.len();

    response = format!("{status}\r\nContent-Length: {body_len}\r\n\r\n{body}");

    // println!("Response: {:#?}", response);
    stream.write_all(response.as_bytes()).unwrap();
}
