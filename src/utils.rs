use std::time::SystemTime;

use rand::seq::SliceRandom;

use crate::types::{Game, Server};

const SERVERS: [[Option<Server>; 10]; 2] = [
  [
    None,
    Some(Server::GenshinCnGf01),
    Some(Server::GenshinCnGf01),
    None,
    None,
    Some(Server::GenshinCnQd01),
    Some(Server::GenshinOsUsa),
    Some(Server::GenshinOsEuro),
    Some(Server::GenshinOsAsia),
    Some(Server::GenshinOsCht),
  ],
  [
    None,
    Some(Server::StarrailProdGfCn),
    Some(Server::StarrailProdGfCn),
    None,
    None,
    Some(Server::StarrailProdQdCn),
    Some(Server::StarrailProdOfficialUsa),
    Some(Server::StarrailProdOfficialEur),
    Some(Server::StarrailProdOfficialAsia),
    Some(Server::StarrailProdOfficialCht),
  ],
];

// From: https://qiita.com/aoyagikouhei/items/b796632ff6581197737c
pub fn random_chars(len: usize) -> Option<String> {
  const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
  let mut rng = &mut rand::thread_rng();
  String::from_utf8(
    CHARS
      .as_bytes()
      .choose_multiple(&mut rng, len)
      .cloned()
      .collect(),
  )
  .ok()
}
pub fn generate_ds(salt: &str) -> Option<String> {
  let t = (SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .ok()?
    .as_millis()
    + 500)
    / 1000;
  let r = random_chars(6)?;
  let h = md5::compute(format!("salt={}&t={}&r={}", salt, t, r).as_bytes());
  Some(format!("{},{},{:x}", t, r, h))
}

pub fn recognize_server(game: Game, uid: &String) -> Option<Server> {
  SERVERS[game as usize][uid.chars().next()?.to_digit(10)? as usize]
}
