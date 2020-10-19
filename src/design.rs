use crate::{
	css::{CSS_COLORS_RGB, CSSColorRGB, CSSKeyword},
	sgr::{
		END_ESCAPE_ASCII,
		START_ESCAPE_ASCII,
		SGRBackground,
		SGRForeground,
		SGRReset,
		SGRSet,
		SGRRGB,
	},
};


use std::fmt::{Display, Formatter, Result};


#[derive(Clone, Debug)]
pub struct Designer {
	string: String,
}


impl Display for Designer {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", self.string)
	}
}


impl Designer {
	pub fn new (raw_string: &str) -> Self {
		Self {
			string: raw_string.to_owned(),
		}
	}

	pub fn none (self) -> String {
		self.string
	}

	pub fn bold (&mut self) -> Self {
		let to_bold = self.to(
			&SGRSet::Bold,
			&SGRReset::Bold
		);

		self.style(&to_bold(&self.string))
  }

	pub fn dim (&mut self) -> Self {
		let to_dim = self.to(
			&SGRSet::Dim,
			&SGRReset::Dim
		);

		self.style(&to_dim(&self.string))
  }

	pub fn italic (&mut self) -> Self {
		let to_italic = self.to(
			&SGRSet::Italic,
			&SGRReset::Italic
		);

		self.style(&to_italic(&self.string))
	}

	pub fn underline (&mut self) -> Self {
		let to_underline = self.to(
			&SGRSet::Underline,
			&SGRReset::Underline
		);

		self.style(&to_underline(&self.string))
	}

	/// NOTE: the "blink" mode works in the tty and xterm only
	pub fn blink (&mut self) -> Self {
		let to_blink = self.to(
			&SGRSet::Blink,
			&SGRReset::Blink
		);

		self.style(&to_blink(&self.string))
	}

	/// NOTE: the "reverse" mode invert the foreground and the background colors
	pub fn reverse (&mut self) -> Self {
		let to_reverse = self.to(
			&SGRSet::Reverse,
			&SGRReset::Reverse
		);

		self.style(&to_reverse(&self.string))
  }

	/// NOTE: the "hidden" mode can be used for passwords
  pub fn hidden (&mut self) -> Self {
		let to_hide = self.to(
			&SGRSet::Hidden,
			&SGRReset::Hidden
		);

		self.style(&to_hide(&self.string))
  }

	pub fn strikethrough (&mut self) -> Self {
		let to_strikethrough = self.to(
			&SGRSet::Strikethrough,
			&SGRReset::Strikethrough
		);

		self.style(&to_strikethrough(&self.string))
  }

	pub fn black (&mut self) -> Self {
		let to_black = self.to(
			&SGRForeground::Black,
			&SGRForeground::DEFAULT
		);

		self.style(&to_black(&self.string))
	}

	pub fn red (&mut self) -> Self {
		let to_red = self.to(
			&SGRForeground::Red,
			&SGRForeground::DEFAULT
		);

		self.style(&to_red(&self.string))
	}

	pub fn green (&mut self) -> Self {
		let to_green = self.to(
			&SGRForeground::Green,
			&SGRForeground::DEFAULT
		);

		self.style(&to_green(&self.string))
  }

	pub fn yellow (&mut self) -> Self {
		let to_yellow = self.to(
			&SGRForeground::Yellow,
			&SGRForeground::DEFAULT
		);

		self.style(&to_yellow(&self.string))
  }

	pub fn blue (&mut self) -> Self {
		let to_blue = self.to(
			&SGRForeground::Blue,
			&SGRForeground::DEFAULT
		);

		self.style(&to_blue(&self.string))
  }

	pub fn magenta (&mut self) -> Self {
		let to_magenta = self.to(
			&SGRForeground::Magenta,
			&SGRForeground::DEFAULT
		);

		self.style(&to_magenta(&self.string))
  }

	pub fn cyan (&mut self) -> Self {
		let to_cyan = self.to(
			&SGRForeground::Cyan,
			&SGRForeground::DEFAULT
		);

		self.style(&to_cyan(&self.string))
  }

	pub fn gray (&mut self) -> Self {
		let to_gray = self.to(
			&SGRForeground::Gray,
			&SGRForeground::DEFAULT
		);

		self.style(&to_gray(&self.string))
  }

	pub fn light_gray (&mut self) -> Self {
		let to_light_gray = self.to(
			&SGRForeground::LightGray,
			&SGRForeground::DEFAULT
		);

		self.style(&to_light_gray(&self.string))
  }

  pub fn light_red (&mut self) -> Self {
		let to_light_red = self.to(
			&SGRForeground::LightRed,
			&SGRForeground::DEFAULT
		);

		self.style(&to_light_red(&self.string))
  }

	pub fn light_green (&mut self) -> Self {
		let to_light_green = self.to(
			&SGRForeground::LightGreen,
			&SGRForeground::DEFAULT
		);

		self.style(&to_light_green(&self.string))
  }

