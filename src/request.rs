
use core::ptr::null_mut;
use std_alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    ffi::CString,
    vec::Vec
};
use winapi::um::wininet::{
    HttpOpenRequestA, HttpSendRequestA, InternetCloseHandle, InternetConnectA, InternetOpenA, InternetReadFile, INTERNET_DEFAULT_HTTPS_PORT, INTERNET_FLAG_RELOAD, INTERNET_FLAG_SECURE, INTERNET_OPEN_TYPE_DIRECT, INTERNET_SERVICE_HTTP  
};
use winapi::um::errhandlingapi::GetLastError;

use crate::error::{ReqResult, RequestError};

#[derive(Debug, Clone)]
pub enum RequestType {
    GET,
    POST,
    DELETE,
    PUT,
    HEAD,
    PATCH,
    OPTIONS,
    CONNECT,
    TRACE
}

impl RequestType {
    pub fn to_str(res_type: RequestType) -> &'static str {
        match res_type {
            RequestType::CONNECT => "CONNECT",
            RequestType::DELETE => "DELETE",
            RequestType::GET => "GET",
            RequestType::HEAD => "HEAD",
            RequestType::OPTIONS => "OPTIONS",
            RequestType::PATCH => "PATCH",
            RequestType::POST => "POST",
            RequestType::PUT => "PUT",
            RequestType::TRACE => "TRACE",
        }
    }

    pub fn to_res_type(res_type: &str) -> RequestType {
        match res_type {
            "CONNECT" => RequestType::CONNECT,
            "DELETE" => RequestType::DELETE,
            "GET" => RequestType::GET,
            "HEAD" => RequestType::HEAD,
            "OPTIONS" => RequestType::OPTIONS,
            "PATCH" => RequestType::PATCH,
            "POST" => RequestType::POST,
            "PUT" => RequestType::PUT,
            "TRACE" => RequestType::TRACE,
            _ => RequestType::GET
        }
    }
}

pub struct ClientBuilder {
    request_type: RequestType,
    url: String,
    headers: BTreeMap<String, String>,
    body: Option<Vec<u8>>
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            request_type: RequestType::GET,
            url: String::new(),
            headers: BTreeMap::new(),
            body: None
        }
    }
    
    pub fn request_type(mut self, request_type: RequestType) -> Self {
        self.request_type = request_type;
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn build(self) -> Client {
        Client {
            request_type: self.request_type,
            url: self.url,
            headers: self.headers,
            body: self.body
        }
    }
}

pub struct Client {
    pub request_type: RequestType,
    pub url: String,
    pub headers: BTreeMap<String, String>,
    pub body: Option<Vec<u8>>
}

pub struct Request {
    pub client: Client,
}

impl Request {
    pub fn send(&self) -> ReqResult<String> {
        unsafe {
            let lpsz_agent = CString::new("WinINet client").unwrap();

            let h_internet = InternetOpenA(
                lpsz_agent.as_ptr(),
                INTERNET_OPEN_TYPE_DIRECT,
                null_mut(),
                null_mut(),
                0
            );

            if h_internet.is_null() {
                return Err(RequestError::new("InternetOpen failed", GetLastError()));
            }
            
            let (host, path) = self.parse_url(&self.client.url).unwrap();
            let lpsz_server_name = CString::new(host).unwrap();
            let lpsz_object_name = CString::new(path).unwrap();

            let h_connect = InternetConnectA(
                h_internet,
                lpsz_server_name.as_ptr(),
                INTERNET_DEFAULT_HTTPS_PORT as u16, // test also out for http requests
                null_mut(),
                null_mut(),
                INTERNET_SERVICE_HTTP,
                0,0
            );

            if h_connect.is_null() {
                InternetCloseHandle(h_internet);
                return Err(RequestError::new("InternetConnect failed", GetLastError()));
            }


            let res_type = CString::new(RequestType::to_str(self.client.request_type.clone())).unwrap();
            let h_request = HttpOpenRequestA(
                h_connect,
                res_type.as_ptr(),
                lpsz_object_name.as_ptr(),
                null_mut(),
                null_mut(),
                null_mut(),
                INTERNET_FLAG_RELOAD | INTERNET_FLAG_SECURE,
                0
            );

            if h_request.is_null() {
                InternetCloseHandle(h_connect);
                InternetCloseHandle(h_internet);
                return Err(RequestError::new("HttpOpenRequest failed", GetLastError()));
            }

            let body_data = self.client.body.as_deref().unwrap_or(&[]);
            let body_len = body_data.len() as u32;
            
            if HttpSendRequestA(
                h_request,
                null_mut(),
                0,
                body_data.as_ptr() as *mut winapi::ctypes::c_void,
                body_len
            ) == 0 {
                InternetCloseHandle(h_request);
                InternetCloseHandle(h_connect);
                InternetCloseHandle(h_internet);
                return Err(RequestError::new("HttpSendRequest failed", GetLastError()));
            }

            let mut data: Vec<u8> = Vec::new();
            let mut buffer: [u8; 1024] = [0; 1024];
            let mut bytes_read: u32 = 0;


            loop {
                if InternetReadFile(h_request, buffer.as_mut_ptr() as *mut _, buffer.len() as u32, &mut bytes_read) == 0 {
                    InternetCloseHandle(h_request);
                    InternetCloseHandle(h_connect);
                    InternetCloseHandle(h_internet);
                    return Err(RequestError::new("InternetReadFile failed", GetLastError()));
                }

                if bytes_read == 0 {
                    break;
                }

                data.extend_from_slice(&buffer[..bytes_read as usize]);
            }

            InternetCloseHandle(h_request);
            InternetCloseHandle(h_connect);
            InternetCloseHandle(h_internet);

            match String::from_utf8(data) {
                Ok(result) => Ok(result),
                Err(_) => Err(RequestError::new("Failed to convert result to utf8 string", GetLastError())),
            }
            
        }
    }

    fn parse_url(&self, url: &str) -> Result<(String, String), u32> {
        if let Some(pos) = url.find("://") {
            let pos = pos + 3;

            if let Some(path_pos) = url[pos..].find('/') {
                let host = url[pos..pos + path_pos].to_string();
                let path = url[pos + path_pos..].to_string();
                return Ok((host, path));
            }
        }
        Err(1)
    }
}

impl Client {
    pub fn get(url: &str) -> Request {
        let client = ClientBuilder::new().url(url).build();
        Request { client }
    }
}