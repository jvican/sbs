use consts::*;

use std::fs;
use std::io;
use std::env;
use std::thread;

use std::iter::FromIterator;

use std::path::Path;
use std::path::PathBuf;

use xdg;

use hyper::Client;
use hyper::header::Connection;
use hyper::client::response::Response;

use flate2::read::GzDecoder;

use tar::Archive;

pub fn openjdk_setup_thread() -> thread::JoinHandle<()> {
  thread::spawn(move || {
    println!("OpenJDK start");
    let xdg_dirs                        = xdg::BaseDirectories::with_prefix(SBS).unwrap();
    let xdg_dirs: &xdg::BaseDirectories = &xdg_dirs;
    let openjdk_relative_path        = PathBuf::from_iter(&[OPENJDK_PREFIX, OPENJDK_NAME]);
    let openjdk_relative_path: &Path = openjdk_relative_path.as_path();
    let openjdk_result = download_if_necessary(xdg_dirs, OPENJDK_CDN_URL, openjdk_relative_path);
    println!("{:?}", openjdk_result);
    let uncompression_result = uncompress_openjdk(xdg_dirs, openjdk_relative_path);
    println!("OpenJDK done: {:?}", uncompression_result);
  })
}

pub fn sbt_setup_thread() -> thread::JoinHandle<()> {
  thread::spawn(move || {
    println!("SBT start");
    let xdg_dirs                        = xdg::BaseDirectories::with_prefix(SBS).unwrap();
    let xdg_dirs: &xdg::BaseDirectories = &xdg_dirs;
    let sbt_relative_path        = PathBuf::from_iter(&[SBT_PREFIX, SBT_VERSION, SBT_NAME]);
    let sbt_relative_path: &Path = sbt_relative_path.as_path();
    let sbt_result     = download_if_necessary(xdg_dirs, SBT_CDN_URL,     sbt_relative_path);
    println!("SBT done: {:?}", sbt_result);
  })
}

pub fn download_if_necessary(xdg_dirs: &xdg::BaseDirectories, source_url: &'static str, target_path: &Path) -> Result<u64,io::Error> {
  println!("XDG:         {:?}", xdg_dirs.get_cache_home());
  println!("source url:  {:?}", source_url);
  println!("target path: {:?}", target_path);
  xdg_dirs.place_cache_file(target_path)
    .and_then(|path|     if path.is_file() { file_exists_err() } else { Ok(path) })
    .and_then(|path|     fs::OpenOptions::new().create(true).write(true).truncate(false).open(path))
    .and_then(|mut file| io::copy(&mut download(source_url), &mut file))
}

pub fn file_exists_err<T>() -> Result<T,io::Error> { Err(io::Error::new(io::ErrorKind::AlreadyExists, "File already exists!")) }

pub fn download(url: &'static str) -> Response {
  Client::new().get(url).header(Connection::close()).send().unwrap()
}

pub fn uncompress_openjdk(xdg_dirs: &xdg::BaseDirectories, openjdk_relative_path: &Path) -> io::Result<()> {
  let openjdk_absolute_path        = xdg_dirs.find_cache_file(openjdk_relative_path).unwrap();
  let openjdk_absolute_path: &Path = openjdk_absolute_path.as_path();

  let target_path        = xdg_dirs.create_cache_directory(OPENJDK_PREFIX).unwrap();
  let target_path: &Path = target_path.as_path();

  uncompress_file(openjdk_absolute_path, target_path)
}

pub fn uncompress_file(source_file: &Path, target_path: &Path) -> io::Result<()> {
  let gz_file = fs::OpenOptions::new().read(true).open(source_file).unwrap();
  let tar_archive = GzDecoder::new(gz_file).unwrap();
  let mut archive = Archive::new(tar_archive);
  println!("{:?}", target_path);
  archive.unpack(target_path)
}

#[allow(deprecated)]
pub fn create_symlink_in_bin() {
  let current_executable_path        = env::current_exe().unwrap();
  let current_executable_path: &Path = current_executable_path.as_path();

  let mut new_executable_home_path   = env::home_dir().unwrap();
  new_executable_home_path.push(BIN_DIR.to_string() + SBS);
  let new_executable_home_path: &Path = new_executable_home_path.as_path();

  let symlink_result = fs::soft_link(current_executable_path, new_executable_home_path);
  println!("{:?}", symlink_result);
}
