use std::time::Duration;
use thirtyfour::prelude::*;
#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver
        .goto("https://www.amazon.com/SHAPERX-Bodysuit-Shapewear-Sculpting-SZ5215-Black-S/dp/B0B1HR89H4/ref=zg_bs_1294868011_sccl_1/135-3072216-7795932?psc=1")
        .await?;

    tokio::time::sleep(Duration::from_secs(10)).await;

    let element = driver
        .find(By::Id("sellerProfileTriggerId"))
        .await?
        .attr("href")
        .await?
        .unwrap();
    let path = format!("https://www.amazon.com{element}");
    dbg!(&path);
    driver.goto(path).await?;
    tokio::time::sleep(Duration::from_secs(20)).await;
    driver.quit().await?;

    Ok(())
}
