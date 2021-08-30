use crate::ip::Ip;

pub async fn get(url: &str) -> surf::Result<surf::Response> {
  let mut res = surf::get(url).await?;
  dbg!(res.body_string().await?);

  Ok(res)
}

pub async fn put(url: &str) -> surf::Result<surf::Response> {

  let uri = url;
  let data = &Ip::new("129.0.0.1");
  let mut res = surf::put(uri).body_json(data)?.await?;
  assert_eq!(res.status(), 200);
  dbg!(res.body_string().await?);

  Ok(res)
}

pub async fn post(url: &str) -> surf::Result<surf::Response> {

  let uri = url;
  let data = &Ip::new("129.0.0.1");
  let mut res = surf::post(uri).body_json(data)?.await?;
  assert_eq!(res.status(), 200);
  dbg!(res.body_string().await?);

  Ok(res)
}

pub async fn delete(url: &str) -> surf::Result<surf::Response> {

  let uri = url;
  let data = &Ip::new("129.0.0.1");
  let mut res = surf::delete(uri).body_json(data)?.await?;
  assert_eq!(res.status(), 200);
  dbg!(res.body_string().await?);

  Ok(res)
}