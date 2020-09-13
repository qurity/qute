fn main () {
  for fgbg in &[38, 48] {
    for color in 0..255 {
      print!("\x1b[{};5;{}m  {:3}  \x1b[0m", fgbg, color, color);
      if (color + 1) % 6 == 4 {
        println!();
      }
    }
    println!();
  }
}
