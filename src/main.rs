extern crate qute;


use qute::prelude::*;


fn main () {
  let black_white = qute!(" BLACK WHITE (BASIC) ")
  .background_black()
  .white();

  let black_yellow = qute!(" BLACK YELLOW (256) ")
  .set_color(16)
  .set_background(220);

  let white_blue = qute!(" WHITE BLUE (RGB) ")
  .set_rgb_color(238, 237, 240)
  .set_rgb_background(1, 1, 100);

  let navy_aliceblue = qute!(" NAVY ALICEBLUE (CSS) ")
  .set_css_color("navy")
  .set_css_background("aliceblue");

  println!();
  println!("{}", black_white);
  println!("{}", black_yellow);
  println!("{}", white_blue);
  println!("{}", navy_aliceblue);
  println!();
}
