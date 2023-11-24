#[derive(Clone)]
pub struct LoginCookie {
  pub ltoken: String,
  pub ltuid: String,
}

impl LoginCookie {
  pub fn new(ltoken: String, ltuid: String) -> Self {
    LoginCookie { ltoken, ltuid }
  }

  pub fn get_cookie(&self) -> String {
    // "ltoken=Ft3meFr0ZKxUnAM1MKNkhNjQfLDIunpTeJZ5vi2H; ltuid=166032325".to_string()
    format!("ltoken={}; ltuid={}", self.ltoken, self.ltuid)
  }
}
