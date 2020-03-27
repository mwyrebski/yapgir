use std::env;
use std::ffi::OsStr;
use std::time::{SystemTime, UNIX_EPOCH};

struct Cli {
    length: u8,
    count: u8,
    lowercase: bool,
    uppercase: bool,
    numbers: bool,
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    unsafe {
        match parse_args(args) {
            Ok(cli) => generate(cli),
            Err(err_msg) => print_usage(err_msg),
        };
    };
}

fn parse_args(args: Vec<String>) -> Result<Cli, Option<String>> {
    let mut length = 10;
    let mut count = 1;
    let mut lowercase = false;
    let mut uppercase = false;
    let mut numbers = false;

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

    Ok(Cli {
        length,
        count,
        lowercase,
        uppercase,
        numbers,
    })
}

unsafe fn generate(cli: Cli) {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    srand(nanos);

    let lowercase = "abcdefghijklmnopqrstuvwxyz".to_string();
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    let numbers = "0123456789".to_string();

    let vec = {
        let mut set = String::new();
        if cli.uppercase {
            set.push_str(&uppercase);
        }
        if cli.numbers {
            set.push_str(&numbers);
        }
        if cli.lowercase || (!cli.uppercase && !cli.numbers) {
            set.push_str(&lowercase);
        }
        set.chars().collect::<Vec<char>>()
    };

    for _ in 0..cli.count {
        let mut pass = String::new();
        for _ in 0..cli.length {
            let r = rand() as usize;
            let ch = vec[r % vec.len()];
            pass.push(ch);
        }
        println!("{}", pass);
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
            \t-l <length>   length of the generated passwords (default: 10)\n\
            \t-c <length>   number of passwords to generate (default: 1)\n\
            \t-t [nul]      type of the passwords, combinations of:\n\
            \t              l - lowercase (default)\n\
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
