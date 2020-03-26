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
    length: u8,
    count: u8,
    ptype: PwdType,
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match parse_args(args) {
        Ok(cli) => generate(cli),
        Err(err_msg) => print_usage(err_msg),
    };
}

fn parse_args(args: Vec<String>) -> Result<Cli, Option<String>> {
    let mut length = 10;
    let mut count = 1;
    let mut ptype = PwdType::Lowercase;

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
            }
            "-c" => {
                i += 1;
                count = args.get(i).ok_or(perr())?.parse().or(Err(perr()))?;
            }
            "-t" => {
                i += 1;
                let t = args.get(i).ok_or(perr())?;
                match t.as_str() {
                    "n" => ptype = PwdType::Numbers,
                    "u" => ptype = PwdType::Uppercase,
                    "l" => ptype = PwdType::Lowercase,
                    _ => return Err(perr()),
                }
            }
            _ => return Err(arg_err(arg)),
        }
        i += 1;
    }

    Ok(Cli {
        length,
        count,
        ptype,
    })
}

fn generate(_cli: Cli) {
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

fn print_usage(err_msg: Option<String>) {
    let program_name = get_program_name();
    if let Some(m) = err_msg {
        eprintln!("ERROR: {}", m);
    }
    eprintln!(
        "\nUsage:\n\
        \t{}\n\n\
            Options:\n\
            \t-l <length>   length of the generated passwords\n\
            \t-c <length>   number of passwords to generate\n\
            \t-t [l,u,n]    l - lowercase\n\
            \t              u - uppercase\n\
            \t              n - number\n",
        program_name
    );
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
