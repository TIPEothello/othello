/*
 File: scraper.rs
 Created Date: 10 May 2023
 Author: realbacon
 -----
 Last Modified: 10/05/2023 11:10:49
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use futures::StreamExt;

use chromiumoxide::browser::{Browser, BrowserConfig};

pub async fn lauch_browser() -> Result<(), Box<dyn std::error::Error>> {
    let (mut browser, mut handler) = Browser::launch(BrowserConfig::builder().build()?).await?;
    let handle = tokio::task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });
    let page = browser.new_page("https://reversi.fr/game.php").await?;
    let elements = page.find_elements("#game-content > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr > td").await?;
    for element in elements {
        let text = element.inner_html().await?;
        println!("{:?}", text);
    }
    browser.close().await?;
    handle.await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 3)]
async fn browser_test() {
    lauch_browser().await.unwrap();
}
