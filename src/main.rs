#![allow(unused_must_use)]
pub mod requests;
use requests::get;
use requests::post;
use requests::put;
use requests::delete;

#[tokio::main]
async fn main(){
  get("https://httpbin.org/get").await;  
  post("https://httpbin.org/post").await;
  put("https://httpbin.org/put").await;
  delete("https://httpbin.org/delete").await;
}



