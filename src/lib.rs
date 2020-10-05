pub mod css;
pub mod design;
pub mod sgr;


pub mod prelude {
  pub use crate::{
    *,
    css::{
      CSSColorRGB,
    },
    design::{
      Designer
    },
    sgr::SGRRGB,
  };
}


#[macro_export]
macro_rules! qute {
  ($e:expr) => {
    $crate::design::Designer::new($e)
  };
}


#[macro_export]
macro_rules! designify {
  ({
    name: $name: ident;
    color: $color: ident;
    background: $background: ident;
    font-weight: $modifier: ident
  }) => {
    println!(
      "{}",
      $crate::design::Designer::new($e)
      .set_css_color($color)
      .set_css_background($background)
      .$modifier()
      ,
    )
  }
}
