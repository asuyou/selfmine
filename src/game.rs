use crate::random::rand_range;
use std::collections::HashSet;

type Position = (usize, usize);

pub enum OpenResult {
  Mine,
  NoMine(u8),
}

#[derive(Debug)]
pub struct Board {
  width: usize,
  height: usize,
  game_over: bool,
  pub total_mines: usize,
  pos_mines_h: HashSet<Position>,
  pos_flags: HashSet<Position>,
  pub pos_open: HashSet<Position>,
}

impl Board {
  pub fn new(width: usize, height: usize, total_mines: usize) -> Board {
    Board {
      width,
      height,
      total_mines,
      game_over: false,
      pos_mines_h: {
        let mut mines = HashSet::new();

        while mines.len() < total_mines {
          let x = rand_range(0, width);
          let y = rand_range(0, height);
          mines.insert((x, y));
        }

        mines
      },
      pos_flags: HashSet::new(),
      pos_open: HashSet::new(),
    }
  }

  pub fn open(&mut self, pos: Position) -> OpenResult {
    if self.pos_mines_h.contains(&pos) {
      self.game_over = true;
      return OpenResult::Mine;
    } else {
      self.pos_open.insert(pos);
      return OpenResult::NoMine(self.neighbors(pos));
    }
  }

  pub fn display_board(&mut self) {
    for i in 0..self.width {
      for j in 0..self.height {
        let pos = (i, j);
        if !self.pos_open.contains(&pos) {
          print!("# ")
        } else if self.pos_mines_h.contains(&pos) {
          print!("* ")
        } else {
          print!("{} ", self.neighbors(pos))
        }
      }

      println!("");
    }
  }

  pub fn display_board_cheat(&mut self) {
    for i in 0..self.width {
      for j in 0..self.height {
        let pos = (i, j);
        if self.pos_mines_h.contains(&pos) {
          print!("* ");
        } else {
          print!("{} ", self.neighbors(pos));
        }
      }

      println!("");
    }
  }

  pub fn toggle_flag(&mut self, pos: Position) {
    if self.pos_open.contains(&pos) {
      return;
    }

    if self.pos_flags.contains(&pos) {
      self.pos_flags.remove(&pos);
    } else {
      self.pos_flags.insert(pos);
    }
  }

  pub fn neighbors(&mut self, pos: Position) -> u8 {
    self
      .iter_neighbors(pos)
      .filter(|pos| self.pos_mines_h.contains(pos))
      .count() as u8
  }

  pub fn iter_neighbors(&mut self, (x, y): Position) -> impl Iterator<Item = Position> {
    let width = self.width;
    let height = self.height;

    (x.max(1) - 1..=(x + 1).min(width - 1))
      .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
      // Ignores value of current tile
      .filter(move |&pos| pos != (x, y))
  }
}