	pub fn light_yellow (&mut self) -> Self {
		let to_light_yellow = self.to(
			&SGRForeground::LightYellow,
			&SGRForeground::DEFAULT
		);

		self.style(&to_light_yellow(&self.string))
	}

	pub fn light_blue (&mut self) -> Self {
		let to_light_blue = self.to(
			&SGRForeground::LightBlue,
			&SGRForeground::DEFAULT
		);

		self.style(&to_light_blue(&self.string))
  }

	pub fn light_magenta (&mut self) -> Self {
		let to_light_magenta = self.to(
			&SGRForeground::LightMagenta,
			&SGRForeground::DEFAULT
		);

		self.style(&to_light_magenta(&self.string))
  }

	pub fn light_cyan (&mut self) -> Self {
		let to_light_cyan = self.to(
			&SGRForeground::Cyan,
			&SGRForeground::DEFAULT
		);

		self.style(&to_light_cyan(&self.string))
  }

	pub fn white (&mut self) -> Self {
		let to_white = self.to(
			&SGRForeground::White,
			&SGRForeground::DEFAULT
		);

		self.style(&to_white(&self.string))
  }

	pub fn background_black (&mut self) -> Self {
		let to_background_black = self.to(
			&SGRBackground::Black,
			&SGRBackground::DEFAULT
		);

		self.style(&to_background_black(&self.string))
  }

	pub fn background_red (&mut self) -> Self {
		let to_background_red = self.to(
			&SGRBackground::Red,
			&SGRBackground::DEFAULT
		);

		self.style(&to_background_red(&self.string))
  }

	pub fn background_green (&mut self) -> Self {
		let to_background_green = self.to(
			&SGRBackground::Green,
			&SGRBackground::DEFAULT
		);

		self.style(&to_background_green(&self.string))
  }

	pub fn background_yellow (&mut self) -> Self {
		let to_background_yellow = self.to(
			&SGRBackground::Yellow,
			&SGRBackground::DEFAULT
		);

		self.style(&to_background_yellow(&self.string))
  }

	pub fn background_blue (&mut self) -> Self {
		let to_background_blue = self.to(
			&SGRBackground::Blue,
			&SGRBackground::DEFAULT
		);

		self.style(&to_background_blue(&self.string))
  }

	pub fn background_magenta (&mut self) -> Self {
		let to_background_magenta = self.to(
			&SGRBackground::Magenta,
			&SGRBackground::DEFAULT
		);

		self.style(&to_background_magenta(&self.string))
  }

	pub fn background_cyan (&mut self) -> Self {
		let to_background_cyan = self.to(
			&SGRBackground::Cyan,
			&SGRBackground::DEFAULT
		);

		self.style(&to_background_cyan(&self.string))
  }

	pub fn background_gray (&mut self) -> Self {
		let to_background_gray = self.to(
			&SGRBackground::Gray,
			&SGRBackground::DEFAULT
		);

		self.style(&to_background_gray(&self.string))
  }

	pub fn background_light_gray (&mut self) -> Self {
		let to_background_light_gray = self.to(
			&SGRBackground::LightGray,
			&SGRBackground::DEFAULT
		);

		self.style(&to_background_light_gray(&self.string))
  }

	pub fn background_light_red (&mut self) -> Self {
		let to_background_light_red = self.to(
			&SGRBackground::LightRed,
			&SGRBackground::DEFAULT
		);

		self.style(&to_background_light_red(&self.string))
  }

	pub fn background_light_green (&mut self) -> Self {
		let to_background_light_green = self.to(
			&SGRBackground::LightGreen,
			&SGRBackground::DEFAULT
		);

		self.style(&to_background_light_green(&self.string))
  }

	pub fn background_light_yellow (&mut self) -> Self {
		let to_background_light_yellow = self.to(
			&SGRBackground::LightYellow,
			&SGRBackground::DEFAULT
		);

		self.style(&to_background_light_yellow(&self.string))
  }

	pub fn background_light_blue (&mut self) -> Self {
		let to_background_light_blue = self.to(
			&SGRBackground::LightBlue,
			&SGRBackground::DEFAULT
		);

		self.style(&to_background_light_blue(&self.string))
  }

	pub fn background_light_magenta (&mut self) -> Self {
		let to_background_light_magenta = self.to(
			&SGRBackground::LightMagenta,
			&SGRBackground::DEFAULT
		);

		self.style(&to_background_light_magenta(&self.string))
  }

  pub fn background_light_cyan (&mut self) -> Self {
		let to_background_light_cyan = self.to(
			&SGRBackground::LightCyan,
			&SGRBackground::DEFAULT
		);

		self.style(&to_background_light_cyan(&self.string))
  }

  pub fn background_white (&mut self) -> Self {
		let to_background_white = self.to(
			&SGRBackground::White,
			&SGRBackground::DEFAULT
		);

		self.style(&to_background_white(&self.string))
	}

	pub fn set_color (&mut self, color: u8) -> Self {
		let to_color = self.set(
			&SGRRGB::Foreground,
			&SGRRGB::SELECTOR
		);

		self.style(&to_color(color))
	}

