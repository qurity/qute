pub mod css;
pub mod design;
pub mod sgr;


#[macro_export]
macro_rules! qute {
  ($e:expr) => {
    $crate::design::Designer::new($e)
  };
}
