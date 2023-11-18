#[derive(Clone)]
pub struct AccountInfo {
  ltoken: String,
  ltuid: String,
}

impl AccountInfo {
  pub fn new(ltoken: String, ltuid: String) -> AccountInfo {
    AccountInfo { ltoken, ltuid }
  }

  pub fn get_cookie(&self) -> String {
    // "ltoken=Ft3meFr0ZKxUnAM1MKNkhNjQfLDIunpTeJZ5vi2H; ltuid=166032325".to_string()
    format!("ltoken={}; ltuid={}", self.ltoken, self.ltuid)
  }
}
