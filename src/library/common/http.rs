use std::env;
use std::io::{BufReader, BufWriter, Error, ErrorKind, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::collections::HashMap;
use std::fmt;

pub type URL = String;

#[derive(Clone, PartialEq, Debug)]
pub enum Method {
    Get,
    Post,
    Custom(String),
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Method::Get => write!(f, "GET"),
            Method::Post => write!(f, "POST"),
            Method::Custom(ref s) => write!(f, "{}", s),
        }
    }
}

pub struct Request {
    pub method: Method,
    pub host: URL,
    resource: URL,
    headers: HashMap<String, String>,
    body: Option<String>,
    pub timeout: Option<u64>,
    pub redirects: Vec<URL>,
}

impl Request {
    pub fn new<T: Into<URL>>(method: Method, url: T) -> Request {
        let (host, resource) = parse_url(url.into());
        Request {
            method,
            host,
            resource,
            headers: HashMap::new(),
            body: None,
            timeout: None,
            redirects: Vec::new(),
        }
    }

    pub fn with_header<T: Into<String>, U: Into<String>>(mut self, key: T, value: U) -> Request {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn with_body<T: Into<String>>(mut self, body: T) -> Request {
        let body = body.into();
        let body_length = body.len();
        self.body = Some(body);
        self.with_header("Content-Length", format!("{}", body_length))
    }

    pub fn with_json<T: serde::ser::Serialize>(
        mut self,
        body: &T,
    ) -> Result<Request, serde_json::Error> {
        self.headers.insert(
            "Content-Type".to_string(),
            "application/json; charset=UTF-8".to_string(),
        );
        Ok(self.with_body(serde_json::to_string(&body)?))
    }

    pub fn with_timeout(mut self, timeout: u64) -> Request {
        self.timeout = Some(timeout);
        self
    }

    pub fn send(self) -> Result<Response, Error> {
        Connection::new(self).send()
    }

    pub fn to_string(&self) -> String {
        let mut http = String::new();
        http += &format!(
            "{} {} HTTP/1.1\r\nHost: {}\r\n",
            self.method, self.resource, self.host
        );
        for (k, v) in &self.headers {
            http += &format!("{}: {}\r\n", k, v);
        }
        http += "\r\n";
        if let Some(ref body) = &self.body {
            http += body;
        }
        http
    }

    pub fn redirect_to(&mut self, url: URL) {
        self.redirects
            .push(create_url(&self.host, &self.resource));

        let (host, resource) = parse_url(url);
        self.host = host;
        self.resource = resource;
    }
}

pub struct Response {
    pub status_code: i32,
    pub reason_phrase: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub body_bytes: Vec<u8>,
}

impl Response {
    pub fn from_bytes(bytes: Vec<u8>) -> Response {
        let (status_code, reason_phrase) = parse_status_line(&bytes);
        let (headers, body_bytes) = parse_http_response_content(&bytes);
        Response {
            status_code,
            reason_phrase,
            headers,
            body: std::str::from_utf8(&body_bytes).unwrap_or("").to_owned(),
            body_bytes,
        }
    }

    pub fn json<'a, T>(&'a self) -> Result<T, serde_json::Error>
    where
        T: serde::de::Deserialize<'a>,
    {
        serde_json::from_str(&self.body)
    }
}

fn create_url(host: &str, resource: &str) -> URL {
    let prefix = "http://";
    return format!("{}{}{}", prefix, host, resource);
}

fn parse_url(url: URL) -> (URL, URL) {
    let mut first = URL::new();
    let mut second = URL::new();
    let mut slashes = 0;
    for c in url.chars() {
        if c == '/' {
            slashes += 1;
        } else if slashes == 2 {
            first.push(c);
        }
        if slashes >= 3 {
            second.push(c);
        }
    }
    if second.is_empty() {
        second += "/";
    }
    if !first.contains(':') {
        first += ":80";
    }
    (first, second)
}

pub fn parse_status_line(http_response: &[u8]) -> (i32, String) {
    let (line, _) = split_at(http_response, "\r\n");
    if let Ok(line) = std::str::from_utf8(line) {
        let mut split = line.split(' ');
        if let Some(code) = split.nth(1) {
            if let Ok(code) = code.parse::<i32>() {
                if let Some(reason) = split.next() {
                    return (code, reason.to_string());
                }
            }
        }
    }
    (503, "Server did not provide a status line".to_string())
}

fn parse_http_response_content(http_response: &[u8]) -> (HashMap<String, String>, Vec<u8>) {
    let (headers_text, body) = split_at(http_response, "\r\n\r\n");

    let mut headers = HashMap::new();
    let mut status_line = true;
    if let Ok(headers_text) = std::str::from_utf8(headers_text) {
        for line in headers_text.lines() {
            if status_line {
                status_line = false;
                continue;
            } else if let Some((key, value)) = parse_header(line) {
                headers.insert(key, value);
            }
        }
    }

    (headers, body.to_vec())
}

fn split_at<'a>(bytes: &'a [u8], splitter: &str) -> (&'a [u8], &'a [u8]) {
    for i in 0..bytes.len() - splitter.len() {
        if let Ok(s) = std::str::from_utf8(&bytes[i..i + splitter.len()]) {
            if s == splitter {
                return (&bytes[..i], &bytes[i + splitter.len()..]);
            }
        }
    }
    (bytes, &[])
}

