///< Tile size in world coordinates.
pub const TILE_SIZE: u32 = 16;
///< For masking in/out the inner-tile world coordinate
pub const TILE_UNIT_MASK: u32 = TILE_SIZE - 1;
///< Pixel distance between tile columns/rows in #ZOOM_BASE
pub const TILE_PIXELS: u32 = 32;
///< Height of a height level in world coordinate AND in pixels in #ZOOM_BASE
pub const TILE_HEIGHT: u32 = 8;

///< Maximum height of a building in pixels in #ZOOM_BASE. (Also applies to "bridge buildings" on the bridge floor.)
pub const MAX_BUILDING_PIXELS: u32 = 200;
///< Maximum width of a vehicle in pixels in #ZOOM_BASE.
pub const MAX_VEHICLE_PIXEL_X: u32 = 192;
///< Maximum height of a vehicle in pixels in #ZOOM_BASE
pub const MAX_VEHICLE_PIXEL_Y: u32 = 96;

///< Maximum allowed tile height
pub const MAX_TILE_HEIGHT: u32 = 255;

///< Lowest possible peak value for heightmap creation
pub const MIN_HEIGHTMAP_HEIGHT: u32 = 1;
///< Lowest possible peak value for world generation
pub const MIN_CUSTOM_TERRAIN_TYPE: u32 = 1;

///< Lower bound of maximum allowed heightlevel (in the construction settings)
pub const MIN_MAP_HEIGHT_LIMIT: u32 = 15;
///< Upper bound of maximum allowed heightlevel (in the construction settings)
pub const MAX_MAP_HEIGHT_LIMIT: u32 = MAX_TILE_HEIGHT;

///< Minimum snowline height
pub const MIN_SNOWLINE_HEIGHT: u32 = 2;
///< Default snowline height
pub const DEF_SNOWLINE_HEIGHT: u32 = 2;
///< Maximum allowed snowline height
pub const MAX_SNOWLINE_HEIGHT: u32 = (MAX_TILE_HEIGHT - 2);

///< Default snow coverage.
pub const DEF_SNOW_COVERAGE: u32 = 40;
///< Default desert coverage.
pub const DEF_DESERT_COVERAGE: u32 = 50;

///< How many bits in map array are dedicated for type of each tile.
pub const TILE_TYPE_BITS: u32 = 4;

/**
 * The different types of tiles.
 *
 * Each tile belongs to one type, according whatever is build on it.
 *
 * @note A railway with a crossing street is marked as TileType::Road.
 */
pub enum TileType {
    ///< A tile without any structures, i.e. grass, rocks, farm fields etc.
    Clear,
    ///< A tile with railway.
    Railway,
    ///< A tile with road and/or tram tracks.
    Road,
    ///< A house by a town.
    House,
    ///< Tile with one or more trees.
    Trees,
    ///< A tile of a station or airport.
    Station,
    ///< Water tile.
    Water,
    ///< Invisible tiles at the SW and SE border.
    Void,
    ///< Part of an industry.
    Industry,
    ///< Tunnel entry/exit and bridge heads.
    TunnelBridge,
    ///< Contains objects such as transmitters and owned land.
    Object,
}

pub type TileIndex = u32;

pub struct Tile {}

pub enum TileBridge {
    None,
    /// North-East
    AxisX,
    /// South-West
    AxisY,
}

pub enum TileZone {
    Normal,
    Desert,
    Rainforest,
}
