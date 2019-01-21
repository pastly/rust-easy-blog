mod post;
use std::io::BufReader;
use post::file::File;

fn main() {
    let text = "Title: How I Met Your Mother\n#Date: Please\nAuthor: Jake 'n Josh\n\nHi\nthere bob\n\n\n    boyo";
    let br = BufReader::new(text.as_bytes());
    let pf = File::new_from_buf(Box::new(br));
    if pf.is_err() {
        println!("ERROR: {}", pf.unwrap_err());
        return;
    }
    let pf = pf.unwrap();
    assert!(pf.has_header("title"));
    println!("{}", pf.get_header("author").unwrap());
    println!("OK");
}
