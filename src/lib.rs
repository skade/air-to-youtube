extern crate xml;
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate derive_builder;

use xml::reader::{EventReader, XmlEvent};
use std::io::Read;

custom_derive!{
    #[derive(Debug, PartialEq, Default, Clone, Builder)]
    pub struct Item {
        pub title: String,
        pub contentId: u64,
        pub contentType: String,
        pub streamFormat: String,
        pub synopsis: String,
        pub description: String,
        pub runtime: u64,
        pub streamUrl: String,
        pub date: String
    }
}

pub fn parse_channel<R: Read>(channel_data: R) -> Vec<Item> {
    let mut items = Vec::new();
    let parser = EventReader::new(channel_data);

    let mut item = Item::default();
    let mut open_element = String::new();

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                open_element.clear();

                open_element.push_str(name.local_name.as_ref());
            }
            Ok(XmlEvent::Characters(chars)) => {
                match open_element.as_ref() {
                    "title" => {
                        let mut splits = chars.trim().split("-").collect::<Vec<_>>();
                        let date = splits.pop().expect("expected a date");
                        item.title(splits.join("-").trim());
                        item.date(date_to_somewhat_iso(date.into()));
                    },
                    "contentId" => {
                        item.contentId(chars.trim().parse::<u64>().expect("contentId should be parseable as a number!"));
                    }
                    "contentType" => {
                        item.contentType(chars.trim());
                    }
                    "streamFormat" => {
                        item.streamFormat(chars.trim());
                    }
                    "streamUrl" => {
                        item.streamUrl(chars.trim());
                    }
                    "synopsis" => {
                        item.synopsis(chars.trim());
                    }
                    "description" => {
                        item.description(chars.trim());
                    }
                    "runtime" => {
                        item.runtime(chars.trim().parse::<u64>().expect("runtime should be parseable as a number!"));
                    }
                    _ => { }
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "item" {
                    items.push(item.clone());
                    item = Item::default();
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    items
}

fn date_to_somewhat_iso(date: String) -> String {
    let fragments = date.trim().split(" ").collect::<Vec<_>>();

    let month = match fragments[0] {
        "Jan" => "01",
        "Feb" => "02",
        "Mar" => "03",
        "Apr" => "04",
        "May" => "05",
        "Jun" => "06",
        "Jul" => "07",
        "Aug" => "08",
        "Sep" => "09",
        "Oct" => "10",
        "Nov" => "11",
        "Dec" => "12",
        _ => panic!("found unknown month!")
    };
    let day = fragments[1];
    let year = fragments[2];

    format!("{}-{}-{}T00:00:00.0Z", year, month, day)
}
