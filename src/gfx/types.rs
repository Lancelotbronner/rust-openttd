use crate::zoom::ZoomLevel;
use enum_bitset::EnumBitset;
use sdl3::rect::Point;
use std::mem::transmute;

///< The number of a sprite, without mapping bits and colourtables.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SpriteId(u32);

///< The number of the palette
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PaletteId(u32);

///< The number of the cursor (sprite)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CursorId(u32);

/** Combination of a palette sprite and a 'real' sprite */
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PalSpriteId {
    ///< The 'real' sprite
    pub sprite: SpriteId,
    ///< The palette (use \c PAL_NONE) if not needed)
    pub pal: PaletteId,
}

/** A single sprite of a list of animated cursors */
#[derive(Copy, Clone)]
pub struct AnimCursor {
    ///< Must be set to LAST_ANIM when it is the last sprite of the loop
    pub sprite: CursorId,
    ///< Amount of ticks this sprite will be shown
    pub display_time: u8,
}

#[derive(Copy, Clone)]
pub struct CursorSprite {
    ///< Image.
    pub image: PalSpriteId,
    ///< Relative position.
    pub pos: Point,
}

/** Collection of variables for cursor-display and -animation */
pub struct CursorVars<'a> {
    /* Logical mouse position */
    ///< logical mouse position
    pub pos: Point,
    ///< relative mouse movement in this tick
    pub delta: Point,
    ///< mouse wheel movement
    pub wheel: i32,
    ///< mouse is moving, but cursor is not (used for scrolling)
    pub fix_at: bool,

    /* 2D wheel scrolling for moving around the map */
    pub wheel_moved: bool,
    pub v_wheel: f32,
    pub h_wheel: f32,

    /* Mouse appearance */
    ///< Sprites comprising cursor.
    pub sprites: Vec<CursorSprite>,
    ///< union of sprite properties
    pub total_offs: Point,
    ///< union of sprite properties
    pub total_size: Point,

    ///< position and size bounding-box for drawing
    pub draw_pos: Point,
    /// < position and size bounding-box for drawing
    pub draw_size: Point,

    ///< in case of animated cursor, list of frames
    pub animate_list: &'a [AnimCursor],
    ///< in case of animated cursor, current frame
    // Note: should start at the end
    pub animate_cur: usize,
    ///< in case of animated cursor, number of ticks to show the current cursor
    pub animate_timeout: u32,

    ///< cursor is visible
    pub visible: bool,
    ///< the rect occupied by the mouse is dirty (redraw)
    pub dirty: bool,
    ///< mouse inside this window, determines drawing logic
    pub in_window: bool,

    /* Drag data */
    ///< vehicle chain is dragged
    pub vehchain: bool,
}

impl<'a> CursorVars<'a> {
    pub fn update_cursor_position_relative(&mut self, delta_x: i32, delta_y: i32) {
        assert!(self.fix_at);
        self.delta.x = delta_x;
        self.delta.y = delta_y;
    }

    pub fn update_cursor_position(&mut self, x: i32, y: i32) -> bool {
        self.delta.x = x - self.pos.x;
        self.delta.y = y - self.pos.y;
        if self.fix_at {
            return self.delta.x != 0 || self.delta.y != 0;
        } else if self.pos.x != x || self.pos.y != y {
            self.dirty = true;
            self.pos.x = x;
            self.pos.y = y;
        }
        false
    }
}

/** Data about how and where to blit pixels. */
pub struct DrawPixelInfo {
    pub dst_ptr: *mut core::ffi::c_void,
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
    pub pitch: i32,
    pub zoom: ZoomLevel,
}

#[derive(Copy, Clone)]
pub union Colour {
    ///< Conversion of the channel information to a 32-bit number.
    pub data: u32,
    pub rgba: ColourRGBA,
    pub argb: ColourARGB,
    pub bgra: ColourBGRA,
    #[cfg(target_family = "wasm")]
    pub channels: ColourRGBA,
    #[cfg(target_endian = "little")]
    pub channels: ColourBGRA,
    #[cfg(target_endian = "big")]
    pub channels: ColourARGB,
}

