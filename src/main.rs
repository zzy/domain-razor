mod email;

use std::fs;
use std::time::Duration;
use std::thread::sleep;

use dotenv::dotenv;
use fantoccini::{ClientBuilder, Locator};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let sleep_duration = dotenv::var("sleep_duration")
        .expect("Expected sleep_duration to be set in env!")
        .parse::<u64>()?;

    // Connect to webdriver instance that is listening on port 4444
    let mut client =
        ClientBuilder::native().connect("http://localhost:4444").await?;

    let file_contents =
        fs::read_to_string("./domains.txt").expect("读取文件错误");
    let domains = file_contents.split(",");

    for domain in domains {
        let base_url = "https://www.huaweicloud.com/whois/whois.html?domain=";
        let domain_url = base_url.to_string() + domain.trim();
        println!("{:?}", &domain_url);

        client.goto(&domain_url).await?;
        sleep(Duration::from_millis(sleep_duration));

        let mut exp_date_div =
            client.find(Locator::Id(r#"expirationDate"#)).await?;
        let exp_date_text = exp_date_div.text().await?;
        println!("{:?}", &exp_date_text);

        if exp_date_text.trim().eq("sleep_duration") {
            email::send_email(&domain).await;
        }

        sleep(Duration::from_millis(sleep_duration));
    }

    client.close_window().await?;
    client.close().await?;

    Ok(())
}
