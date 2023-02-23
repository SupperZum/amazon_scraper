use serde::Serialize;
use std::time::Duration;
use thirtyfour::prelude::*;

#[derive(Serialize)]
struct Record {
    store: String,
    comments: String,
    about: String,
    details: String,
}
#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver
        .goto("https://www.amazon.com/SHAPERX-Bodysuit-Shapewear-Sculpting-SZ5215-Black-S/dp/B0B1HR89H4/ref=zg_bs_1294868011_sccl_1/135-3072216-7795932?psc=1")
        .await?;

    let mut csv = csv::Writer::from_path("stores.csv").expect("Failed to open output csv file");

    tokio::time::sleep(Duration::from_secs(5)).await;

    let element = driver.find(By::Id("sellerProfileTriggerId")).await?;
    let store = element.text().await?;

    element.click().await?;

    let element = driver
        .find(By::ClassName(
            "a-link-normal.feedback-detail-description.no-text-decoration",
        ))
        .await?;
    let comments = element.text().await?;

    let element = driver
        .find(By::ClassName(
            "a-row.a-spacing-none.spp-expander-more-content",
        ))
        .await?;
    let about = element.text().await?;

    let element = driver
        .find(By::Id("page-section-detail-seller-info"))
        .await?;
    let details = element.text().await?;

    let rec1 = Record {
        store,
        comments,
        about,
        details,
    };

    csv.serialize(rec1).unwrap();

    csv.flush().unwrap();

    driver.quit().await?;

    Ok(())
}
