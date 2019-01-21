mod post;
mod util;
use std::io::BufReader;
use post::file::File as PostFile;
use std::path::{Path,PathBuf};
use util::fs;

fn main() {
    let fnames = fs::recursive_find_files(&Path::new("./testdata"));
    let fnames = fs::paths_with_extension(&fnames, ".reb");
    for fname in fnames {
        let buf = BufReader::new(std::fs::File::open(&fname).unwrap());
        let post = PostFile::new_from_buf(Box::new(buf));
        println!("{:?}", post);
    }
    //let text = "Title: How I Met Your Mother\n#Date: Please\nAuthor: Jake 'n Josh\n\nHi\nthere bob\n\n\n    boyo";
    //let br = BufReader::new(text.as_bytes());
    //let pf = File::new_from_buf(Box::new(br));
    //if pf.is_err() {
    //    println!("ERROR: {}", pf.unwrap_err());
    //    return;
    //}
    //let pf = pf.unwrap();
    //assert!(pf.has_header("title"));
    //println!("{}", pf.get_header("author").unwrap());
    //println!("OK");
}