	pub fn set_background (&mut self, color: u8) -> Self {
		let to_background = self.set(
			&SGRRGB::Background,
			&SGRRGB::SELECTOR
		);

		self.style(&to_background(color))
	}

	pub fn set_rgb_color (&mut self, r: u8, g: u8, b: u8) -> Self {
		let to_rgb = self.rgb(
			&SGRRGB::Foreground,
			&SGRRGB::DEFAULT
		);

		self.style(&to_rgb(r, g, b))
	}

	pub fn set_rgb_background (&mut self, r: u8, g: u8, b: u8) -> Self {
		let to_rgb = self.rgb(
			&SGRRGB::Background,
			&SGRRGB::DEFAULT
		);

		self.style(&to_rgb(r, g, b))
	}

	/// default color: rgb(0, 0, 0)
	pub fn set_css_color (&mut self, name: &str) -> Self {
		let to_rgb = self.rgb(
			&SGRRGB::Foreground,
			&SGRRGB::DEFAULT
		);

		let color = self.get_css_color(&name);

		self.style(&to_rgb(color.r, color.g, color.b))
	}

	/// default color: rgb(0, 0, 0)
	pub fn set_css_background (&mut self, name: &str) -> Self {
		let to_rgb = self.rgb(
			&SGRRGB::Background,
			&SGRRGB::DEFAULT
		);

		let color = self.get_css_color(&name);

		self.style(&to_rgb(color.r, color.g, color.b))
	}

	/// get css color by keyword
	fn get_css_color (&mut self, keyword: &str) -> CSSColorRGB {
		for color in CSS_COLORS_RGB {
			if keyword == color.keyword.to_string() {
				return * color;
			}
		}

		return CSS_COLORS_RGB[CSSKeyword::Black as usize];
	}

	/// set is private and it uses for rgb color and css color
	/// @param {&'s T} start_escape_sequence: the start SGR number
	/// @param {&'s U} end_escape_sequence: the end SGR numer
	fn rgb <'s, T: Display, U: Display> (
		&mut self,
		start_escape_sequence: &'s T,
		end_escape_sequence: &'s U,
	) -> Box<dyn Fn (u8, u8, u8) -> String + 's > {
		let string = self.string.to_owned();

		Box::new(move | r: u8, g: u8, b: u8 | {
			format!(
				"{}{};{};{};{};{}{}{}{}{}{}",
				START_ESCAPE_ASCII, // "\x1B["
				start_escape_sequence, // "number"
				end_escape_sequence,
				r, // 0..255
				g, // 0..255
				b, // 0..255
				END_ESCAPE_ASCII, // "m"
				string,
				START_ESCAPE_ASCII, // "\x1B["
				SGRReset::ALL, // 0
				END_ESCAPE_ASCII, // "m"
			)
		})
	}

	/// use for basic ansi 256 color
	/// set is private and it uses for 256 color
	/// @param {&'s T} start_escape_sequence: the start SGR number
	/// @param {&'s U} end_escape_sequence: the end SGR numer
	/// returns function closure. it takes color parameter
	/// the format string example: ```"\x1b[38;5;16mHello World\x1b[0m"```
	fn set <'s, T: Display, U: Display> (
		&mut self,
		start_escape_sequence: &'s T,
		end_escape_sequence: &'s U,
	) -> Box<dyn Fn (u8) -> String + 's > {
		let string = self.string.to_owned();

		Box::new(move | color: u8 | {
			format!(
				"{}{};{};{}{}{}{}{}{}",
				START_ESCAPE_ASCII, // "\x1B["
				start_escape_sequence, // "number"
				end_escape_sequence, // "number"
				color, // 0..255
				END_ESCAPE_ASCII, // "m"
				string, // it can be any string i.e "hello world" for example
				START_ESCAPE_ASCII, // "\x1B["
				SGRReset::ALL, // 0
				END_ESCAPE_ASCII, // "m"
			)
		})
	}

	/// use for basic ansi color
	/// set is private and it uses for basic color
	/// @param {&'s T} start_escape_sequence: the start SGR number
	/// @param {&'s U} end_escape_sequence: the end SGR numer
	fn to <'s, T: Display, U: Display> (
		&mut self,
		start_escape_sequence: &'s T,
		end_escape_sequence: &'s U,
	) -> Box<dyn Fn (&str) -> String + 's > {
		Box::new(move |raw_string: &str| {
			format!(
				"{}{}{}{}{}{}{}{}{}{}",
				START_ESCAPE_ASCII, // "\x1B["
				start_escape_sequence, // "number"
				END_ESCAPE_ASCII, // "m"
				raw_string, // it can be any string i.e "hello world" for example
				START_ESCAPE_ASCII, // "\x1B["
				end_escape_sequence, // "number"
				END_ESCAPE_ASCII, // "m"
				START_ESCAPE_ASCII, // "\x1B["
				SGRReset::ALL, // 0
				END_ESCAPE_ASCII, // "m"
			)
		})
	}

	/// create new instance with the formatted string
	fn style (&mut self, new_string: &str) -> Self {
		Designer {
			string: new_string.to_owned(),
		}
	}
}
