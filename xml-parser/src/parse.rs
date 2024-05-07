use std::fs;

use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;

pub fn run() {
    let xml = fs::read_to_string("./test.xml").unwrap();
    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);

    let mut txt = Vec::new();
    let mut buf = Vec::new();
    let mut path = vec![String::from("root")];

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break println!("Completed."),

            Ok(Event::Start(node)) => {
                let name = String::from_utf8(node.name().0.to_vec()).unwrap();
                path.push(name);
                process_attributes(&mut path, node);
                println!("{}", path.join("/"));
            }

            Ok(Event::Empty(node)) => {
                let name = String::from_utf8(node.name().0.to_vec()).unwrap();
                path.push(format!("{} empty", name));
                println!("{}", path.join("/"));
                process_attributes(&mut path, node);
                path.pop();
            }

            Ok(Event::End(node)) => {
                let name = String::from_utf8(node.name().0.to_vec()).unwrap();
                path.pop();
                println!("{}, closed: {}", path.join("/"), name);
            }

            Ok(Event::Text(text)) => {
                let text = text.unescape().unwrap().into_owned();
                path.push(format!("@text: {}", text));

                txt.push(text);

                println!("{}", path.join("/"));

                path.pop();
            }

            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    println!("txt is: {}", txt.join(", "));
}

fn process_attributes(path: &mut Vec<String>, node: BytesStart) {
    for attribute in node.attributes() {
        let att = attribute.unwrap();
        let key = String::from_utf8(att.key.0.to_vec()).unwrap();
        let value = String::from_utf8(att.value.to_vec()).unwrap();

        path.push(format!("@attribute {}={}", key, value));
        println!("{}", path.join("/"));
        path.pop();
    }
}
