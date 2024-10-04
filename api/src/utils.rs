use std::time::SystemTime;

use phf::phf_map;
use rand::seq::SliceRandom;

use crate::types::{Game, Server};

const SERVERS: [phf::Map<u8, Server>; 3] = [
  phf_map!(
    1u8=>Server::GenshinCnGf01,
    2u8=>Server::GenshinCnGf01,
    5u8=>Server::GenshinCnQd01,
    6u8=>Server::GenshinOsUsa,
    7u8=>Server::GenshinOsEuro,
    8u8=>Server::GenshinOsAsia,
    9u8=>Server::GenshinOsCht,
  ),
  phf_map!(
    1u8=>Server::StarrailProdGfCn,
    2u8=>Server::StarrailProdGfCn,
    5u8=>Server::StarrailProdQdCn,
    6u8=>Server::StarrailProdOfficialUsa,
    7u8=>Server::StarrailProdOfficialEur,
    8u8=>Server::StarrailProdOfficialAsia,
    9u8=>Server::StarrailProdOfficialCht,
  ),
  phf_map!(
    10u8=>Server::ZzzProdGfUs,
    13u8=>Server::ZzzProdGfJp,
    15u8=>Server::ZzzProdGfEu,
    17u8=>Server::ZzzProdGfSg,
  ),
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
  SERVERS[game as usize]
    .get(&(uid.chars().next()?.to_digit(10)? as u8))
    .cloned()
}
