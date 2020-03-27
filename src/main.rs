use core::convert::identity;
use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const PROGRAM_NAME: &str = "passworus";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";

struct CliOptions {
    length: u8,
    count: u8,
    lowercase: bool,
    uppercase: bool,
    numbers: bool,
}

fn main() {
    let mut args = env::args();
    let program_path = &args.nth(0).unwrap();
    let args: Vec<String> = args.collect();

    unsafe {
        match parse_options(args) {
            Ok(cli) => generate(cli),
            Err(err) => print_usage(program_path, err),
        };
    };
}

fn parse_options(args: Vec<String>) -> Result<CliOptions, Option<String>> {
    let mut length = 10;
    let mut count = 1;
    let mut lowercase = true;
    let mut uppercase = true;
    let mut numbers = true;

    let arg_err = |arg| Some(format!("wrong argument ({})", arg));
    let param_arg_err = |arg| Some(format!("wrong parameter for arg ({})", arg));

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        let perr = || param_arg_err(arg);
        match arg.as_str() {
            "-h" => return Err(None),
            "-l" => {
                i += 1;
                length = args.get(i).ok_or(perr())?.parse().or(Err(perr()))?;
                if length < 1 {
                    return Err(perr());
                }
            }
            "-c" => {
                i += 1;
                count = args.get(i).ok_or(perr())?.parse().or(Err(perr()))?;
                if count < 1 {
                    return Err(perr());
                }
            }
            "-t" => {
                i += 1;
                let t = args.get(i).ok_or(perr())?;
                if t.len() < 1 {
                    return Err(perr());
                }
                numbers = false;
                uppercase = false;
                lowercase = false;
                for ch in t.chars() {
                    match ch {
                        'n' => numbers = true,
                        'u' => uppercase = true,
                        'l' => lowercase = true,
                        _ => return Err(perr()),
                    }
                }
            }
            _ => return Err(arg_err(arg)),
        }
        i += 1;
    }

    Ok(CliOptions {
        length,
        count,
        lowercase,
        uppercase,
        numbers,
    })
}

unsafe fn generate(cli: CliOptions) {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    srand(nanos);

    let chars_vec = {
        let mut set = String::new();
        if cli.uppercase {
            set.push_str(UPPERCASE);
        }
        if cli.numbers {
            set.push_str(NUMBERS);
        }
        if cli.lowercase {
            set.push_str(LOWERCASE);
        }
        set.chars().collect::<Vec<char>>()
    };

    for _ in 0..cli.count {
        let mut pass = String::new();
        for _ in 0..cli.length {
            let r = rand() as usize;
            let ch = chars_vec[r % chars_vec.len()];
            pass.push(ch);
        }
        println!("{}", pass);
    }
}

fn print_usage(program_path: &String, err_msg: Option<String>) {
    let error = err_msg
        .map(|e| format!("ERROR: {}\n", e))
        .unwrap_or(String::default());
    eprintln!(
        "{}\
        \nUsage:\n\
        \t{}\n\n\
            Options:\n\
            \t-l <length>   length of the generated passwords (default: 10)\n\
            \t-c <length>   number of passwords to generate (default: 1)\n\
            \t-t [nul]      type of the passwords, any of:\n\
            \t              l - lowercase\n\
            \t              u - uppercase\n\
            \t              n - number\n\
            \t              (default: lun - all options)\n",
        error,
        get_file_name(program_path)
    );
}

fn get_file_name(exe_path: &String) -> String {
    let filename = Path::new(exe_path)
        .file_name()
        .map_or(OsStr::new(PROGRAM_NAME), identity);
    filename.to_str().unwrap().to_string()
}

extern "C" {
    fn srand(seed: u32) -> u32;
    fn rand() -> u32;
}
