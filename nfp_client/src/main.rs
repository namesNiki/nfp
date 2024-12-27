use std::{fmt::format, io::{Read, Write}, path::Path};

use clap::{Parser, Subcommand};


#[derive(clap::Parser, Debug)]
struct Args {
    ip: String,

    #[arg(short, long, default_value_t = 6969)]
    port: u16,

    #[arg(short, long)]
    file: bool,

    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Cat {
        path_server: String
    },
    Echo {
        path_server: String,
        data: String
    },
    Ls {
        path_server: Option<String>
    },
    Rm {
        path_server: String
    },
    Rmr {
        path_server: String
    },
    Cp {
        path_client: String,
        path_server: String
    }
}

fn check_for_errors_in_response<F: Fn(&str) -> &str>(resp_result: &str, resp_header: &mut std::str::Split<'_, char>, reason_match: F)  {
    if resp_result == "ERR" {
        let Some(resp_reason) = resp_header.next() else {
            println!("Error, bad response from server");
            std::process::exit(1);
        };

        let reason = reason_match(resp_reason.trim());
        println!("Error, {reason}");
        std::process::exit(1);
    }

}

fn process_response(resp: &String, command: Commands) {
    let resp = resp.splitn(2, '\n').collect::<Vec<&str>>();

    let mut resp_header = if let Some(req_0) = resp.get(0) {
        req_0.split(' ')
    } else {
        println!("Error, bad response from server");
        std::process::exit(1);
    };

    let Some(resp_result) = resp_header.next() else {
        println!("Error, bad response from server");
        std::process::exit(1);
    };

    match command {
        Commands::Cat {..} => {
            check_for_errors_in_response(resp_result, &mut resp_header, |resp| {
                match resp {
                    "IS_DIR" => "given path is a directory",
                    "NO_SUCH_FILE" => "given file does not exist",
                    "NOT_ENOUGH_ARGS" => "program didnt provide enough arguments to the server",
                    "COULDNT_READ_FILE" => "could not read the file",
                    _ => "not sure",
                }
            });

            let Some(resp_body) = resp.get(1) else {
                println!("Error, server didnt send the file back");
                std::process::exit(1);
            };

            println!("{resp_body}");
        },
        Commands::Ls { .. } => {
            check_for_errors_in_response(resp_result, &mut resp_header, |resp| {
                match resp {
                    "NOT_DIR" => "given path is not a directory",
                    "NO_SUCH_DIR" => "given directory does not exist",
                    "NOT_ENOUGH_ARGS" => "program didnt provide enough arguments to the server",
                    _ => "not sure",
                }
            });

            let Some(resp_body) = resp.get(1) else {
                println!("Error, server didnt send the file back");
                std::process::exit(1);
            };

            println!("{resp_body}");
        }
        Commands::Rm { .. } => {
            check_for_errors_in_response(resp_result, &mut resp_header, |resp| {
                match resp {
                    "IS_DIR" => "given path is a directory",
                    "NO_SUCH_FILE" => "given file does not exist",
                    "NOT_ENOUGH_ARGS" => "program didnt provide enough arguments to the server",
                    "COULDNT_REMOVE_FILE" => "could not remove the file",
                    _ => "not sure",
                }
            });

            println!("File removed succesfully from the server");
        }
        Commands::Rmr { .. } => {
            check_for_errors_in_response(resp_result, &mut resp_header, |resp| {
                match resp {
                    "NOT_DIR" => "given path is not a directory",
                    "NO_SUCH_DIR" => "given directory does not exist",
                    "NOT_ENOUGH_ARGS" => "program didnt provide enough arguments to the server",
                    "COULDNT_REMOVE_DIR" => "could not remove directory",
                    _ => "not sure",
                }
            });

            println!("Directory removed succesfully from the server");
        }
        Commands::Echo { .. } | Commands::Cp { .. } => {
            check_for_errors_in_response(resp_result, &mut resp_header, |resp| {
                match resp {
                    "FILE_EXISTS" => "file already exists",
                    "MISSING_BODY" => "the data to put into the server wasnt provided by the program",
                    "COULDNT_WRITE_TO_FILE" => "couldn't write to the file",
                    "NOT_ENOUGH_ARGS" => "program didnt provide enough arguments to the server",
                    _ => "not sure",
                }
            });

            println!("File succesfully created on the server");
        }
    }
}

fn write_request(request: String, command: Commands, mut stream: std::net::TcpStream) {
    if let Err(_) = stream.write_all(request.as_bytes()) {
        println!("Error, couldn't write the request");
        std::process::exit(1);
    }

    let mut buf = String::new();

    if let Err(_) = stream.read_to_string(&mut buf) {
        println!("Error, couldn't read the response");
        std::process::exit(1);
    }

    process_response(&buf, command);
}

fn main() {
    let args = Args::parse();

    let Ok(stream) = std::net::TcpStream::connect(args.ip+":"+args.port.to_string().as_str()) else {
        println!("Error, couldn't find server");
        std::process::exit(1);
    };

    match args.command.clone() {
        Commands::Cat {path_server} => {
            let request = format!("GET {path_server} ");
            write_request(request, args.command, stream);
        }
        Commands::Ls { path_server } => {
            let path_server = if let Some(path) = path_server {
               path
            } else {
               ".".to_string()
            };
            let request = format!("LIST {path_server} ");
            write_request(request, args.command, stream);
        }
        Commands::Echo { path_server, data } => {
            let request = format!("PUT {path_server}\n{data} ");
            write_request(request, args.command, stream);
        }
        Commands::Rm { path_server } => {
            let request = format!("RM {path_server} ");
            write_request(request, args.command, stream);
        }
        Commands::Rmr { path_server } => {
            let request = format!("RM {path_server} ");
            write_request(request, args.command, stream);
        }
        Commands::Cp { path_server, path_client } => {
            let filename = Path::new(&path_client);

            if !filename.exists() {
                println!("Error, no such file");
                std::process::exit(1);
            }

            if filename.is_dir() {
                println!("Error, couldn't open file, provided filename is a directory");
                std::process::exit(1);
            }

            let mut buf = String::new();

            let mut file = std::fs::File::open(filename).unwrap();

            if let Err(_) = file.read_to_string(&mut buf) {
                println!("Error, couldn't read file");
                std::process::exit(1);
            }

            let request = format!("PUT {path_server}\n{buf} ");
            write_request(request, args.command, stream);
        }
    }
}
