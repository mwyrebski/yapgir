use std::env;
use std::ffi::OsStr;
use std::time::{SystemTime, UNIX_EPOCH};

const RAND_MAX: u32 = 32767;

enum PwdType {
    Lowercase,
    Uppercase,
    Numbers,
}

struct Cli {
    usage: bool,
    length: u8,
    count: u8,
    pwd_type: PwdType,
}

fn main() {
    let mut usage = false;
    for argument in env::args() {
        if argument == "h" {
            usage = true;
        }
    }

    if usage {
        let program_name = get_program_name();
        print!(
            "\nUsage: {}\n\n\
                Options:\n\
                \t-l <length>   length of the generated passwords",
            program_name
        );
    }

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    unsafe {
        srand(nanos);
        let r = &rand();
        if *r > RAND_MAX {
            panic!("random number should never be bigger than RAND_MAX");
        }
        println!("RAND: {}", r);
    }
}

fn get_program_name() -> String {
    let exe_path = std::env::current_exe().unwrap();
    let filename = exe_path.file_name().map_or(OsStr::new("_"), |x| x);
    let program_name = filename.to_str().unwrap();
    String::from(program_name)
}

extern "C" {
    fn srand(seed: u32) -> u32;
    fn rand() -> u32;
}
