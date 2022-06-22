// Serde Imports -> Deserializing URL
use serde::{Deserialize};

// Reqwest Imports -> HTTP Requests
use error_chain::{error_chain, ChainedError};
use std::io::Read;
use reqwest::blocking::Response;
use reqwest::Url;

// Local Imports
use crate::http::{RequestError, Fault, request_error};

// Function to perform a GET request from a URL
pub fn request_from_url<'a>(url: &'a str) -> Result<Response, RequestError> {

    // Attempt a GET request using the URL
    let mut res = match reqwest::blocking::get(url) {
        Ok(val) => val,
        Err(e) => return Err(request_error(Fault::HttpResponse, e.to_string()))
        
    };
    println!("{:?}", res.url());
    
    Ok(res)


    

    // Read body of response object into string.
    // let mut body = String::new();
    // match res.read_to_string(&mut body)  {
    //     Ok(val) => val,
    //     Err(e) => return Err(request_error(Fault::BodyParse, e.to_string())) 
    // };  

    //request::parse_host_string(res.url().host());
    
    
    // Return ok if no errors occured.

}


pub fn parse_host_string(url_object: Url) {
    

}