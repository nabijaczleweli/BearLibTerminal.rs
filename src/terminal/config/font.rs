use std::fmt;
use std::path::Path;
use geometry::Size;
use terminal::config::{ConfigPart, escape_config_string};


pub fn bitmap<T: AsRef<Path>>(origin: Origin, path: T) -> BitmapFont {
	BitmapFont{
		origin: origin,
		path: path.as_ref().to_str().unwrap().to_string(),
		size: None,
		resize: None,
		resize_filter: None,
		resize_mode: None,
		raw_size: None,
		codepage: None,
		align: None,
		spacing: None,
	}
}

pub fn true_type<T: AsRef<Path>>(origin: Origin, path: T, tile_size: Size) -> TrueTypeFont {
	TrueTypeFont{
		origin: origin,
		path: path.as_ref().to_str().unwrap().to_string(),
		size: tile_size,
		size_reference: None,
		mode: None,
		codepage: None,
		align: None,
		spacing: None,
	}
}


#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Origin {
	Default,
	Offset(char),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum RasterizationMode {
	Monochrome,
	Normal,
	Lcd,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ResizeFilter {
	Nearest,
	Bilinear,
	Bicubic,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ResizeMode {
	Stretch,
	Fit,
	Crop,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Align {
	Center,
	TopLeft,
	BottomLeft,
	TopRight,
	BottomRight,
}

// What's the format for this?
// ref: http://foo.wyrd.name/en:bearlibterminal:reference:configuration#font_and_tileset_management
// pub enum Background {
// 	Auto,
// 	Color(Color),
// }


#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct BitmapFont {
	origin       : Origin,
	path         : String,
	size         : Option<Size>,
	resize       : Option<Size>,
	resize_filter: Option<ResizeFilter>,
	resize_mode  : Option<ResizeMode>,
	raw_size     : Option<Size>,
	codepage     : Option<String>,
	align        : Option<Align>,
	spacing      : Option<Size>,
//	transparent  : Option<Background>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TrueTypeFont {
	origin        : Origin,
	path          : String,
	size          : Size,
	size_reference: Option<char>,
	mode          : Option<RasterizationMode>,
	codepage      : Option<String>,
	align         : Option<Align>,
	spacing       : Option<Size>,
}


/// For all functions consult the corresponding attributes in
/// [the official docs](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#font_and_tileset_management).
impl BitmapFont {
	/// The size of a single tile in the tileset.
	pub fn size         (mut self, size: Size)                  -> Self {self.size          = Some(size)         ; self}
	/// The size to resize the image to.
	pub fn resize       (mut self, resize: Size)                -> Self {self.resize        = Some(resize)       ; self}
	/// How to resize the image.
	///
	/// Default: `ResizeFilter::Bilinear`.
	pub fn resize_filter(mut self, resize_filter: ResizeFilter) -> Self {self.resize_filter = Some(resize_filter); self}
	/// Resize aspect method.
	///
	/// Default: `ResizeMode::Stretch`.
	pub fn resize_mode  (mut self, resize_mode: ResizeMode)     -> Self {self.resize_mode   = Some(resize_mode)  ; self}
	/// Raw memory size, `size` if not specified.
	pub fn raw_size     (mut self, raw_size: Size)              -> Self {self.raw_size      = Some(raw_size)     ; self}
	/// Tileset's codepage.
	///
	/// Default: `"ascii"`.
	pub fn codepage     (mut self, codepage: String)            -> Self {self.codepage      = Some(codepage)     ; self}
	/// How tiles are to be aligned.
	///
	/// Default: `Align::Center`.
	pub fn align        (mut self, align: Align)                -> Self {self.align         = Some(align)        ; self}
	/// Tile alignment area \[cells\].
	///
	/// Default: `1x1`.
	pub fn spacing      (mut self, spacing: Size)               -> Self {self.spacing       = Some(spacing)      ; self}
}

/// For all functions consult the corresponding attributes in
/// [the official docs](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#font_and_tileset_management).
impl TrueTypeFont {
	/// Character used for size probing.
	///
	/// Default: `'@'`.
	pub fn size_reference(mut self, size_reference: char)    -> Self {self.size_reference = Some(size_reference); self}
	/// Rasterization mode. Note: `RasterizationMode::Lcd` forces opaque black background.
	///
	/// Default: `RasterizationMode::Normal`.
	pub fn mode          (mut self, mode: RasterizationMode) -> Self {self.mode           = Some(mode)          ; self}
	/// Reverse codepage for loading symbols.
	pub fn codepage      (mut self, codepage: String)        -> Self {self.codepage       = Some(codepage)      ; self}
	/// How tiles are to be aligned.
	///
	/// Default: `Align::Center`.
	pub fn align         (mut self, align: Align)            -> Self {self.align          = Some(align)         ; self}
	/// Tile alignment area \[cells\].
	///
	/// Default: `1x1`.
	pub fn spacing       (mut self, spacing: Size)           -> Self {self.spacing        = Some(spacing)       ; self}
}


impl ConfigPart for BitmapFont {
	fn to_config_str(&self) -> String {
		format!("{}: {}{}{}{}{}{}{}{};", self.origin, escape_config_string(&self.path),
			match self.resize {
				None             => "".to_string(),
				Some(ref resize) => format!(", resize={}", resize),
			},
			match self.resize_filter {
				None                    => "".to_string(),
				Some(ref resize_filter) => format!(", resize-filter={}", resize_filter),
			},
			match self.resize_mode {
				None                  => "".to_string(),
				Some(ref resize_mode) => format!(", resize-mode={}", resize_mode),
			},
			match self.raw_size {
				None               => "".to_string(),
				Some(ref raw_size) => format!(", raw-size={}", raw_size),
			},
			match self.codepage {
				None               => "".to_string(),
				Some(ref codepage) => format!(", codepage={}", escape_config_string(codepage)),
			},
			match self.align {
				None            => "".to_string(),
				Some(ref align) => format!(", align={}", align),
			},
			match self.spacing {
				None              => "".to_string(),
				Some(ref spacing) => format!(", spacing={}", spacing),
			}
		)
	}
}

impl ConfigPart for TrueTypeFont {
	fn to_config_str(&self) -> String {
		format!("{}: {}, size={}{}{}{}{}{};", self.origin, escape_config_string(&self.path), self.size,
			match self.size_reference {
				None                     => "".to_string(),
				Some(ref size_reference) => format!(", size-reference=0x{:X}", *size_reference as i32),
			},
			match self.mode {
				None           => "".to_string(),
				Some(ref mode) => format!(", mode={}", mode),
			},
			match self.codepage {
				None               => "".to_string(),
				Some(ref codepage) => format!(", codepage={}", escape_config_string(codepage)),
			},
			match self.align {
				None            => "".to_string(),
				Some(ref align) => format!(", align={}", align),
			},
			match self.spacing {
				None              => "".to_string(),
				Some(ref spacing) => format!(", spacing={}", spacing),
			}
		)
	}
}


impl fmt::Display for Origin {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Origin::Default   => formatter.write_str("font"),
			&Origin::Offset(o) => formatter.write_str(&*&format!("0x{:X}", o as i32)),
		}
	}
}

impl fmt::Display for RasterizationMode {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&RasterizationMode::Monochrome => formatter.write_str("monochrome"),
			&RasterizationMode::Normal     => formatter.write_str("normal"),
			&RasterizationMode::Lcd        => formatter.write_str("lcd"),
		}
	}
}

impl fmt::Display for ResizeFilter {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&ResizeFilter::Nearest  => formatter.write_str("nearest"),
			&ResizeFilter::Bilinear => formatter.write_str("bilinear"),
			&ResizeFilter::Bicubic  => formatter.write_str("bicubic"),
		}
	}
}

impl fmt::Display for ResizeMode {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&ResizeMode::Stretch => formatter.write_str("stretch"),
			&ResizeMode::Fit     => formatter.write_str("fit"),
			&ResizeMode::Crop    => formatter.write_str("crop"),
		}
	}
}

impl fmt::Display for Align {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Align::Center      => formatter.write_str("center"),
			&Align::TopLeft     => formatter.write_str("top-left"),
			&Align::BottomLeft  => formatter.write_str("bottom-left"),
			&Align::TopRight    => formatter.write_str("top-right"),
			&Align::BottomRight => formatter.write_str("bottom-right"),
		}
	}
}
