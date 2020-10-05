#[macro_use]


extern crate qute;
use qute::prelude::*;


fn main () {
  println!(
    "{}",
    qute!(" BLACK WHITE (BASIC) ")
      .background_black()
      .white()
    ,
  );
}
