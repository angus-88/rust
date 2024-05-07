mod deserialize;
mod parse;

fn main() {
    // parse::run();
    deserialize::run().expect("Error");

    // https://serde.rs/transcode.html
}
