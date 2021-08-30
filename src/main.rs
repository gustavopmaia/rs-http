#![allow(unused_must_use)]
pub mod requests;
pub mod ip;

use requests::{get, post, put, delete};

#[tokio::main]
async fn main(){
  get("https://httpbin.org/get").await;  
  post("https://httpbin.org/post").await;
  put("https://httpbin.org/put").await;
  delete("https://httpbin.org/delete").await;
}



