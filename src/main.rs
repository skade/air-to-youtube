extern crate air_to_youtube;

#[macro_use] extern crate shells;

use air_to_youtube::*;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let f = File::open("sample_data/rust.xml").unwrap();
    let r = BufReader::new(f);

    let items = parse_channel(r);

    for i in items {
        println!("Downloading video {}: {} - {}", &i.contentId, &i.title, &i.streamUrl);
        sh!("youtube-dl '{}' -o '{} - {}.mp4'", &i.streamUrl, &i.contentId, &i.title);

        println!("Uploading video {}: {} - {}", &i.contentId, &i.title, &i.streamUrl);
        sh!("youtube-upload --privacy=private --client-secrets=client_secrets.json --recording-date={} --default-language=en --tags 'rust, mozilla air' --title='{}' --description='{}' --playlist='Mozilla Air' '{} - {}.mp4'", &i.date, &i.title, &i.synopsis, &i.contentId, &i.title);
    }
}
