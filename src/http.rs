use core::str;
use std::{collections::HashMap, io::BufRead, net::SocketAddr};

#[allow(unused)]
#[derive(Debug)]
pub struct Request<'buf> {
    pub method: &'buf str,
    pub path: &'buf str,
    pub version: &'buf str,
    pub addr: SocketAddr,
    pub headers: HashMap<&'buf str, &'buf str>,
    pub body: Option<Vec<u8>>,
}
#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub body: String,
    pub keep_alive: bool,
}

pub fn parse_http_then<R: BufRead, T: Fn(&Request) -> Response>(
    mut reader: R,
    addr: SocketAddr,
    then: T,
) -> Response {
    let mut pre_buffer = String::new();
    reader.read_line(&mut pre_buffer).unwrap();

    let [method, path, version] = pre_buffer
        .split_whitespace()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let mut buffer = String::new();

    let mut headers = Vec::new();

    let mut prev_len = 0;
    while let Ok(count) = reader.read_line(&mut buffer) {
        let post_len = buffer.len();
        if count == 0 {
            break;
        }

        let Some(rel_split_location) = buffer[prev_len..post_len].find(": ") else {
            break;
        };
        let split_location = prev_len + rel_split_location;

        headers.push((
            (prev_len, split_location),
            (split_location + 2, post_len - 2),
        ));
        prev_len = post_len;
    }
    let headers = headers
        .into_iter()
        .map(|((k1, k2), (v1, v2))| (&buffer[k1..k2], &buffer[v1..v2]))
        .collect::<HashMap<_, _>>();

    let mut body = None;

    if let Some(content_len_raw) = headers.get("Content-Length") {
        let content_len: usize = content_len_raw.parse().unwrap();
        let mut body_buffer = vec![0u8; content_len];
        reader.read_exact(&mut body_buffer).unwrap();

        body = Some(body_buffer);
    }

    let request = Request {
        method,
        path,
        version,
        addr,
        headers,
        body,
    };

    then(&request)
}
