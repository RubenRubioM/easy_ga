//! This module contains the implementation for verbosity debugging in the library.

use cpu_time::ProcessTime;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

static mut LEVEL: VerbosityLevel = VerbosityLevel::DISABLED;
static mut TYPE: VerbosityType = VerbosityType::LOG_AND_SAVE;
const PATH: &str = "target/tmp/";

lazy_static! {
    static ref UUID: String = Uuid::new_v4().to_string();
}

/// Enumeration for the different levels of verbosity.
#[derive(Debug, PartialEq, PartialOrd)]
pub enum VerbosityLevel {
    DISABLED = 0,
    LOW = 1,
    MID = 2,
    HIGH = 3,
}

#[allow(non_camel_case_types)]
pub enum VerbosityType {
    LOG = 0,
    SAVE = 1,
    LOG_AND_SAVE = 2,
}

/// Sets de verbosity level.
#[allow(non_snake_case)]
pub fn LOG_verbosity(verbosity: VerbosityLevel) {
    // unsafe: Modification of static mutable variable.
    unsafe {
        LEVEL = verbosity;
    }
}

/// Sets de verbosity type.
#[allow(non_snake_case)]
pub fn LOG_verbosity_type(verbosity_type: VerbosityType) {
    // unsafe: Modification of static mutable variable.
    unsafe {
        TYPE = verbosity_type;
    }
}

/// Prints the log.
#[allow(non_snake_case)]
pub fn LOG(verbosity: VerbosityLevel, text: &str) {
    unsafe {
        if LEVEL != VerbosityLevel::DISABLED && verbosity != VerbosityLevel::DISABLED {
            let now = ProcessTime::now();
            // Format: MS |VerbosityLevel| Text
            let message = format!("{:?} |{:?}| {text}", now.as_duration(), verbosity);
            if verbosity <= LEVEL {
                match TYPE {
                    VerbosityType::LOG => {
                        print(&message);
                    }
                    VerbosityType::SAVE => {
                        save(&message);
                    }
                    VerbosityType::LOG_AND_SAVE => {
                        print(&message);
                        save(&message);
                    }
                }
            }
        }
    }
}

/// Prints the LOG.
fn print(text: &str) {
    println!("{text}");
}

/// Saves the LOG.
#[allow(unused_assignments)]
fn save(text: &str) {
    let mut file: File;
    let full_path = format!("{PATH}easy_ga-{}.log", *UUID);
    let path = std::path::Path::new(&full_path);
    if !Path::exists(path) {
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        file = File::create(&full_path).unwrap();
    }

    file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    writeln!(file, "{text}").unwrap();
}
