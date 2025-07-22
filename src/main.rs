use std::thread::Builder;
use compio::{runtime::RuntimeBuilder};
use cyper::{Client, ClientBuilder};

fn main() {
    let rt = RuntimeBuilder::new().build().unwrap();
    rt.block_on(async move {
        let client = ClientBuilder::new().use_rustls_default().build();
        let response = client
            .get("https://www.example.com/")
            .unwrap()
            .send()
            .await
            .unwrap();
        println!("{}", response.text().await.unwrap());
    });
}
