use crate::types::Game;

pub trait GameIdentification {
  fn game() -> Game;
}
