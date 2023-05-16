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

use chromiumoxide::{browser::{Browser, BrowserConfig}, Page};
use futures::StreamExt;
use regex::Regex;
use tokio::task::JoinHandle;

use crate::board::{self, Case, Board};

#[derive(Clone, Copy, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

pub async fn lauch_browser() -> Result<(Browser, JoinHandle<()>), Box<dyn std::error::Error>> {
    let (browser, mut handler) = Browser::launch(BrowserConfig::builder().build()?).await?;
    let handle = tokio::task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });
    let page = browser.new_page("https://reversi.fr/game.php").await?;


    //browser.close().await?;
    //handle.await?;
    Ok((browser, handle))
}

pub async fn setup_page(
    page: &mut Page,
    difficulty: Difficulty
) -> Result<(), Box<dyn std::error::Error>> {
    match difficulty {
        Difficulty::Easy => {
            page.find_element("#level0").await?.click().await?;
        },
        Difficulty::Medium => {
            page.find_element("#level1").await?.click().await?;
        },
        Difficulty::Hard => {
            page.find_element("#level2").await?.click().await?;
        }
    }
    Ok(())
}

pub async fn get_raw_board(
    page: &mut chromiumoxide::Page,
) -> Result<[[board::Case; 8]; 8], Box<dyn std::error::Error>> {
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
    Ok(board)
}

pub async fn play_move(
    page: &mut chromiumoxide::Page,
    input: (usize, usize)
) -> Result<(), Box<dyn std::error::Error>> {
    page.find_element(format!("#game-content > \
    table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > \
    tr:nth-child({}) > td:nth-child({}) > img", input.0 + 1, input.1 + 1)).await?.hover().await?.click().await?;
    Ok(())
}

pub async fn get_last_move(
    page: &mut chromiumoxide::Page,
    old: [[board::Case; 8]; 8]
) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let new = get_raw_board(page).await?;
    for i in 0..8_usize {
        for j in 0..8_usize {
            if old[i][j] == Case::Empty && new[i][j] != Case::Empty {
                return Ok((i,j));
            }
        }
    }
    
    println!("{}", board::Board::from_cases(new));
    panic!()
}



#[tokio::test(flavor = "multi_thread", worker_threads = 3)]
async fn browser_test() {
    let (mut browser, handle) = lauch_browser().await.unwrap();
    let mut page = browser.pages().await.unwrap();
    let page = page.get_mut(0).unwrap();
    let cases = get_raw_board(page).await.unwrap();
    let mut board = Board::from_cases(cases);
    println!("{}", board);
    play_move(page, (4, 5)).await.unwrap();
    board.make_move(&(4,5)).unwrap();

    tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;

    let (i, j) = get_last_move(page, board.cases).await.unwrap();

    board.make_move(&(i, j)).unwrap();

    println!("{}", board);

    browser.close().await.unwrap();
    handle.await.unwrap();
}
