//! Module containing font changing tools.
//!
//! # Examples
//! The following from [the official documentation](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#font_and_tileset_management):
//!
//! ```text
//! font: UbuntuMono-R.ttf, size=12;
//! 0x5E: curcumflex.png;
//! 0xE000: tileset.png, size=16x16, spacing=2x1;
//! ```
//!
//! Is equivalent to
//!
//! ```
//! use bear_lib_terminal::terminal;
//! use bear_lib_terminal::terminal::config::font;
//! use bear_lib_terminal::geometry::Size;
//! # assert!({let result =
//! terminal::set(font::true_type(font::Origin::Root, "UbuntuMono-R.ttf", Size::new(0, 12)));
//! # result}); assert!({let result =
//! terminal::set(font::bitmap(font::Origin::Offset('^'), "circumflex.png"));
//! # result}); assert!({let result =
//! terminal::set(font::bitmap(font::Origin::Offset('\u{E000}'), "tileset.png").size(Size::new(16, 16)).spacing(Size::new(2, 1)));
//! # result});
//! ```


use std::fmt;
use std::path::Path;
use geometry::Size;
use terminal::config::{ConfigPart, escape_config_string};


/// Construct a bitmap font override segment repr.
pub fn bitmap<T: AsRef<Path>>(origin: Origin, path: T) -> Bitmap {
	Bitmap{
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

/// Construct a TrueType font override segment repr.
///
/// If `title_size.width` is `0`, the resulting `size` prop will be `size=<title_size.width>` as opposed to `size=<title_size>`.
pub fn true_type<T: AsRef<Path>>(origin: Origin, path: T, tile_size: Size) -> TrueType {
	TrueType{
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


/// The origin for the font (the part before `:` in the config string).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Origin {
	/// `font`
	Root,
	/// `0xNNNN`
	Offset(char),
}

/// Rasterization mode for TrueType fonts.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum RasterizationMode {
	Monochrome,
	Normal,
	/// Forces an opaque black background.
	Lcd,
}

/// Resizing filter for bitmaps.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ResizeFilter {
	Nearest,
	Bilinear,
	Bicubic,
}

/// How to aspect-change when resizing a bitmap.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ResizeMode {
	Stretch,
	Fit,
	Crop,
}

/// Per-tileset tile alignment.
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


/// A bitmap font override segment repr, constructed with [`true_type()`](fn.true_type.html).
///
/// Refer to [the official documentation](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#font_and_tileset_management).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Bitmap {
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

/// A TrueType font override segment repr, constructed with [`true_type()`](fn.true_type.html).
///
/// Refer to [the official documentation](http://foo.wyrd.name/en:bearlibterminal:reference:configuration#font_and_tileset_management).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TrueType {
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
impl Bitmap {
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
impl TrueType {
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


impl ConfigPart for Bitmap {
	fn to_config_str(&self) -> String {
		format!("{}: {}{}{}{}{}{}{}{}{};", self.origin, escape_config_string(&self.path),
			match self.size {
				None           => "".to_string(),
				Some(ref size) => format!(", size={}", size),
			},
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

impl ConfigPart for TrueType {
	fn to_config_str(&self) -> String {
		format!("{}: {}, size={}{}{}{}{}{};", self.origin, escape_config_string(&self.path),
			match self.size {
				Size{width: 0, height} => format!("{}", height),
				size                   => format!("{}", size),
			},
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
			&Origin::Root      => formatter.write_str("font"),
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
