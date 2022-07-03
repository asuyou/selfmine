use crate::game;
use std::{thread, time};

#[derive(Debug)]
pub struct Algo {
  width: usize,
  height: usize,
  game: game::Board,
}

impl Algo {
  pub fn new(width: usize, height: usize, total_mines: usize) -> Algo {
    Algo {
      width,
      height,
      game: game::Board::new(width, height, total_mines),
    }
  }

  pub fn start(&mut self, wait: time::Duration) {
    thread::sleep(wait);
  }
}
