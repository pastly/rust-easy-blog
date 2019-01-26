mod post;
mod util;

//#[macro_use]
extern crate structopt;
#[macro_use]
extern crate log;
extern crate config;
extern crate env_logger;

use std::fs::{File, metadata};
use std::path::{Path, PathBuf};
use std::io::BufReader;

use config::Config;
use config::File as ConfigFile;
use structopt::StructOpt;

use util::fs::{recursive_find_files, paths_with_extension};
use post::file::File as PostFile;
use post::render_post_body;

#[derive(StructOpt, Debug)]
#[structopt(name = "reb")]
struct Args {
    //#[structopt(short="v", long="verbose", parse(from_occurrences))]
    ///// Say more things, repeat to make reb even louder
    //verbose: u8,
    #[structopt(subcommand)]
    cmd: CommandArgs,
}
#[derive(StructOpt, Debug)]
enum CommandArgs {
    #[structopt(name = "init")]
    /// Initialize a directory for reb
    Init {
        #[structopt(short = "f", long = "force")]
        /// Force overwriting existing config
        force: bool,
    },
    #[structopt(name = "build")]
    /// Update the compiled blog
    Build {
        #[structopt(short = "r", long = "rebuild")]
        /// Force a rebuild of all output files
        rebuild: bool,
    },
}

fn find_all_post_files(post_dname: &str) -> Vec<PostFile> {
    let post_files = recursive_find_files(post_dname);
    let post_files = paths_with_extension(&post_files, ".reb");
    let post_files: Vec<PostFile> = {
        let mut v = vec![];
        for fname in post_files {
            let buf = BufReader::new(File::open(&fname).unwrap());
            let mod_time = metadata(&fname).unwrap().modified().unwrap();
            let post = PostFile::new_from_buf(Box::new(buf), Some(mod_time));
            if post.is_err() {
                error!("{}", post.unwrap_err());
            } else {
                v.push(post.unwrap());
            }
        }
        v
    };
    post_files
}

fn init(args: Args, conf: Config) -> Result<(), String> {
    trace!("Calling init with {:?}", args);
    Ok(())
}

fn build(args: Args, conf: Config) -> Result<(), String> {
    trace!("Calling build with {:?}", args);
    let post_files = find_all_post_files(&conf.get_str("paths.post_dname").unwrap());
    debug!("Found {} valid post files", post_files.len());
    if post_files.is_empty() {
        return Ok(())
    }
    for pf in &post_files {
        debug!("{:?} {}", pf.get_last_modified(), pf.get_header("title").unwrap());
    }
    Ok(())
}

fn get_config() -> Result<Config, String> {
    let mut conf = Config::new();
    let ok = conf.merge(ConfigFile::with_name("src/config.default.toml"));
    if ok.is_err() {
        return Err(ok.unwrap_err().to_string());
    }
    Ok(conf)
}

fn search_path(exe: &Path) -> Option<PathBuf> {
    std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths)
            .filter_map(|dir| {
                let p = dir.join(&exe);
                if p.is_file() {
                    Some(p)
                } else {
                    None
                }
            })
            .next()
    })
}

// Only returns Ok(..) if the config is well-formed.
fn normalize_config(conf: &mut Config) -> Result<(), String> {
    // Find various executables. First search in the current working directory, then fall back to
    // searching the PATH
    for key in ["paths.parse_bin"].iter() {
        let value = conf.get_str(key).unwrap();
        let s = Path::new(&value);
        let final_s = if s.is_file() {
            // If it exists in the current directory, use that
            String::from("./") + s.to_str().unwrap()
        } else {
            // Otherwise search path
            let s = search_path(s);
            if s.is_none() {
                return Err(format!("Could not find {} for key={} in PATH", value, key));
            }
            s.unwrap().to_str().unwrap().to_string()
        };
        debug!("Found {:?} for parse_bin", final_s);
        conf.set::<String>("paths.parse_bin", final_s).unwrap();
    }
    Ok(())
}

fn main() -> Result<(), String> {
    env_logger::init();
    let args = Args::from_args();
    let mut conf = get_config()?;
    normalize_config(&mut conf)?;
    match args.cmd {
        CommandArgs::Init { force } => init(args, conf),
        CommandArgs::Build { rebuild } => build(args, conf),
    }
}
