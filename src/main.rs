use quick_xml::events::Event;
use quick_xml::reader::Reader;

pub mod bmecat;

fn main() {
    // let xml = "files/fein_de_deu_BMEcat_full_og.xml";
    let xml = "files/nw_bmecat.xml";
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
                    if let Ok(Event::Text(temp)) = reader.read_event_into(&mut buf) {
                        article.id = temp.decode().unwrap().into_owned();
                    }
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
}
