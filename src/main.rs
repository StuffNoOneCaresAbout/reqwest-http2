#[tokio::main]
async fn main() {
    let resp = reqwest::get("https://icanhazip.com/cdn-cgi/trace")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{}", resp);
}
