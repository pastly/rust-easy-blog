mod post;
mod template;
mod util;

//#[macro_use]
extern crate structopt;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate config;
extern crate env_logger;
extern crate rand;
extern crate tempfile;

use std::fs::{copy, create_dir_all, metadata, File, OpenOptions};
use std::io::{BufReader, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use chrono::Datelike;
use config::Config;
use config::File as ConfigFile;
use structopt::StructOpt;
use tempfile::NamedTempFile;

use post::file::File as PostFile;
use template::{begin_html, css, end_html, page_footer, page_header, post_footer, post_header};
use util::fs::{paths_with_extension, recursive_find_files};
use util::gen_id;

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
    #[structopt(name = "create")]
    /// Compose a new blog post
    Create {
        /// The title of the post
        #[structopt(required = true)]
        title: Vec<String>,
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
            let post = PostFile::new_from_buf(Box::new(buf), Some(mod_time), None);
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

fn init(args: Args, _conf: Config) -> Result<(), String> {
    trace!("Calling init with {:?}", args);
    Ok(())
}

fn render_index(parser: &str, title: &str, subtitle: &str, posts: &[PostFile]) -> Vec<u8> {
    let mut v = vec![];
    write!(v, "{}", begin_html(title)).unwrap();
    write!(v, "{}", page_header(&title, &subtitle)).unwrap();
    for pf in posts {
        v.extend(render_post_preview(&parser, &pf, true));
    }
    write!(v, "{}", page_footer()).unwrap();
    write!(v, "{}", end_html()).unwrap();
    v
}

fn render_post_header(pf: &PostFile, with_link: bool) -> Vec<u8> {
    let mut v = vec![];
    write!(
        v,
        "{}",
        post_header(
            pf.get_header("title").unwrap(),
            pf.get_header("author").unwrap(),
            if with_link {
                Some("/posts/".to_string() + &pf.get_long_rendered_filename())
            } else {
                None
            },
        )
    )
    .unwrap();
    v
}

fn render_post_body(parser: &str, pf: &PostFile) -> Vec<u8> {
    let mut v = vec![];
    let mut proc = Command::new(parser)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute parser command");
    {
        let stdin = proc
            .stdin
            .as_mut()
            .expect("Failed to open stdin on parser command");
        stdin
            .write_all(pf.get_body().as_bytes())
            .expect("Failed to write post body to parser stdin");
    }
    let output = proc
        .wait_with_output()
        .expect("Failed to get post output from parser stdout");
    writeln!(v, "<div class='post_body'>").unwrap();
    v.extend(output.stdout);
    writeln!(v, "</div> <!-- post_body -->").unwrap();
    v
}

fn render_post_footer() -> Vec<u8> {
    let mut v = vec![];
    write!(v, "{}", post_footer()).unwrap();
    v
}

fn render_post(parser: &str, blog_title: &str, blog_subtitle: &str, pf: &PostFile) -> Vec<u8> {
    let mut v = vec![];
    let title = pf.get_header("title").unwrap() + " | " + blog_title;
    write!(v, "{}", begin_html(&title)).unwrap();
    write!(v, "{}", page_header(&blog_title, &blog_subtitle)).unwrap();
    v.extend(&render_post_preview(&parser, &pf, false));
    write!(v, "{}", page_footer()).unwrap();
    write!(v, "{}", end_html()).unwrap();
    v
}

fn render_post_preview(parser: &str, pf: &PostFile, with_links: bool) -> Vec<u8> {
    let mut v = vec![];
    writeln!(v, "<article>").unwrap();
    v.extend(&render_post_header(&pf, with_links));
    v.extend(&render_post_body(parser, &pf));
    v.extend(&render_post_footer());
    writeln!(v, "</article>").unwrap();
    v
}

fn render_css() -> Vec<u8> {
    let mut v = vec![];
    write!(v, "{}", css()).unwrap();
    v
}

fn build(args: Args, conf: Config) -> Result<(), String> {
    trace!("Calling build with {:?}", args);
    let post_files = find_all_post_files(&conf.get_str("paths.post_dname").unwrap());
    debug!("Found {} valid post files", post_files.len());
    if post_files.is_empty() {
        return Ok(());
    }
    let build_dname = conf.get_str("paths.build_dname").unwrap();
    let parser = conf.get_str("paths.parse_bin").unwrap();
    let blog_title = conf.get_str("strings.blog_title").unwrap();
    let blog_subtitle = conf.get_str("strings.blog_subtitle").unwrap();
    {
        let fname = build_dname.clone() + "/index.html";
        let mut fd = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(fname)
            .unwrap();
        match fd.write_all(&render_index(
            &parser,
            &blog_title,
            &blog_subtitle,
            &post_files,
        )) {
            Ok(_) => {}
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }
    for post_file in &post_files {
        let dname = build_dname.clone() + "/posts";
        let fname = dname + "/" + &post_file.get_long_rendered_filename();
        debug!("Rendering {} ...", fname);
        let mut fd = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(fname)
            .unwrap();
        match fd.write_all(&render_post(
            &parser,
            &blog_title,
            &blog_subtitle,
            &post_file,
        )) {
            Ok(_) => {}
            Err(e) => {
                return Err(e.to_string());
            }
        };
    }
    {
        let fname = build_dname.clone() + "/static/style.css";
        debug!("Rendering {} ...", fname);
        let mut fd = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(fname)
            .unwrap();
        match fd.write_all(&render_css()) {
            Ok(_) => {}
            Err(e) => {
                return Err(e.to_string());
            }
        };
    }
    copy(
        &conf.get_str("paths.blog_img_fname").unwrap(),
        build_dname.clone() + "/static/img/header.jpg",
    )
    .unwrap();
    copy(
        &conf.get_str("paths.favicon_fname").unwrap(),
        build_dname.clone() + "/static/img/favicon.png",
    )
    .unwrap();
    Ok(())
}

fn create(args: Args, conf: Config) -> Result<(), String> {
    trace!("Calling create with {:?}", args);
    let cmd_args = args.cmd;
    let title = &match cmd_args {
        CommandArgs::Create { title } => title.join(" "),
        _ => {
            assert!(false);
            String::new()
        }
    };
    let author = &conf.get_str("strings.blog_author").unwrap();
    let post_id = gen_id();
    let date = chrono::Local::now().to_rfc2822();
    let editor = conf.get_str("paths.editor_bin").unwrap();
    let mut file = NamedTempFile::new().unwrap();
    debug!("Temp file at {:?}", file.path());
    write!(
        file,
        "Title: {title}
Author: {author}
Date: {date}
ID: {post_id}

Post body starts here
",
        title = title,
        author = author,
        post_id = post_id,
        date = date,
    )
    .unwrap();
    let mut proc = Command::new(editor)
        .arg(file.path().to_str().unwrap())
        .spawn()
        .expect("Failed to execute parser command");
    proc.wait().unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    let mod_time = metadata(&file.path()).unwrap().modified().unwrap();
    let buf = BufReader::new(file);
    let pf = PostFile::new_from_buf(Box::new(buf), Some(mod_time), None).unwrap();
    let date = chrono::DateTime::parse_from_rfc2822(&pf.get_header("date").unwrap()).unwrap();
    let out_fname = Path::new(&conf.get_str("paths.post_dname").unwrap())
        .join(&date.year().to_string())
        .join(&date.month().to_string())
        .join(pf.get_suggested_source_filename());
    info!("Saving post to {}", out_fname.to_str().unwrap());
    create_dir_all(out_fname.parent().unwrap()).unwrap();
    let mut fd = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&out_fname)
        .unwrap();
    write!(fd, "{}", pf.to_string()).unwrap();
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
    for key in ["paths.parse_bin", "paths.editor_bin"].iter() {
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
        debug!("Found {:?} for {}", final_s, key);
        conf.set::<String>(key, final_s).unwrap();
    }
    Ok(())
}

fn ensure_dirs(conf: &Config) -> Result<(), String> {
    let mut err = vec![];
    let dnames = vec![
        conf.get_str("paths.post_dname").unwrap(),
        conf.get_str("paths.build_dname").unwrap(),
        conf.get_str("paths.build_dname").unwrap() + "/posts",
        conf.get_str("paths.build_dname").unwrap() + "/p",
        conf.get_str("paths.build_dname").unwrap() + "/static",
        conf.get_str("paths.build_dname").unwrap() + "/static/img",
    ];
    for d in &dnames {
        let meta = metadata(d);
        if meta.is_err() {
            let meta = meta.unwrap_err();
            if meta.kind() == std::io::ErrorKind::NotFound {
                debug!("Making directory {}", d);
                match create_dir_all(d) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e.to_string());
                    }
                };
            } else {
                err.push(meta.to_string());
            }
            continue;
        }
        let meta = meta.unwrap();
        if meta.is_file() {
            err.push(format!("{} must be a directory, but is a file", d));
        }
    }
    if err.is_empty() {
        Ok(())
    } else {
        Err(err.join(", "))
    }
}

fn main() -> Result<(), String> {
    env_logger::init();
    let args = Args::from_args();
    let mut conf = get_config()?;
    normalize_config(&mut conf)?;
    ensure_dirs(&conf)?;

    match args.cmd {
        CommandArgs::Init { .. } => init(args, conf),
        CommandArgs::Build { .. } => build(args, conf),
        CommandArgs::Create { .. } => create(args, conf),
    }
}
