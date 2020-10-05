#[macro_use]


extern crate qute;
use qute::prelude::*;


fn main () {
  println!(
    "{}",
    qute!(" WHITE BLUE (RGB) ")
      .set_rgb_color(238, 237, 240)
      .set_rgb_background(1, 1, 100)
    ,
  );
}
