mod post;
mod util;

use post::file::File as PostFile;

use tini::Ini;

use std::io::BufReader;
use std::path::{Path,PathBuf};
use util::fs;

fn main() {
    let conf = Ini::from_buffer(
        vec!["[strings]",
        "blog_title = My First Blog",
        "blog_subtitle = Where I write about things and stuff",
        "blog_author = John Doe",
        "[paths]",
        "post_dname = posts",
        "build_dname = build",
        ].join("\n"));
    conf.to_file("conf.ini");
    let s: String = conf.get("strings", "blog_title").unwrap();
    println!("{}", s);

    //let fnames = fs::recursive_find_files(&Path::new("./testdata"));
    //let fnames = fs::paths_with_extension(&fnames, ".reb");
    //for fname in fnames {
    //    let buf = BufReader::new(std::fs::File::open(&fname).unwrap());
    //    let post = PostFile::new_from_buf(Box::new(buf));
    //    println!("{:?}", post);
    //}

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
