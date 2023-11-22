use strum_macros::Display;

#[derive(Copy, Clone, Debug)]
pub enum Region {
  Overseas = 0,
  Chinese = 1,
}

#[derive(Copy, Clone, Display, Debug)]
pub enum Server {
  #[strum(serialize = "cn_gf01")]
  GenshinCnGf01,
  #[strum(serialize = "cn_qd01")]
  GenshinCnQd01,
  #[strum(serialize = "os_usa")]
  GenshinOsUsa,
  #[strum(serialize = "os_euro")]
  GenshinOsEuro,
  #[strum(serialize = "os_asia")]
  GenshinOsAsia,
  #[strum(serialize = "os_cht")]
  GenshinOsCht,
  #[strum(serialize = "prod_gf_cn")]
  StarrailProdGfCn,
  #[strum(serialize = "prod_qd_cn")]
  StarrailProdQdCn,
  #[strum(serialize = "prod_official_usa")]
  StarrailProdOfficialUsa,
  #[strum(serialize = "prod_official_eur")]
  StarrailProdOfficialEur,
  #[strum(serialize = "prod_official_asia")]
  StarrailProdOfficialAsia,
  #[strum(serialize = "prod_official_cht")]
  StarrailProdOfficialCht,
}

#[derive(Copy, Clone, Debug)]
pub enum Game {
  Genshin = 0,
  Starrail = 1,
}
