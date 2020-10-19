use std::fmt::{Display, Formatter, Result};


pub const START_ESCAPE_ASCII: &str = "\x1b[";
pub const END_ESCAPE_ASCII: &str = "m";


/// Set Graphic(s) Rendition (SGR)


/// SGR Set attributes i.e modifier
/// it contains enums to apply modifiers effects
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum SGRSet {
  Bold = 1,
  Dim = 2,
  Italic = 3,
  Underline = 4,
  Blink = 5,
  Reverse = 7,
  Hidden = 8,
  Strikethrough = 9,
}


impl Display for SGRSet {
	fn fmt (&self, f: &mut Formatter) -> Result {
    match self {
      SGRSet::Bold => write!(f, "{}", 1),
      SGRSet::Dim => write!(f, "{}", 2),
      SGRSet::Italic => write!(f, "{}", 3),
      SGRSet::Underline => write!(f, "{}", 4),
      SGRSet::Blink => write!(f, "{}", 5),
      SGRSet::Reverse => write!(f, "{}", 7),
      SGRSet::Hidden => write!(f, "{}", 8),
      SGRSet::Strikethrough => write!(f, "{}", 9),
    }
	}
}


/// SGR Reset attributes i.e modifier
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum SGRReset {
  ALL = 0,
  Bold = 21,
  Dim = 22,
  Italic = 23,
  Underline = 24,
  Blink = 25,
  Reverse = 27,
  Hidden = 28,
  Strikethrough = 29,
}


impl Display for SGRReset {
	fn fmt (&self, f: &mut Formatter) -> Result {
    match self {
      SGRReset::ALL => write!(f, "{}", 0),
      SGRReset::Bold => write!(f, "{}", 21),
      SGRReset::Dim => write!(f, "{}", 22),
      SGRReset::Italic => write!(f, "{}", 23),
      SGRReset::Underline => write!(f, "{}", 24),
      SGRReset::Blink => write!(f, "{}", 25),
      SGRReset::Reverse => write!(f, "{}", 27),
      SGRReset::Hidden => write!(f, "{}", 28),
      SGRReset::Strikethrough => write!(f, "{}", 29),
    }
	}
}


/// SGR Foreground attributes i.e text color
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum SGRForeground {
  DEFAULT = 39,
  Black = 30,
  Red = 31,
  Green = 32,
  Yellow = 33,
  Blue = 34,
  Magenta = 35,
  Cyan = 36,
  Gray = 37,
  LightGray = 90,
  LightRed = 91,
  LightGreen = 92,
  LightYellow = 93,
  LightBlue = 94,
  LightMagenta = 95,
  LightCyan = 96,
  White = 97,
}


impl Display for SGRForeground {
	fn fmt (&self, f: &mut Formatter) -> Result {
    match self {
      SGRForeground::DEFAULT => write!(f, "{}", 39),
      SGRForeground::Black => write!(f, "{}", 30),
      SGRForeground::Red => write!(f, "{}", 31),
      SGRForeground::Green => write!(f, "{}", 32),
      SGRForeground::Yellow => write!(f, "{}", 33),
      SGRForeground::Blue => write!(f, "{}", 34),
      SGRForeground::Magenta => write!(f, "{}", 35),
      SGRForeground::Cyan => write!(f, "{}", 36),
      SGRForeground::Gray => write!(f, "{}", 37),
      SGRForeground::LightGray => write!(f, "{}", 90),
      SGRForeground::LightRed => write!(f, "{}", 91),
      SGRForeground::LightGreen => write!(f, "{}", 92),
      SGRForeground::LightYellow => write!(f, "{}", 93),
      SGRForeground::LightBlue => write!(f, "{}", 94),
      SGRForeground::LightMagenta => write!(f, "{}", 95),
      SGRForeground::LightCyan => write!(f, "{}", 96),
      SGRForeground::White => write!(f, "{}", 97),
    }
	}
}


/// SGR Background attributes i.e background color
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum SGRBackground {
  DEFAULT = 49,
  Black = 40,
  Red = 41,
  Green = 42,
  Yellow = 43,
  Blue = 44,
  Magenta = 45,
  Cyan = 46,
  Gray = 47,
  LightGray = 100,
  LightRed = 101,
  LightGreen = 102,
  LightYellow = 103,
  LightBlue = 104,
  LightMagenta = 105,
  LightCyan = 106,
  White = 107,
}


impl Display for SGRBackground {
	fn fmt (&self, f: &mut Formatter) -> Result {
    match self {
      SGRBackground::DEFAULT => write!(f, "{}", 49),
      SGRBackground::Black => write!(f, "{}", 40),
      SGRBackground::Red => write!(f, "{}", 41),
      SGRBackground::Green => write!(f, "{}", 42),
      SGRBackground::Yellow => write!(f, "{}", 43),
      SGRBackground::Blue => write!(f, "{}", 44),
      SGRBackground::Magenta => write!(f, "{}", 45),
      SGRBackground::Cyan => write!(f, "{}", 46),
      SGRBackground::Gray => write!(f, "{}", 47),
      SGRBackground::LightGray => write!(f, "{}", 100),
      SGRBackground::LightRed => write!(f, "{}", 101),
      SGRBackground::LightGreen => write!(f, "{}", 102),
      SGRBackground::LightYellow => write!(f, "{}", 103),
      SGRBackground::LightBlue => write!(f, "{}", 104),
      SGRBackground::LightMagenta => write!(f, "{}", 105),
      SGRBackground::LightCyan => write!(f, "{}", 106),
      SGRBackground::White => write!(f, "{}", 107),
    }
	}
}


/// SGR RGB attributes i.e primary color (red, green blue) 
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum SGRRGB {
  DEFAULT = 2,
  SELECTOR = 5,
  Foreground = 38,
  Background = 48,
}


impl Display for SGRRGB {
	fn fmt(&self, f: &mut Formatter) -> Result {
    match self {
      SGRRGB::DEFAULT => write!(f, "{}", 2),
      SGRRGB::SELECTOR => write!(f, "{}", 5),
      SGRRGB::Foreground => write!(f, "{}", 38),
      SGRRGB::Background => write!(f, "{}", 48),
    }
	}
}
