use captcha_rs::{Captcha, CaptchaBuilder};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct CaptchaParameter {
  pub length: Option<usize>,
  pub width: Option<u32>,
  pub height: Option<u32>,
  pub complexity: Option<u32>,
  pub compression: Option<u8>,
}

impl CaptchaParameter {
  pub fn build(&self,value:String) -> Captcha {
    let default = CaptchaParameter::default();
    
    CaptchaBuilder::new()
      .text(value)
      .length(self.length.unwrap_or(default.length.unwrap_or(5)))
      .width(self.width.unwrap_or(default.width.unwrap_or(130)))
      .height(self.height.unwrap_or(default.height.unwrap_or(40)))
      .complexity(self.complexity.unwrap_or(default.complexity.unwrap_or(1)))
      .compression(
        self
          .compression
          .unwrap_or(default.compression.unwrap_or(99)),
      )
      .build()
  }
}
