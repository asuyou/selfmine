use crate::game;
use crate::random::rand_range;
use std::collections::HashSet;

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
    self.game.display_board_cheat();
    // let changed = true;

    let mut x = rand_range(0, self.width);
    let mut y = rand_range(0, self.height);

    while self.game.open((x, y)) != game::OpenResult::NoMine(0) {
      x = rand_range(0, self.width);
      y = rand_range(0, self.height);
    }
    // self.game.open((x, y));

    // while changed && !self.game.game_over {
    for _ in 0..=self.width.max(self.height) {
      for pos in &self.game.pos_open.clone() {
        self.equal_flag(*pos);
        self.equal_open(*pos);
      }
      println!("{}", "=".repeat(2 * self.width));
      self.game.display_board();
      thread::sleep(wait);
    }
    if self.game.game_over {
      println!("GAME OVER");
    }
  }

  fn equal_open(&mut self, pos: game::Position) {
    let flag_count = self
      .game
      .iter_neighbors(pos)
      .filter(|pos| self.game.pos_flags.contains(pos))
      .count() as u8;

    let to_open = self
      .game
      .iter_neighbors(pos)
      .filter(|pos| !self.game.pos_open.contains(pos))
      .filter(|pos| !self.game.pos_flags.contains(pos))
      .collect::<HashSet<game::Position>>();

    if flag_count == self.game.neighbors_mines(pos) {
      for pos in to_open {
        // println!("{:?}", pos);
        self.game.open(pos);
      }
    }
  }

  fn equal_flag(&mut self, pos: game::Position) {
    let neighbors_mines = self.game.neighbors_mines(pos);

    let unopened = self
      .game
      .iter_neighbors(pos)
      .filter(|pos| !self.game.pos_open.contains(pos))
      .collect::<HashSet<game::Position>>();

    let len_closed_pos = unopened.len() as u8;

    if len_closed_pos == neighbors_mines {
      for pos in unopened {
        self.game.toggle_flag(pos);
      }
    }
  }
}
