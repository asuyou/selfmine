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
    let changed = true;

    let x = rand_range(0, self.width);
    let y = rand_range(0, self.height);

    self.game.open((x, y));

    // while changed && !self.game.game_over {
    for _ in 0..=10 {
      for pos in &self.game.pos_open.clone() {
        self.equal_open(*pos);
      }
      self.game.display_board();
      thread::sleep(wait);
    }
    if self.game.game_over {
      println!("GAME OVER");
    }
    // }
  }

  fn equal_open(&mut self, pos: game::Position) {
    // let to_open: HashSet<game::Position> = HashSet::new();
    let to_open = self
      .game
      .iter_neighbors(pos)
      .filter(|pos| !self.game.pos_open.contains(pos))
      .filter(|pos| !self.game.pos_flags.contains(pos))
      .collect::<HashSet<game::Position>>();
    for pos in to_open {
      self.game.open(pos);
    }
  }

  fn equal_flag(&mut self, _pos: game::Position) {
    todo!()
  }
}
