// URLParser Imports -> Checks whether a URL form is valid
extern crate url as URLParser;

// Reqwest Imports
use error_chain::{error_chain, ChainedError};
use std::io::Read;
use reqwest::blocking::Response;

   

mod request;

// Checks a URL, if valid, a request is performed, otherwise an error is returned.
pub fn try_request_from_url<'a>(url: &'a str) -> Result<Response, RequestError> {
    match check_url(url) {
        Ok(_) => request::request_from_url(url),
        Err(e) => Err(e)

    }

}


fn check_url(url: &str) -> Result<(), RequestError> {
    // Check whether the URL is valid.
    if let Err(ParseError) = URLParser::Url::parse(url) {
        return Err(request_error(Fault::URLParse, ParseError.to_string()))
    }
    Ok(())

}

pub fn parse_response_url(res_object: Response) {
    //res_object.headers()
        
}


// Constructs a RequestError from the given parameters.
// !!! This can potentially be done better
fn request_error(fault: Fault, msg: String) -> RequestError {
    RequestError { fault: (fault), msg: (msg) }
}   


// pub struct ParsedURL {
//     pub host: Option<Box<str>>,
//     pub path: Option<Box<str>>,
//     pub server: Option<Box<str>>,
//     pub port: Option<u16>,
//     pub date: Option<Box<str>>



// }

// Error Struct to allow for message passing.
pub struct RequestError {
    pub fault: Fault,
    pub msg: String
}

// Type of Fault to be used in a RequestError
#[derive(Debug)]
pub enum Fault {
    URLParse,
    HttpResponse,
    BodyParse
}
