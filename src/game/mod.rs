mod random;
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
  total_mines: usize,
  mines_found: usize,
  pos_mines_h: HashSet<Position>,
  pos_flags: HashSet<Position>,
  pos_open: HashSet<Position>,
}

impl Board {
  pub fn new(width: usize, height: usize, total_mines: usize) -> Board {
    Board {
      width,
      height,
      total_mines,
      mines_found: 0,
      pos_mines_h: {
        let mut mines = HashSet::new();

        while mines.len() < total_mines {
          let x = random::rand_range(0, width);
          let y = random::rand_range(0, height);
          mines.insert((x, y));
        }

        mines
      },
      pos_flags: HashSet::new(),
      pos_open: HashSet::new(),
    }
  }

  fn open(&mut self, pos: Position) -> OpenResult {
    if self.pos_mines_h.contains(&pos) {
      return OpenResult::Mine;
    } else {
      self.pos_open.insert(pos);
      return OpenResult::NoMine(4);
    }
  }

  fn neighbors(&mut self, (x, y): Position) -> u8 {
    return 0;
  }
}
