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
