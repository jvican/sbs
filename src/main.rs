mod consts;
mod utils;

extern crate clap;
extern crate xdg;
extern crate hyper;
extern crate flate2;
extern crate tar;

use clap::{Arg, App};
use std::process::Command;
use consts::SBS;
use utils::{create_symlink_in_bin, openjdk_setup_thread, sbt_setup_thread};

/*
  WHAT IS SUPPOSED TO HAPPEN:

  0. Copy this binary to $HOME/bin/.
  1. Figure out which versions we want: 
      Java version – use the following precedence:
      - Use commandline argument (TODO)
      - Read from build.properties (TODO)
      - Query current stable version available from Azul (TODO)
      - Use hard-coded version
      SBT launcher version – use the following precedence:
      - Use commandline argument (TODO)
      - Query current stable version available from repository (TODO)
      - Use hard-coded version
      SBT version: (TODO)
      The SBT launcher deals with that, but if a "bootstrap SBT file" becomes available in the future:
       - Read from build.properties (TODO)
       - Use hard-coded version (in sbt-launcher) (TODO)
  2. Figure out whether we have the requested versions available locally, otherwise download and extract:
      OpenJDK:
      - Does the directory exist? (TODO)
        yes -> nothing to do
        no  -> Do we have a complete zip archive? (TODO)
               yes -> extract it
               no  -> download and extract it
      SBT launcher:
      - Does the jar file exist?
        yes -> nothing to do
        no  -> download it
      SBT: (TODO)
      The SBT launcher deals with that, but if a "bootstrap SBT file" becomes available in the future:
      - Download SBT bootstrap file
      - Extract it to the place SBT expects it
  3, Run SBT using the requested JDK and SBT launcher version.

  OPTIONS WHICH MIGHT MAKE SENSE:
  - offline:          Try to build/run without internet access (TODO)
  - local-jdk=<path>: Try to use a locally installed JDK       (TODO)
 */
fn main() {
  let matches = App::new(SBS)
                    .version("0.0.1")
                    .author("Simon Ochsenreither <simon@ochsenreither.de>")
                    .about("Downloads, installs and manages OpenJDK and SBT")
                    .arg(Arg::with_name("java-version")
                         .short("j")
                         .long("java")
                         .value_name("VERSION")
                         .help("Sets a custom config file")
                         .takes_value(true))
                    .arg(Arg::with_name("sbt-launcher-version")
                         .short("s")
                         .long("sbt-l-v")
                         .value_name("VERSION")
                         .help("Sets the input file to use")
                         .takes_value(true))
                    .get_matches();

  let jv = matches.value_of("java-version");
  let sv = matches.value_of("sbt-launcher-version");
  println!("{:?}", jv);
  println!("{:?}", sv);

  create_symlink_in_bin();

  let openjdk_setup_thread_result = openjdk_setup_thread().join();
  let     sbt_setup_thread_result =     sbt_setup_thread().join();
  println!("{:?}", openjdk_setup_thread_result);
  println!("{:?}", sbt_setup_thread_result);

  let command_result =
    Command::new("/home/soc/.cache/sbs/openjdk-zulu/zulu8.13.0.5-jdk8.0.72-linux_x64/bin/java")
            .arg("-jar").arg("/home/soc/.cache/sbs/sbt-launcher/0.13.11/sbt-launch.jar")
          //.env("JAVA_HOME", "/home/soc/.cache/sbs/openjdk-zulu/zulu8.13.0.5-jdk8.0.72-linux_x64/") // not really necessary
            .status();

  println!("{:?}", command_result);
}
