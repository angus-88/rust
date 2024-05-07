// Run a HTTP server in one thread that listens and accepts XML POST requests and stores the successfully parsed people.
// POST /xml-to-json Returns 200 with Person in JSON format
// POST /xml-to-json Returns a 400 for malformed xml

// Main thread reads xml from file and breaks it up into individual people before calling API.

use std::thread;

mod server;

fn main() {
    let server_thread = thread::spawn(start);

    println!("Hello, world!");

    server_thread.join().unwrap();
}

#[tokio::main]
async fn start() {
    server::start_server().await;
    // .expect("Server was unable to start");
}
