mod algo;
mod game;
mod random;

fn main() {
  let width = 20;
  let height = 30;
  let mine_count = 60;
  let mut game = algo::Algo::new(width, height, mine_count);
  game.start(std::time::Duration::from_millis(100));
}
