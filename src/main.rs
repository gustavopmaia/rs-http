#![allow(unused_must_use)]
pub mod requests;
use requests::get;
use requests::post;

#[tokio::main]
async fn main(){
  get("https://httpbin.org/get").await;  
  post("https://httpbin.org/post").await;
}



