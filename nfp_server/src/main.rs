use std::{io::{Read, Write}, path::Path};

const REQ_SIZE: usize = 1024*1024;

fn process_request(req: &String, size: usize, stream: &mut std::net::TcpStream) -> String {
    let req = req.splitn(2, '\n').collect::<Vec<&str>>();

    let mut req_header = if let Some(req_0) = req.get(0) {
        req_0.split(' ')
    } else {
        return "ERR BAD_REQ_1".to_string();
    };


    let Some(req_type) = req_header.next() else {
        return "ERR BAD_REQ_2".to_string();
    };

    println!("\"{req_type}\"");
    return match req_type.trim() {
        "GET" => {
            let mut file = if let Some(path) = req_header.next() {
                let path = Path::new(path.trim());
                if path.is_dir() {
                    return "ERR IS_DIR\n".to_string();
                }

                if !path.exists() {
                    return "ERR NO_SUCH_FILE\n".to_string();
                }

                std::fs::File::open(path).unwrap()
            } else {
                return "ERR NOT_ENOUGH_ARGS\n".to_string();
            };

            let mut buf = "OK FILE\n".to_string();

            if let Err(_) = file.read_to_string(&mut buf) {
                return "ERR COULDNT_READ_FILE\n".to_string();
            };

            buf
        }
        "PUT" => {
            if let Some(path) = req_header.next() {
                let path = Path::new(path.trim());
                let Ok(mut file) = std::fs::File::create_new(path) else {
                    return "ERR FILE_EXISTS\n".to_string();
                };

                let Some(req_body) = req.get(1) else {
                    return "ERR MISSING_BODY\n".to_string();
                };

                if let Err(_) = file.write_all(req_body.as_bytes()) {
                    return "ERR COULDNT_WRITE_TO_FILE".to_string();
                }

                "OK PUT\n".to_string()
            } else {
                "ERR NOT_ENOUGH_ARGS\n".to_string()
            }
        }
        "RM" => {
            if let Some(path) = req_header.next() {
                let path = Path::new(path.trim());
                if !path.exists() {
                    return "ERR NO_SUCH_FILE\n".to_string();
                }

                if path.is_dir() {
                    return "ERR IS_DIR\n".to_string();
                }

                if let Err(_) = std::fs::remove_file(path) {
                    return "ERR COULDNT_REMOVE_FILE\n".to_string();
                }

                "OK REMOVED\n".to_string()
            } else {
                "ERR NOT_ENOUGH_ARGS\n".to_string()
            }
        }
        "RMR" => {
            if let Some(path) = req_header.next() {
                let path = Path::new(path.trim());
                if !path.exists() {
                    return "ERR NO_SUCH_DIR\n".to_string();
                }

                if path.is_file() {
                    return "ERR NOT_DIR\n".to_string();
                }

                if let Err(_) = std::fs::remove_dir_all(path) {
                    return "ERR COULDNT_REMOVE_DIR\n".to_string();
                }

                "OK REMOVED\n".to_string()
            } else {
                "ERR NOT_ENOUGH_ARGS\n".to_string()
            }
        }
        "LIST" => {
            let path = if let Some(path) = req_header.next() {
                let path = Path::new(path.trim());
                if !path.is_dir() {
                    return "ERR NOT_DIR\n".to_string();
                }

                path
            } else {
                return "ERR NOT_ENOUGH_ARGS\n".to_string();
            };

            if !path.exists() {
                return "ERR NO_SUCH_DIR\n".to_string();
            }

            std::fs::read_dir(path).expect("ERR ????").map(|dir| dir.unwrap().path().to_str().unwrap().to_string()).fold("OK DIR\n".to_string(), |acc, el| acc + &el + "\n")
        }
        _ => {
            "ERR NO_SUCH_COMMAND\n".to_string()
        }
    }
}

fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:6969").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut request: [u8; REQ_SIZE] = [0; REQ_SIZE];

        
        let Ok(size) = stream.read(&mut request) else {
            println!("Couldn't read request\n");
            continue;
        };


        let request = request.iter().map(|x| *x as char).collect::<String>().trim().to_string();

        let response = process_request(&request, size, &mut stream);

        stream.write(response.as_bytes()).unwrap();

        println!(" New connection!\n\n Request:\n{request}\n\n Response:\n{response}");
    }
}
