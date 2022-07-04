mod algo;
mod game;
mod random;

fn main() {
  let width = 10;
  let height = 10;
  let mine_count = 30;
  let mut game = algo::Algo::new(width, height, mine_count);
  game.start(std::time::Duration::from_millis(200));
}
