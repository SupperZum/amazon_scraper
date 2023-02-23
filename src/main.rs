use std::time::Duration;
use thirtyfour::prelude::*;
#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver
        .goto("https://www.amazon.com/SHAPERX-Bodysuit-Shapewear-Sculpting-SZ5215-Black-S/dp/B0B1HR89H4/ref=zg_bs_1294868011_sccl_1/135-3072216-7795932?psc=1")
        .await?;

    tokio::time::sleep(Duration::from_secs(5)).await;

    let element = driver.find(By::Id("sellerProfileTriggerId")).await?;
    println!("Store:\t {}\n", element.text().await?);
    element.click().await?;

    let element = driver
        .find(By::ClassName(
            "a-link-normal.feedback-detail-description.no-text-decoration",
        ))
        .await?;
    println!("Comments: \t{}\n", element.text().await?);

    let element = driver
        .find(By::ClassName(
            "a-row.a-spacing-none.spp-expander-more-content",
        ))
        .await?;
    println!("About: \n{}\n", element.text().await?);

    let element = driver
        .find(By::Id("page-section-detail-seller-info"))
        .await?;
    println!("Details: \n{}", element.text().await?);

    driver.quit().await?;

    Ok(())
}
