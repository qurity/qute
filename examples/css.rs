#[macro_use]


extern crate qute;
use qute::prelude::*;


fn main () {
  println!(
    "{}",
    qute!(" NAVY ALICEBLUE (CSS) ")
      .set_css_color("navy")
      .set_css_background("aliceblue")
    ,
  );
}
