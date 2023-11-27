use api::types::Game;

#[derive(Debug, poise::ChoiceParameter)]
pub enum GameChoice {
  Genshin,
  Starrail,
}
impl GameChoice {
  pub fn to_game(&self) -> Game {
    match self {
      Self::Genshin => Game::Genshin,
      Self::Starrail => Game::Starrail,
    }
  }
}

#[derive(Debug, poise::ChoiceParameter)]
pub enum SetChoice {
  Enable,
  Disable,
}
impl SetChoice {
  pub fn to_bool(&self) -> bool {
    match self {
        SetChoice::Enable => true,
        SetChoice::Disable => false,
    }
  }
}
