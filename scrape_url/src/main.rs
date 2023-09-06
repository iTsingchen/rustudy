use std::fs;

async fn scrape_url(url: &str, output: &str) {
    println!("Fetching url: {}", url);
    let body = reqwest::get(url).await.unwrap().text().await.unwrap();

    println!("Converting html to markdown...");

    let md = html2md::parse_html(body.as_str());

    fs::write(output, md.as_bytes()).unwrap();
}

#[tokio::main]
pub async fn main() {
    let url = "https://www.rust-lang.org/";
    let output_dir = "target/output";
    let file_path = output_dir.to_owned() + "/rust.md";

    fs::create_dir(output_dir).unwrap();
    scrape_url(url, file_path.as_str()).await;

    println!("Yeah!!!");
}
