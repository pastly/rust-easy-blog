mod post;
mod util;

#[macro_use]
extern crate structopt;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::path::Path;

use tini::Ini;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name="reb")]
struct Args {
    //#[structopt(short="v", long="verbose", parse(from_occurrences))]
    ///// Say more things, repeat to make reb even louder
    //verbose: u8,
    #[structopt(subcommand)]
    cmd: CommandArgs,
}
#[derive(StructOpt, Debug)]
enum CommandArgs {
    #[structopt(name="init")]
    /// Initialize a directory for reb
    Init {
        #[structopt(short="f", long="force")]
        /// Force overwriting existing config
        force: bool,
    },
    #[structopt(name="build")]
    /// Update the compiled blog
    Build {
        #[structopt(short="r", long="rebuild")]
        /// Force a rebuild of all output files
        rebuild: bool,
    },
}

fn init(args: Args) -> Result<(), String> {
    trace!("Calling init with {:?}", args);
    let path = Path::new("reb.ini");
    let force = match &args.cmd {
        CommandArgs::Init{force} => force,
        _ => unreachable!(),
    };
    if path.exists() && !force {
        let path = path.to_str().unwrap();
        let s = format!("{} exists and refusing to overwrite it", path);
        error!("{}", s);
        return Err(s);
    }
    let path = path.to_str().unwrap();
    info!("Writing default config to {}", path);
    Ini::from_buffer("
[strings]
blog_title = My First BLog
blog_subtitle = Where I write about things and stuff
blog_author = John Doe

[paths]
post_dname = posts
build_dname = build
").to_file(path);
    //if path.exists() && !args.cmd.force {
    //    error!("{} exists and refusing to overwrite it", path.to_str().unwrap());
    //}
    //let config = get_default_config();
    Ok(())
}

fn build(args: Args) -> Result<(), String> {
    trace!("Calling build with {:?}", args);
    Ok(())
}

fn main() -> Result<(), String> {
    env_logger::init();
    let args = Args::from_args();
    match args.cmd {
        CommandArgs::Init{force} => init(args),
        CommandArgs::Build{rebuild} => build(args),
    }
    //let conf = Ini::from_buffer(
    //    vec!["[strings]",
    //    "blog_title = My First Blog",
    //    "blog_subtitle = Where I write about things and stuff",
    //    "blog_author = John Doe",
    //    "[paths]",
    //    "post_dname = posts",
    //    "build_dname = build",
    //    ].join("\n"));
    //conf.to_file("conf.ini");
    //let s: String = conf.get("strings", "blog_title").unwrap();
    //println!("{}", s);

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
