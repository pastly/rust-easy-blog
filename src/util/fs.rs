use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};

/// Find all files that are in *d* or in some subdir of *d*, performing a breadth first search
pub fn recursive_find_files(d: &Path) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = Vec::new();
    let mut q: VecDeque<PathBuf> = VecDeque::new();
    q.push_back(PathBuf::from(d));
    while !q.is_empty() {
        let path = q.pop_front().unwrap();
        let new_dirs = fs::read_dir(path);
        // Ignore dir if there was an issue reading it
        if new_dirs.is_err() {
            continue;
        }
        for entry in new_dirs.unwrap() {
            // Ignore entry if there was an issue getting it
            if entry.is_err() {
                continue;
            }
            let path = entry.unwrap().path();
            if path.is_dir() {
                q.push_back(PathBuf::from(path));
            } else {
                v.push(PathBuf::from(path));
            }
        }
    }
    v
}

///// Given a vec of files *paths*, return all the ones that have a filename matching *s*
/////
///// If *s* is "foo", then "foo/bar" is not a match, but "foo" and "bar/foo" both are
//fn paths_file_name_matching(paths: Vec<PathBuf>, s: &str) -> Vec<PathBuf> {
//    paths.into_iter().filter(|f| f.file_name().unwrap() == s).collect()
//}

/// Given a vec of files *paths*, return all that have any extension in *exts*
fn paths_with_any_extension(paths: &[PathBuf], exts: &[&str]) -> Vec<PathBuf> {
    let mut out = vec![];
    for path in paths {
        for ext in exts {
            if path.to_str().unwrap().ends_with(ext) {
                out.push(path.clone());
                break;
            }
        }
    }
    out
}

pub fn paths_with_extension(paths: &[PathBuf], ext: &str) -> Vec<PathBuf> {
    paths_with_any_extension(paths, &[ext])
}

///// Recursively find all ignore files in directory *d*
//pub fn find_ignore_files(d: &Path) -> Vec<PathBuf> {
//    let files = recursive_find_files(d);
//    let files = paths_file_name_matching(files, "gsignore");
//    files
//}
//
///// Recursively find all metadata files in directory *d*
//pub fn find_metadata_files(d: &Path) -> Vec<PathBuf> {
//    let files = recursive_find_files(d);
//    let files = paths_file_name_matching(files, "metadata");
//    files
//}
//
///// Recursively find all image files (based on extension) in directory *d*
//pub fn find_image_files(d: &Path) -> Vec<PathBuf> {
//    let exts: Vec<&str> = vec![".png", ".jpg", ".jpeg", ".gif", ".bmp"];
//    let files = recursive_find_files(d);
//    let files = paths_with_any_extension(files, &exts);
//    files
//}
