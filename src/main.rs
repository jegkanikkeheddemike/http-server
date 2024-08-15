use std::{
    fs::read_to_string,
    io::{BufReader, BufWriter, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    path::PathBuf,
    thread,
};

use http::{parse_http_then, Response};
mod http;

fn main() {
    let server = TcpListener::bind("0.0.0.0:4000").unwrap();
    println!("Running on http://0.0.0.0:4000/");

    loop {
        match server.accept() {
            Ok((stream, addr)) => {
                thread::spawn(move || read_stream(stream, addr));
            }
            Err(err) => eprintln!("{err:#?}"),
        }
    }
}

fn read_stream(stream: TcpStream, addr: SocketAddr) {
    let mut writer = BufWriter::new(stream.try_clone().unwrap());
    let mut reader = BufReader::new(stream);

    loop {
        let response = parse_http_then(&mut reader, addr, |request| {
            // let keep_alive = request.headers.get("Connection") == Some(&"keep-alive");
            let keep_alive = false;

            let mut file_path = PathBuf::from(format!("./public{}", request.path));

            if file_path.is_dir() {
                file_path.push("index.html");
            }

            let Ok(file) = read_to_string(&file_path) else {
                return Response {
                    status: 404,
                    body: read_to_string("./public/404.html")
                        .unwrap_or_else(|_| "404 missing".into()),
                    keep_alive,
                };
            };
            return Response {
                status: 200,
                body: file,
                keep_alive,
            };
        });

        writeln!(writer, "HTTP/1.1 {} OK", response.status).unwrap();
        writeln!(writer, "Content-Type: text/html").unwrap();
        writeln!(writer, "Content-Length: {}", response.body.len()).unwrap();
        writeln!(writer, "").unwrap();
        writer.write_all(response.body.as_bytes()).unwrap();

        if !response.keep_alive {
            break;
        }
    }
}
