use std::sync::Arc;

use anyhow::{Result, Error};
use crate::board::Case;
use headless_chrome::{Browser, Tab};

pub enum Difficulty {
    Easy = 0,
    Intermediate = 1,
    Hard = 2
}

pub fn launch(color: Case, difficulty: Difficulty) -> Result<Arc<Tab>, Error> {
    let browser = Browser::default()?;

    let tab = browser.new_tab()?;
    tab.navigate_to("http://reversi.fr/game.php")?;

    match difficulty {
        Difficulty::Easy => {
            tab.wait_for_element("input#level0")?.click()?;
        },
        Difficulty::Intermediate => {
            tab.wait_for_element("input#level1")?.click()?;
        },
        Difficulty::Hard => {
            tab.wait_for_element("input#level2")?.click()?;
        },
    }

    if color == Case::White {
        tab.wait_for_element("input#mode1")?.click()?;
    }

    Ok(tab)
}


pub fn get_board(tab: Arc<Tab>) -> Result<[[Case; 8]; 8]> {
    let board = [[Case::Empty; 8]; 8];

    for element in tab.wait_for_elements("#game-content > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr > td")? {
        println!("{:?}", element.get_attributes()?);
    }

    Ok(board)
}

#[test]
fn test_scrapper() {
    let tab = launch(Case::Black, Difficulty::Easy);
    assert!(tab.is_ok());
    let tab = tab.unwrap();
    let board = get_board(tab);
    if board.is_err() {
        println!("{:?}", board);
    }
    assert!(board.is_ok());
}
