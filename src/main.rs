use hyper::{Client, body::Buf};
//use tokio::io::{stdout, AsyncWriteExt as _};
use tokio::process::Command;
use serde_json::Value;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new(); 

    let uri = "http://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=en-US".parse()?;

    let res = client.get(uri).await?;

    println!("Response: {}", res.status());

    let body = hyper::body::aggregate(res).await?;

    let json:Value = serde_json::from_reader(body.reader())?;
    
    println!("Json: {}", json);
    
    let images = &json["images"][0];
    let url:&str = images["url"].as_str().unwrap();
    let uri2 = String::from("http://www.bing.com") + url; 
    let uri2 = uri2.parse()?; 
    println!("Result url: {}", uri2);

    let res = client.get(uri2).await?;
    let body = hyper::body::to_bytes(res).await?;

    //println!("Raw picture");
    //for byte in &body {
    //    print!("{}", byte);
    //}
    fs::write("bing_wallpaper.jpg", body)?;

    let mut child = Command::new("xfconf-query")
        .arg("-c")
        .arg("xfce4-desktop")
        .arg("-p")
        .arg("/backdrop/screen0/monitorHDMI-0/workspace1/last-image")
        .arg("-s")
        .arg("/home/kasper/repos/manjaro-bing-wall/bing_wallpaper.jpg")
        .spawn()
        .expect("failed to spawn");

    let status = child.wait().await?;
    println!("the command exited with: {}", status);
     
    //let output = Command::new("xfconf-query")
      //  .arg("-c")
      //  .arg("xfce4-desktop")
      //  .arg("-p")
      //  .arg("/backdrop/screen0/monitor0/workspace0/last-image")
      //  .output();

    //let output = output.await?;

    //println!("Command succesfull: {}", output.status.success());
    //for b in output.stdout {
    //    print!("{}", b);
    //}

    Ok(())
}

//xfconf-query -c xfce4-desktop -p /backdrop/screen0/monitor0/workspace0/last-image


