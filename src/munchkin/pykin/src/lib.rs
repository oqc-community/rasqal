#![warn(clippy::all, clippy::pedantic)]
#![allow(non_upper_case_globals, non_snake_case, unused_macros, dead_code, unused_variables, unused_must_use)]
#![allow(clippy::needless_pass_by_value)]
#![feature(get_mut_unchecked)]
#![feature(unwrap_infallible)]
#![feature(strict_provenance)]

extern crate core;

use std::env::{current_exe};
use std::fs::{File};
use log::{Level, LevelFilter, log, log_enabled};

mod python;
mod execution;
mod smart_pointers;
mod runtime;
mod builders;
mod hardware;
mod instructions;
mod evaluator;
mod graphs;
mod analysis;

const DEFAULT_LOG_FILE: &str = "mk_logs.txt";

/// Native initialization of the loggers. Defaults to executable position if deployed, if it
/// detects it's in development mode it'll move log file back up the folder tree.
#[ctor::ctor]
fn native_logger_initialize() {
     let path = if let Ok(val) = current_exe() {
          // If we're embedded we need to be given a different file path to log too.
          if val.ends_with("python.exe") {
               return;
          }

          let current_folder = val.parent().unwrap();

          // Walk back to root munchkin folder if we're in a build, otherwise at that folder level.
          if current_folder.ends_with("deps") {
               Some(current_folder.parent().unwrap().parent().unwrap().parent().unwrap().join(DEFAULT_LOG_FILE).to_str().unwrap().to_string())
          } else {
               Some(current_folder.join(DEFAULT_LOG_FILE).to_str().unwrap().to_string())
          }
     } else {
          None
     };

     initialize_loggers(path);
     log!(Level::Info, "Initialized on library startup.");
}

fn initialize_loggers(log_path: Option<String>) {
     // If we've already been enabled, just do nothing.
     if log_enabled!(Level::Error) {
          return;
     }

     let mut appended_messages = Vec::new();
     if let Some(logging_path) = log_path {
          let file = File::create(logging_path.clone());
          if let Ok(file) = file {
               // TODO: Just print to both commandline and file.
               let target = Box::new(file);
               env_logger::builder()
                 .target(env_logger::Target::Pipe(target))
                 .filter_level(LevelFilter::Debug)
                 .format_suffix("\n")
                 .init();

               log!(Level::Info, "File logging initialized.");
               return;
          }

          appended_messages.push(format!("Attempted to open file at {} to log, failed with: {}", logging_path, file.err().unwrap().to_string()));
     }

     // If we're fallen through previous forms of logger init have failed.
     env_logger::builder().filter_level(LevelFilter::Debug).init();

     log!(Level::Info, "Commandline logging initialized.");
     for val in appended_messages {
          log!(Level::Info, "{}", val);
     }
}