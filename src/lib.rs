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
macro_rules! style {
  ({
    value: $value:expr;
    color: $color:expr;
    background: $background:expr;
    font-weight: $modifier:ident;
  }) => {
    println!(
      "{}",
      $crate::design::Designer::new($value)
      .set_css_color($color)
      .set_css_background($background)
      .$modifier()
      ,
    )
  }
}