/** Packed colour union to access the alpha, red, green, and blue channels from a 32 bit number for Emscripten build. */
#[derive(Copy, Clone)]
#[repr(C)]
pub union ColourRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/** Packed colour union to access the alpha, red, green, and blue channels from a 32 bit number for big-endian systems. */
#[derive(Copy, Clone)]
#[repr(C)]
pub union ColourARGB {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/** Packed colour union to access the alpha, red, green, and blue channels from a 32 bit number for little-endian systems. */
#[derive(Copy, Clone)]
#[repr(C)]
pub union ColourBGRA {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

/** Available font sizes */
#[derive(EnumBitset, Clone)]
#[bitset(name = FontSizes)]
#[repr(u8)]
pub enum FontSize {
    ///< Index of the normal font in the font tables.
    Normal,
    ///< Index of the small font in the font tables.
    Small,
    ///< Index of the large font in the font tables.
    Large,
    ///< Index of the monospaced font in the font tables.
    Monospace,
}

impl FontSize {
    pub fn name(&self) -> &'static str {
        match self {
            FontSize::Normal => "medium",
            FontSize::Small => "small",
            FontSize::Large => "large",
            FontSize::Monospace => "mono",
        }
    }
}

impl FontSizes {
    /** Mask of font sizes required to be present. */
    pub const REQUIRED: FontSizes =
        FontSizes::from_array([FontSize::Normal, FontSize::Small, FontSize::Large]);
}

/**
 * Used to only draw a part of the sprite.
 * Draw the subsprite in the rect (sprite_x_offset + left, sprite_y_offset + top) to (sprite_x_offset + right, sprite_y_offset + bottom).
 * Both corners are included in the drawing area.
 */
pub struct Subsprite {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

/** One of 16 base colours used for companies and windows/widgets. */
pub enum Colours {
    DarkBlue,
    PaleGreen,
    Pink,
    Yellow,
    Red,
    LightBlue,
    Green,
    DarkGreen,
    Blue,
    Cream,
    Mauve,
    Purple,
    Orange,
    Brown,
    Grey,
    White,
}
//TODO: DECLARE_INCREMENT_DECREMENT_OPERATORS(Colours)

/** Colour for pixel/line drawing. */
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PixelColour(pub u8);

/** Colour of the strings, see _string_colourmap in table/string_colours.h or docs/ottd-colourtext-palette.png */
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum TextColour {
    Blue,
    Silver,
    Gold,
    Red,
    Purple,
    LightBrown,
    Orange,
    Green,
    Yellow,
    DarkGreen,
    Cream,
    Brown,
    White,
    LightBlue,
    Grey,
    DarkBlue,
    Black,
    Invalid = 0xFF,
}

impl TextColour {
    ///< Marker for telling to use the colour from the string.
    pub const FROM_STRING: TextColour = TextColour::Blue;
}

#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = ExtendedTextColourFlags)]
pub enum ExtendedTextColourFlag {
    ///< Colour value is already a real palette colour index, not an index of a StringColour.
    IsPaletteColour,
    ///< Do not add shading to this text colour.
    NoShade,
    ///< Ignore colour changes from strings.
    Forced,
}

/** Container for the text colour and some text colour related flags for drawing. */
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ExtendedTextColour {
    pub colour: TextColour,
    pub flags: ExtendedTextColourFlags,
}

impl ExtendedTextColour {
    pub fn from_pixel(pc: PixelColour) -> Self {
        ExtendedTextColour {
            colour: unsafe { transmute(pc) },
            flags: ExtendedTextColourFlags::empty(),
        }
    }

