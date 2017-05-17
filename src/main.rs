extern crate irc;
use irc::client::prelude::*;

extern crate clap;
use clap::{App, Arg};

extern crate fuse;
// use fuse::{Filesystem, Request, ReplyDirectory};
use fuse::*;

extern crate daemonize;
use daemonize::Daemonize;

extern crate libc;
use libc::{ENOENT, ENOSYS};

extern crate time;
use time::Timespec;

use std::mem;
use std::os::raw::c_int;
use std::env::current_dir;
use std::process::exit;
use std::path::{Path, PathBuf};
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

pub mod ircfs;
use ircfs::*;

fn main() {
    let matches = App::new("riiir")
        .arg(Arg::with_name("mountpoint")
             .value_name("PATH")
             .required(true)
             .index(1))
        .arg(Arg::with_name("config")
             .value_name("FILE")
             .help("Specify path to config file")
             .short("c")
             .long("config")
             .takes_value(true))
        .arg(Arg::with_name("daemonize")
             .short("d")
             .long("daemonize"))
        .get_matches();

    // Replace this with clap later
    let mut mountpoint = PathBuf::from(matches.value_of_os("mountpoint").unwrap());

    if mountpoint.is_relative() {
        let mut current_directory = match current_dir() {
            Ok(dir) => dir,
            Err(e) => {
                println!("Could not determine current directory: {}", e);
                exit(1);
            },
        };
        current_directory.push(mountpoint);
        mountpoint = current_directory;
    }

    if matches.is_present("daemonize") {
        let daemon = Daemonize::new()
            .privileged_action(move || {
                fuse::mount(IrcFs::new(), &mountpoint, &[]);
            });

        let _ = daemon.start();
    } else {
        fuse::mount(IrcFs::new(), &mountpoint, &[]);
    }
}
