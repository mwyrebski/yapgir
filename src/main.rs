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
    pwd_type: PwdType,
}

fn main() {
    match parse_args() {
        Ok(cli) => generate(cli),
        Err(msg_opt) => {
            print_usage(msg_opt);
            return;
        }
    };
}

fn parse_args() -> Result<Cli, Option<String>> {
    let mut length = 10;
    let mut count = 1;
    let mut pwd_type = PwdType::Lowercase;
    let mut parse_count_param = false;
    let mut parse_length_param = false;
    let mut parse_type_param = false;

    let arg_err = |arg| Some(format!("wrong argument ({})", arg));
    let param_arg_err = |param, arg| Some(format!("wrong parameter ({}) for arg ({})", param, arg));

    let args = env::args();
    for arg in args {
        if parse_length_param {
            parse_length_param = false;
            match arg.parse() {
                Ok(val) => length = val,
                Err(_) => return Err(param_arg_err(&arg, "-l")),
            }
        } else if parse_count_param {
            parse_count_param = false;
            match arg.parse() {
                Ok(val) => count = val,
                Err(_) => return Err(param_arg_err(&arg, "-c")),
            }
        } else if parse_type_param {
            parse_type_param = false;
            match arg.as_str() {
                "n" => pwd_type = PwdType::Numbers,
                "u" => pwd_type = PwdType::Uppercase,
                "l" => pwd_type = PwdType::Lowercase,
                _ => return Err(param_arg_err(&arg, "-t")),
            }
        } else {
            //let c = arg.chars().nth(0).unwrap();
            if arg.len() < 2
            // || c != '-'
            {
                return Err(arg_err(&arg));
            }
            match arg.as_str() {
                "-h" => return Err(None),
                "-l" => parse_length_param = true,
                "-c" => parse_count_param = true,
                "-t" => parse_type_param = true,
                _ => return Err(arg_err(&arg)),
            }
        }
    }

    Ok(Cli {
        length,
        count,
        pwd_type,
    })
}

fn generate(cli: Cli) {
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

fn print_usage(message: Option<String>) {
    let program_name = get_program_name();
    if let Some(m) = message {
        println!("ERROR: {}", m);
    }
    println!(
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