    /// Decode the network encoded text colour.
    /// @param tc The network encoded colour.
    pub fn from_network(tc: u16) -> Self {
        unsafe { _NetworkExtendedTextColour { raw: tc }.value }
    }

    /// Encode this text colour for sending over the network.
    pub fn to_network(self) -> u16 {
        unsafe { _NetworkExtendedTextColour { value: self }.raw }
    }
}

union _NetworkExtendedTextColour {
    pub raw: u16,
    pub value: ExtendedTextColour,
}

/* A few values that are related to animations using palette changes */
///< number of animated colours
pub const PALETTE_ANIM_SIZE: u8 = 28;
///< Index in  the _palettes array from which all animations are taking places (table/palettes.h)
pub const PALETTE_ANIM_START: u8 = 227;

/** Define the operation GfxFillRect performs */
pub enum FillRectMode {
    ///< Fill rectangle with a single colour
    Opaque,
    ///< Draw only every second pixel, used for greying-out
    Checker,
    ///< Apply a recolour sprite to the screen content
    Recolour,
}

/** Palettes OpenTTD supports. */
pub enum PaletteType {
    ///< Use the DOS palette.
    DOS,
    ///< Use the Windows palette.
    Windows,
}

/** Types of sprites that might be loaded */
#[repr(u8)]
pub enum SpriteType {
    ///< The most basic (normal) sprite
    Normal = 0,
    ///< Special sprite for the map generator
    MapGen = 1,
    ///< A sprite used for fonts
    Font = 2,
    ///< Recolour sprite
    Recolour = 3,
    ///< Pseudosprite or other unusable sprite, used only internally
    Invalid = 4,
}

/**
 * The number of milliseconds per game tick.
 * The value 27 together with a day length of 74 ticks makes one day 1998 milliseconds, almost exactly 2 seconds.
 * With a 2 second day, one standard month is 1 minute, and one standard year is slightly over 12 minutes.
 */
pub const MILLISECONDS_PER_TICK: u32 = 27;

/** Information about the currently used palette. */
pub struct Palette {
	///< Current palette. Entry 0 has to be always fully transparent!
	pub palette: [Colour; 256],
	///< The first dirty element.
	pub first_dirty: u32,
	///< The number of dirty elements.
	pub count_dirty: u32,
}

/** Modes for 8bpp support */
pub enum Support8bpp {
	///< No support for 8bpp by OS or hardware, force 32bpp blitters.
	None,
	///< No 8bpp support by hardware, do not try to use 8bpp video modes or hardware palettes.
	System,
	///< Full 8bpp support by OS and hardware.
	Hardware
}

/** How to align the to-be drawn text. */
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StringAlignment(u8);

/** How to align the to-be drawn text. */
impl StringAlignment {
	///< Left align the text.
	pub const LEFT: StringAlignment = StringAlignment(0x00);
	///< Horizontally center the text.
	pub const HOR_CENTER: StringAlignment = StringAlignment(0x01);
	///< Right align the text (must be a single bit)
	pub const RIGHT: StringAlignment = StringAlignment(0x02);
	///< Mask for horizontal alignment.
	pub const HOR_MASK: StringAlignment = StringAlignment(0x03);

	///< Top align the text.
	pub const TOP: StringAlignment = StringAlignment(0x00);
	///< Vertically center the text.
	pub const VERT_CENTER: StringAlignment = StringAlignment(0x04);
	///< Bottom align the text. (must be a single bit)
	pub const BOTTOM: StringAlignment = StringAlignment(0x08);
	///< Mask for vertical alignment.
	pub const VERT_MASK: StringAlignment = StringAlignment(0x0C);

	///< Center both horizontally and vertically.
	pub const CENTER: StringAlignment = StringAlignment(0x0F);

	///< Force the alignment, i.e. don't swap for RTL languages.
	pub const FORCE: StringAlignment = StringAlignment(0x10);
}

#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = DirectionKeys)]
pub enum DirectionKey {
	Left, Up, Right, Down
}
