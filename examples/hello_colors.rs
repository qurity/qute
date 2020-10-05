#[macro_use]


extern crate qute;
use qute::prelude::*;


fn main () {
  for fgbg in &[38, 48] {
    for color in 0..255 {
      print!(
        "{}",
        qute!(&format!("  {:3}  ", color))
        .set_color(*fgbg)
        .set_background(color)
      );

      if (color + 1) % 6 == 4 {
        println!();
      }
    }
    println!();
  }
}