pub fn parse_header(line: &str) -> Option<(String, String)> {
    if let Some(index) = line.find(':') {
        let key = line[..index].trim().to_string();
        let value = line[(index + 1)..].trim().to_string();
        Some((key, value))
    } else {
        None
    }
}

pub struct Connection {
    request: Request,
    timeout: Option<u64>,
}

impl Connection {
    pub fn new(request: Request) -> Connection {
        let timeout = request
            .timeout
            .or_else(|| match env::var("WB_TIMEOUT") {
                Ok(t) => t.parse::<u64>().ok(),
                Err(_) => None,
            });
        Connection { request, timeout }
    }

    pub fn send(self) -> Result<Response, Error> {
        let bytes = self.request.to_string().into_bytes();

        let tcp = create_tcp_stream(&self.request.host, self.timeout)?;

        let mut stream = BufWriter::new(tcp);
        stream.write_all(&bytes)?;

        let tcp = stream.into_inner()?;
        let mut stream = BufReader::new(tcp);
        // TODO: Simplify
        match read_from_stream(&mut stream, false) {
            Ok(response) => handle_redirects(self, Response::from_bytes(response)),
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock | ErrorKind::TimedOut => Err(Error::new(
                    ErrorKind::TimedOut,
                    format!(
                        "Request timed out! Timeout: {:?}",
                        stream.get_ref().read_timeout()
                    ),
                )),
                _ => Err(err),
            },
        }
    }
}

fn handle_redirects(connection: Connection, response: Response) -> Result<Response, Error> {
    let status_code = response.status_code;
    match status_code {
        301 | 302 | 303 | 307 => {
            let url = response.headers.get("Location");
            if url.is_none() {
                return Err(Error::new(
                    ErrorKind::Other,
                    "'Location' header missing in redirect.",
                ));
            }
            let url = url.unwrap();
            let mut request = connection.request;

            if request.redirects.contains(&url) {
                Err(Error::new(ErrorKind::Other, "Infinite redirection loop."))
            } else {
                request.redirect_to(url.clone());
                if status_code == 303 {
                    match request.method {
                        Method::Post => {
                            request.method = Method::Get;
                        }
                        _ => {}
                    }
                }

                request.send()
            }
        }

        _ => Ok(response),
    }
}

fn create_tcp_stream<A>(host: A, timeout: Option<u64>) -> Result<TcpStream, Error>
where
    A: ToSocketAddrs,
{
    let stream = TcpStream::connect(host)?;
    if let Some(secs) = timeout {
        let dur = Some(Duration::from_secs(secs));
        stream.set_read_timeout(dur)?;
        stream.set_write_timeout(dur)?;
    }
    Ok(stream)
}

fn read_from_stream<T: Read>(stream: T, head: bool) -> Result<Vec<u8>, Error> {
    let mut response = Vec::new();
    let mut response_length = None;
    let mut chunked = false;
    let mut expecting_chunk_length = false;
    let mut byte_count = 0;
    let mut last_newline_index = 0;
    let mut blank_line = false;
    let mut status_code = None;

    for byte in stream.bytes() {
        let byte = byte?;
        response.push(byte);
        byte_count += 1;
        if byte == b'\n' {
            if status_code.is_none() {
                status_code = Some(parse_status_line(&response).0);
            }

            if blank_line {
                if let Some(code) = status_code {
                    if head || code / 100 == 1 || code == 204 || code == 304 {
                        response_length = Some(response.len());
                    }
                }
                if response_length.is_none() {
                    if let Ok(response_str) = std::str::from_utf8(&response) {
                        let len = get_response_length(response_str);
                        response_length = Some(len);
                        if len > response.len() {
                            response.reserve(len - response.len());
                        }
                    }
                }
            } else if let Ok(new_response_length_str) =
                std::str::from_utf8(&response[last_newline_index..])
            {
                if expecting_chunk_length {
                    expecting_chunk_length = false;

                    if let Ok(n) = usize::from_str_radix(new_response_length_str.trim(), 16) {
                        response.truncate(last_newline_index);
                        byte_count = last_newline_index;
                        if n == 0 {
                            break;
                        } else {
                            response_length = Some(byte_count + n + 2);
                        }
                    }
                } else if let Some((key, value)) = parse_header(new_response_length_str) {
                    if key.trim() == "Transfer-Encoding" && value.trim() == "chunked" {
                        chunked = true;
                    }
                }
            }

            blank_line = true;
            last_newline_index = byte_count;
        } else if byte != b'\r' {
            blank_line = false;
        }

        if let Some(len) = response_length {
            if byte_count >= len {
                if chunked {
                    expecting_chunk_length = true;
                } else {
                    break;
                }
            }
        }
    }

    Ok(response)
}

fn get_response_length(response: &str) -> usize {
    let mut byte_count = 0;
    for line in response.lines() {
        byte_count += line.len() + 2;
        if line.starts_with("Content-Length: ") {
            if let Ok(length) = line[16..].parse::<usize>() {
                byte_count += length;
            }
        }
    }
    byte_count
}

pub fn create_request<T: Into<URL>>(method: Method, url: T) -> Request {
    Request::new(method, url.into())
}

pub fn get<T: Into<URL>>(url: T) -> Request {
    create_request(Method::Get, url)
}

pub fn post<T: Into<URL>>(url: T) -> Request {
    create_request(Method::Post, url)
}
