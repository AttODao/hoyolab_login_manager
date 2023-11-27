#[derive(Debug)]
pub enum RegisterResult {
  Registered,
  Updated,
}

#[derive(Debug)]
pub enum SetCookieResult {
  Set,
  AccountNotRegistered,
}

#[derive(Debug)]
pub enum SettingResult {
  Set,
  CookieNotSet,
  AccountNotRegistered,
}
