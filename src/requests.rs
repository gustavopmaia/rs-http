use serde::{Deserialize, Serialize};

pub async fn get() -> surf::Result<surf::Response> {
  let mut res = surf::get("https://httpbin.org/get").await?;
  dbg!(res.body_string().await?);

  Ok(res)
}

#[derive(Deserialize, Serialize)]
struct Ip {
    ip: String
}

pub async fn post() -> surf::Result<surf::Response> {
  #[derive(Deserialize, Serialize)]
    struct Ip {
        ip: String
    }

    let uri = "https://httpbin.org/post";
    let data = &Ip { ip: "129.0.0.1".into() };
    let mut res = surf::post(uri).body_json(data)?.await?;
    assert_eq!(res.status(), 200);
    dbg!(res.body_string().await?);

    Ok(res)
}