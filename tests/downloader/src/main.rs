use marie_core::{downloader::images::ImageDownloader, net::HttpClient};

#[tokio::main]
async fn main() {
    let client = HttpClient::new();
    let downloader = ImageDownloader::new(
        "fuck".into(),
        "http://localhost:8080".into(),
        client,
    );

    const EXPECTED: [&str; 10] = [
        "generated/image_1.png",
        "generated/image_2.png",
        "generated/image_3.png",
        "generated/image_4.png",
        "generated/image_5.png",
        "generated/image_6.png",
        "generated/image_7.png",
        "generated/image_8.png",
        "generated/image_9.png",
        "generated/image_10.png",
    ];

    let urls = downloader.download().await.unwrap();

    assert_eq!(urls.len(), 10);
    assert_eq!(urls, EXPECTED);
}
