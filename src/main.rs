use chrono::prelude::*;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

pub mod bmecat;

fn main() {
    let start_time = Local::now();
    //let xml = "files/fein_de_deu_BMEcat_full_og.xml";
    //let xml = "files/nw_bmecat.xml";
    let xml = "files/nw_bmecat_2.xml";
    let mut reader = Reader::from_file(xml).unwrap();
    reader.config_mut().trim_text(true);

    let mut count = 0;
    let mut buf = Vec::new();
    let mut article = bmecat::Article::default();

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"ARTICLE" => {
                    article = bmecat::Article::default();
                    count += 1;
                }
                b"SUPPLIER_AID" => {
                    article.id = read_text_content(&mut reader, &mut buf);
                }
                _ => (),
            },

            Ok(Event::End(e)) => match e.name().as_ref() {
                b"ARTICLE" => {
                    if count % 1000 == 0 {
                        println!("Article Count: {} - ID {}", count, article.id);
                    }
                }
                _ => (),
            },
            // Ok(Event::Text(e)) => txt.push(e.decode().unwrap().into_owned()),
            _ => (),
        }
        buf.clear();
    }
    println!("Total articles found: {}", count);
    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);

    println!("{:?}", duration);
}

fn read_text_content(
    reader: &mut Reader<std::io::BufReader<std::fs::File>>,
    buf: &mut Vec<u8>,
) -> String {
    if let Ok(Event::Text(temp)) = reader.read_event_into(buf) {
        str_conv(temp.decode().unwrap_or_default().as_ref())
    } else {
        String::new()
    }
}

fn str_conv(str: &str) -> String {
    // only chars in Windows-1252 range
    str.chars()
        .filter(|c| ((c >= &'!' && c <= &'ÿ') && c != &',' && c != &'\'') || c == &' ')
        .collect()
}
