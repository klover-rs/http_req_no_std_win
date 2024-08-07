#![no_std]

pub mod request;
mod error;

pub extern crate alloc as std_alloc;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use request::{ClientBuilder, Request, RequestType};

    use super::*;

    #[test]
    fn get_request() {
        let client_builder = ClientBuilder::new()
            .url("https://reqres.in/api/users")
            .request_type(RequestType::GET)
            .build();

        let request = Request { client: client_builder };

        request.send().unwrap();
    }

    #[test]
    pub fn post_request() {
        let body = r#"{"name":"your_mom","job":"gay"}"#.as_bytes().to_vec();
    
        let client_builder = ClientBuilder::new()
            .url("https://reqres.in/api/users")
            .request_type(RequestType::POST)
            .body(body)
            .build();
    
        let request = Request { client: client_builder };

        request.send().unwrap();
    }

    #[test]
    pub fn put_request() {
        let body = r#"{"name":"morpheus","job":"gay"}"#.as_bytes().to_vec();
    
        let client_builder = ClientBuilder::new()
            .url("https://reqres.in/api/users/2")
            .request_type(RequestType::PUT)
            .body(body)
            .build();
    
        let request = Request { client: client_builder };

        request.send().unwrap();
    }

    #[test]
    pub fn delete_request() {
        let client_builder = ClientBuilder::new()
        .url("https://reqres.in/api/users/2")
        .request_type(RequestType::DELETE)
        .build();
    
        let request = Request { client: client_builder };

        request.send().unwrap(); 
    }
}
