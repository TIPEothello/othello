/*
 File: scraper.rs
 Created Date: 10 May 2023
 Author: realbacon
 -----
 Last Modified: 10/05/2023 11:43:10
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;
use regex::Regex;
use tokio::task::JoinHandle;

use crate::board;

pub async fn lauch_browser() -> Result<(Browser, JoinHandle<()>), Box<dyn std::error::Error>> {
    let (browser, mut handler) = Browser::launch(BrowserConfig::builder().build()?).await?;
    let handle = tokio::task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });
    browser.new_page("https://reversi.fr/game.php").await?;

    //browser.close().await?;
    //handle.await?;
    Ok((browser, handle))
}

pub async fn get_board(
    page: &mut chromiumoxide::Page,
) -> Result<board::Board, Box<dyn std::error::Error>> {
    let elements = page.find_elements("#game-content > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr > td").await?;
    let mut board = [[board::Case::Empty; 8]; 8];
    for element in elements {
        let text = element.inner_html().await?.unwrap();
        // get the "onmouseover" attribute
        let re = Regex::new(r#"onmouseover="mOvr\((\d+),(\d+)\)".*src="([^"]+)\.gif""#).unwrap();
        let cap = re.captures(&text[..]).unwrap();
        let x = cap[1].parse::<usize>().unwrap();
        let y = cap[2].parse::<usize>().unwrap();
        let color = match &cap[3] {
            "black" => board::Case::Black,
            "white" => board::Case::White,
            _ => board::Case::Empty,
        };
        board[x][y] = color;
    }
    let board = board::Board::from_cases(board);
    Ok(board)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 3)]
async fn browser_test() {
    let (mut browser, handle) = lauch_browser().await.unwrap();
    let mut page = browser.pages().await.unwrap();
    let page = page.get_mut(0).unwrap();
    let board = get_board(page).await.unwrap();
    println!("{}", board);
    browser.close().await.unwrap();
    handle.await.unwrap();
}
