use std::collections::HashMap;
use std::fmt::Write;
use std::{fmt, io};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> fmt::Display for HttpResponse<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let body = self.body();
        let body_len = body.len();
        write!(
            f,
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            self.version,
            self.status_code,
            self.status_text,
            self.headers_as_string(),
            body_len,
            body
        )
    }
}
impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code;
        };
        response.headers = match &headers {
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
            Some(_h) => headers,
        };
        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into(),
        };

        response.body = body;
        response
    }

    pub fn send_response(&self, write_stream: &mut impl io::Write) -> io::Result<()> {
        let response_string = format!("{}", self);
        let _ = write!(write_stream, "{}", response_string);

        Ok(())
    }

    fn headers_as_string(&self) -> String {
        let mut s = String::new();
        if let Some(headers) = &self.headers {
            for (k, v) in headers {
                let _ = write!(s, "{}:{}\r\n", k, v);
            }
        }
        s
    }

    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    }

    fn headers(&self) -> String {
        let Some(map) = self.headers.as_ref() else {
            return String::new();
        };

        let mut header_string = String::new();
        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            None => "",
            Some(b) => b.as_ref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new("200", None, Some("xxx".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1".into(),
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_response_struct_creation_400() {
        let response_actual = HttpResponse::new("400", None, Some("xxx".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1".into(),
            status_code: "400",
            status_text: "Bad Request",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_response_struct_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1".into(),
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxx".into()),
        };
        let http_string: String = format!("{}", response_expected);
        let actual_string =
            "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 3\r\n\r\nxxx";
        assert_eq!(http_string, actual_string);
    }
}
