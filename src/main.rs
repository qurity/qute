#[macro_use]


extern crate qute;
use qute::design::Designer;


fn main () {
  println!(
    "\n{} {} {} {}\n",
    qute!(" BLACK WHITE (BASIC) ")
      .background_black()
      .white()
    ,
    qute!(" BLACK YELLOW (256) ")
      .set_color(16)
      .set_background(220)
    ,
    qute!(" WHITE BLUE (RGB) ")
      .set_rgb_color(238, 237, 240)
      .set_rgb_background(1, 1, 100)
    ,
    qute!(" NAVY ALICEBLUE (CSS) ")
      .set_css_color("navy")
      .set_css_background("aliceblue")
    ,
  );
}
