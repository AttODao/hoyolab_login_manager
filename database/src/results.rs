use sea_orm::InsertResult;

#[derive(Debug)]
pub enum RegisterResult {
  Registered,
  Updated,
}

#[derive(Debug)]
pub enum SetResult {
  Set,
  AccountNotRegistered,
}
