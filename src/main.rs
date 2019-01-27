mod post;
mod util;

//#[macro_use]
extern crate structopt;
#[macro_use]
extern crate log;
extern crate config;
extern crate env_logger;

use std::fs::{create_dir_all, metadata, File, OpenOptions};
use std::io::{BufReader, BufWriter, Cursor, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use config::Config;
use config::File as ConfigFile;
use structopt::StructOpt;

use post::file::File as PostFile;
use util::fs::{paths_with_extension, recursive_find_files};

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

fn render_index(parser: &str, title: &str, subtitle: &str, posts: &[PostFile]) -> Vec<u8> {
    let mut v = vec![];
    write!(
        v,
        "
<html>
<head>
    <title>Blog Title</title>
    <link href='/static/style.css' rel='stylesheet' type='text/css' />
    <link rel='icon' type=image/png' href='/static/img/favicon.png' />
    <meta charset='utf-8' />
</head>
<body>
<div id='page_content'>
<header>
    <h1>{}</h1>
    <h2>{}</h2>
</header>\n",
        title, subtitle
    );
    for pf in posts {
        v.extend(render_post(&parser, &pf));
    }
    write!(
        v,
        "
</div> <!-- page_content -->
</body>
</html>\n"
    );
    v
}

fn render_post_header(pf: &PostFile) -> Vec<u8> {
    let mut v = vec![];
    write!(
        v,
        "
<div class='post_header'>
    <h1 class='post_title'>{}</h1>
    <p class='post_author'>{}</p>
    <p class='post_date'></p>
    <p class='post_mod_date'></p>
    <p class='post_permalink'></p>
</div> <!-- post_header -->\n",
        pf.get_header("title").unwrap(),
        pf.get_header("author").unwrap()
    );
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
        let mut stdin = proc
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
    write!(v, "<div class='post_body'>\n");
    v.extend(output.stdout);
    write!(v, "</div> <!-- post_body -->\n");
    v
}

fn render_post(parser: &str, pf: &PostFile) -> Vec<u8> {
    let mut v = vec![];
    write!(v, "<article>\n");
    v.extend(&render_post_header(&pf));
    v.extend(&render_post_body(parser, &pf));
    write!(v, "</article>\n");
    v
}

fn render_css() -> Vec<u8> {
    let mut v = vec![];
    write!(v, "
body {{
    font-family: Georgia, 'Times New Roman', Times, serif;
    margin: 0;
    padding: 0;
    background-color: #F3F3F3;
}}
header,
footer,
article {{
    background-color: #FFF;
    border: 1px solid #CCC;
}}
article {{
    padding: 20px 40px 20px 40px;
}}
#page_content {{
    padding: 5px;
    background-color: #DDD;
    max-width: 900px;
    margin: 24px auto;
}}\n");
    v
}

fn build(args: Args, conf: Config) -> Result<(), String> {
    trace!("Calling build with {:?}", args);
    let post_files = find_all_post_files(&conf.get_str("paths.post_dname").unwrap());
    debug!("Found {} valid post files", post_files.len());
    if post_files.is_empty() {
        return Ok(());
    }
    for pf in &post_files {
        debug!(
            "{:?} {}",
            pf.get_last_modified(),
            pf.get_header("title").unwrap()
        );
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
        fd.write_all(&render_index(
            &parser,
            &blog_title,
            &blog_subtitle,
            &post_files,
        ));
    }
    {
        let mut fd = Cursor::new(vec![]);
        fd.write_all(&render_index(
            &parser,
            &blog_title,
            &blog_subtitle,
            &post_files,
        ));
        println!("{}", String::from_utf8(fd.into_inner()).unwrap());
    }
    {
        let mut fd = Cursor::new(vec![]);
        fd.write_all(&render_post(&parser, &post_files[0]));
        println!("{}", String::from_utf8(fd.into_inner()).unwrap());
    }
    {
        let fname = build_dname.clone() + "/static/style.css";
        let mut fd = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(fname)
            .unwrap();
        fd.write_all(&render_css());
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

fn ensure_dirs(conf: &Config) -> Result<(), String> {
    let mut err = vec![];
    let dnames = vec![
        conf.get_str("paths.post_dname").unwrap(),
        conf.get_str("paths.build_dname").unwrap(),
        conf.get_str("paths.build_dname").unwrap() + "/static",
    ];
    for d in &dnames {
        let meta = metadata(d);
        if meta.is_err() {
            let meta = meta.unwrap_err();
            if meta.kind() == std::io::ErrorKind::NotFound {
                debug!("Making directory {}", d);
                create_dir_all(d);
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
        CommandArgs::Init { force } => init(args, conf),
        CommandArgs::Build { rebuild } => build(args, conf),
    }
}
