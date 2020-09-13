#[macro_use]


extern crate qute;


#[cfg(test)]
mod design {
  use super::*;
  use qute::design::Designer;


  #[test]
  fn test_css_colors () {
    qute!(" Hello ")
      .background_black()
      .white();
  }
}
