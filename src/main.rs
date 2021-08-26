#![allow(unused_must_use)]
pub mod requests;
use requests::get;
use requests::post;

#[tokio::main]
async fn main(){
  get().await;  
  post().await;
}



