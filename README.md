# What is this project about?
http_req_no_std_win is as the name already says, a library which is no_std compatible and made for windows
the goal of this project is to eleminate large binary sizes by using no_std, the library is kept relatively simple

example usage
```rs
use http_req_no_std_win::request::{ClientBuilder, Request, RequestType};

fn main() {
    let body = r#"{"name":"morpheus","job":"jobless"}"#.as_bytes().to_vec();
    
    let client_builder = ClientBuilder::new()
        .url("https://reqres.in/api/users/2")
        .request_type(RequestType::GET)
        //.body(body)
        .build();
    
    let request = Request { client: client_builder };

    match request.send() {
        Ok(response) => println!("Response: {:?}", response),
        Err(error) => println!("Request failed with error code: {:?}", error),
    }
}
```
