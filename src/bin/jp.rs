use std::{
    env,
    fs::read_to_string,
    io::{stdin, Read},
    process::exit,
};

use json_prettier::JsonObject;

fn usage() {
    eprintln!("jp - command line JSON minimum prettier");
    eprintln!("USAGE:");
    eprintln!("      mj [OPTIONS...] [FILE] [OPTIONS...]");
    eprintln!("ARGS:");
    eprintln!("     <FILE> A JSON file");
    eprintln!("OPTIONS:");
    eprintln!("       -h,--help      Print help information");
    eprintln!("       -c,--color     Color JSON output");
    eprintln!("       -m,--minimize  Minimize JSON output");
}

fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[m", s)
}
fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[m", s)
}
fn yellow(s: &str) -> String {
    format!("\x1b[33m{}\x1b[m", s)
}
fn do_minimum_output(value: &JsonObject, color: bool) {
    match value {
        JsonObject::Number(v) => {
            print!("{}", v);
        }
        JsonObject::Bool(v) => {
            print!("{}", v);
        }
        JsonObject::String(s) => {
            let s = if color { green(s) } else { s.to_string() };
            print!("\"{}\"", s);
        }
        JsonObject::Array(vs) => {
            print!("[");
            vs.iter().enumerate().for_each(|(i, v)| {
                do_minimum_output(v, color);
                if i != vs.len() - 1 {
                    print!(",");
                }
            });
            print!("]");
        }
        JsonObject::Object(vs) => {
            print!("{{");
            vs.iter().enumerate().for_each(|(i, (k, v))| {
                let k = if color { yellow(k) } else { k.to_string() };
                print!("\"{}\":", k);
                do_minimum_output(v, color);
                if i != vs.len() - 1 {
                    print!(",");
                }
            });
            print!("}}");
        }
        JsonObject::Null => {
            let v = if color {
                red("null")
            } else {
                "null".to_string()
            };
            print!("{}", v);
        }
    }
}
fn do_output(value: &JsonObject, color: bool, indent: usize, special: bool) {
    match value {
        JsonObject::Number(v) => {
            print!("{}", v);
        }
        JsonObject::Bool(v) => {
            print!("{}", v)
        }
        JsonObject::String(s) => {
            let s = if color { green(s) } else { s.to_string() };
            print!("\"{}\"", s);
        }
        JsonObject::Array(vs) => {
            if special {
                println!("[");
            } else {
                println!("{:indent$}[", "", indent = indent);
            }

            vs.iter().enumerate().for_each(|(i, v)| {
                print!("{:indent$}", "", indent = indent + 3);
                match &v {
                    JsonObject::Object(_) | JsonObject::Array(_) => {
                        do_output(v, color, indent + 3, true);
                    }
                    _ => {
                        do_output(v, color, indent + 3, false);
                    }
                };

                if i != vs.len() - 1 {
                    println!(",");
                } else {
                    println!();
                }
            });
            print!("{:indent$}]", "", indent = indent);
        }
        JsonObject::Object(vs) => {
            if special {
                println!("{{");
            } else {
                println!("{:indent$}{{", "", indent = indent);
            }
            vs.iter().enumerate().for_each(|(i, (k, v))| {
                let k = if color { yellow(k) } else { k.to_string() };
                print!("{:indent$}\"{}\": ", "", k, indent = indent + 3);
                match &v {
                    JsonObject::Object(_) | JsonObject::Array(_) => {
                        do_output(v, color, indent + 3, true);
                    }
                    _ => {
                        do_output(v, color, indent + 3, false);
                    }
                };

                if i != vs.len() - 1 {
                    println!(",");
                } else {
                    println!();
                }
            });
            print!("{:indent$}}}", "", indent = indent);
        }
        JsonObject::Null => {
            let v = if color {
                red("null")
            } else {
                "null".to_string()
            };
            print!("{}", v);
        }
    }
}

fn main() {
    let (args, options): (Vec<String>, Vec<String>) = env::args()
        .into_iter()
        .skip(1)
        .partition(|str| !str.starts_with('-'));

    let mut color_output = false;
    let mut minimize_output = false;
    options
        .into_iter()
        .for_each(|option| match option.as_str() {
            "-h" | "--help" => {
                usage();
                exit(0);
            }
            "-c" | "--color" => {
                color_output = true;
            }
            "-m" | "--minimize" => {
                minimize_output = true;
            }
            _ => {
                eprintln!("error: an unrecognized option {}", option);
                usage();
                exit(1);
            }
        });
    if args.len() > 1 {
        eprintln!("error: the number of argument must be 0 or 1");
        usage();
        exit(1);
    }

    let input_json = if let Some(file_name) = args.first() {
        read_to_string(file_name)
            .ok()
            .unwrap_or_else(|| panic!("error: can't open a file {}", file_name))
    } else {
        let mut buffer = String::new();
        stdin()
            .read_to_string(&mut buffer)
            .expect("error: can't read a string from stdin");
        buffer
    };
    let json_value = json_prettier::parse(&input_json).expect("error: failed to parse json");
    if minimize_output {
        do_minimum_output(&json_value, color_output);
    } else {
        do_output(&json_value, color_output, 0, false);
    }
}
