use std::fmt::{Debug, Formatter};

use itertools::Itertools;

use crate::tcp::request::Parameter::{Form, Json, Nothing};

type Key = String;
type Val = String;

#[derive(Eq, PartialEq)]
pub struct Request {
    pub method: String,
    pub target: String,
    pub version: String,
    pub headers: Vec<(Key, Val)>,
    pub parameter: Parameter,
}

impl Debug for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Request {{ {} {} {}, [{}], {} }}",
            self.method,
            self.target,
            self.version,
            self.headers.iter().map(|(k, v)| format!("{}: {}", k, v)).join(", "),
            match &self.parameter {
                Form { values } => values.iter().map(|(k, v)| format!("{}={}", k, v)).join("&"),
                Json { value } => value.clone(),
                Nothing => String::from(""),
            }
        )
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Parameter {
    Form { values: Vec<(Key, Val)> },
    Json { value: String },
    Nothing,
}

pub fn parse_request<S: Into<String>>(raw: S) -> Request {
    let raw = raw.into();
    let lines = raw.split('\n').map(|line| line.trim()).collect::<Vec<&str>>();

    let crlf = lines.iter().position(|&line| line.is_empty()).unwrap();

    let start_line = &lines[0];
    let header_lines = &lines[1..crlf];
    let body_line = &lines[crlf + 1];

    let (method, target, version) = parse_start_line(start_line);
    let headers = parse_header_lines(header_lines);
    let parameter = match detect_content_type(&headers) {
        Some("application/x-www-form-urlencoded") => parse_form(body_line),
        Some("application/json") => Json { value: body_line.to_string() },
        _ => Nothing,
    };

    Request { method, target, version, headers, parameter }
}

fn parse_start_line(line: &str) -> (String, String, String) {
    let sp = line.split(' ').collect_vec();
    (sp[0].to_string(), sp[1].to_string(), sp[2].to_string())
}

fn parse_header_lines(lines: &[&str]) -> Vec<(Key, Val)> {
    lines
        .iter()
        .map(|line| {
            let sp = line.split(':').collect_vec();
            (sp[0].trim().to_ascii_lowercase(), sp[1].trim().to_string())
        })
        .collect()
}

fn detect_content_type(headers: &[(Key, Val)]) -> Option<&str> {
    headers.iter().filter(|&(k, _)| k == &"content-type".to_string()).map(|(_, v)| v.as_str()).next()
}

fn parse_form(line: &str) -> Parameter {
    let values = line
        .split('&')
        .map(|line| {
            let sp = line.split('=').collect_vec();
            (sp[0].to_string(), sp[1].trim_matches('\0').to_string())
        })
        .collect();
    Form { values }
}

#[cfg(test)]
mod tests {
    use crate::tcp::request::Parameter::{Form, Json, Nothing};
    use crate::tcp::request::{parse_request, Request};

    #[test]
    fn test_form_body() {
        let act = parse_request(
            "POST /foo/bar HTTP/1.1
Accept: */*
Content-Length: 16
Content-Type: application/x-www-form-urlencoded

name=John&age=39",
        );

        let exp = Request {
            method: String::from("POST"),
            target: String::from("/foo/bar"),
            version: String::from("HTTP/1.1"),
            headers: vec![
                (String::from("accept"), String::from("*/*")),
                (String::from("content-length"), String::from("16")),
                (String::from("content-type"), String::from("application/x-www-form-urlencoded")),
            ],
            parameter: Form {
                values: vec![(String::from("name"), String::from("John")), (String::from("age"), String::from("39"))],
            },
        };

        assert_eq!(exp, act);
    }

    #[test]
    fn test_json_body() {
        let act = parse_request(
            r#"POST /foo/bar HTTP/1.1
Accept: */*
Content-Length: 27
content-type: application/json

{"name": "John", "age": 39}"#,
        );

        let exp = Request {
            method: String::from("POST"),
            target: String::from("/foo/bar"),
            version: String::from("HTTP/1.1"),
            headers: vec![
                (String::from("accept"), String::from("*/*")),
                (String::from("content-length"), String::from("27")),
                (String::from("content-type"), String::from("application/json")),
            ],
            parameter: Json { value: String::from(r#"{"name": "John", "age": 39}"#) },
        };

        assert_eq!(exp, act);
    }

    #[test]
    fn test_no_body() {
        let act = parse_request(
            "GET /foo/bar HTTP/1.1
Accept: */*

",
        );

        let exp = Request {
            method: String::from("GET"),
            target: String::from("/foo/bar"),
            version: String::from("HTTP/1.1"),
            headers: vec![(String::from("accept"), String::from("*/*"))],
            parameter: Nothing,
        };

        assert_eq!(exp, act);
    }
}
