use colored::Colorize;

#[derive(Clone, Copy, Debug, Default)]
pub struct Color {
	r: u8,
	g: u8,
	b: u8,
}

impl Color {
	pub fn hex(&self) -> String {
		format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
	}

	pub fn color<T>(&self, t: T) -> String
	where
		T: Colorize,
	{
		t.truecolor(self.r, self.g, self.b).to_string()
	}

	pub fn on_color<T>(&self, t: T) -> String
	where
		T: Colorize,
	{
		t.on_truecolor(self.r, self.g, self.b).to_string()
	}
}

impl From<[u8; 3]> for Color {
	fn from(color: [u8; 3]) -> Self {
		Self {
			r: color[0],
			g: color[1],
			b: color[2],
		}
	}
}

impl From<u32> for Color {
	fn from(color: u32) -> Self {
		Self {
			r: ((color >> 16) & 0xff) as u8,
			g: ((color >> 8) & 0xff) as u8,
			b: (color & 0xff) as u8,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn into_color() {
		matches!(Color::from([0, 0, 0]), Color { r: 0, g: 0, b: 0 });
		matches!(
			Color::from(0xbeeeef),
			Color {
				r: 0xbe,
				g: 0xee,
				b: 0xef
			}
		);
	}

	#[test]
	fn hex() {
		assert_eq!(Color::from([0, 0, 0]).hex(), "#000000");
		assert_eq!(Color::from(0xbeeeef).hex(), "#beeeef");
	}
}
