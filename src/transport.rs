/** Type for the company global vehicle unit number. */
pub type UnitId = u16;

/** Available types of transport */
pub enum TransportType {
    /* These constants are for now linked to the representation of bridges
     * and tunnels, so they can be used by GetTileTrackStatus_TunnelBridge.
     * In an ideal world, these constants would be used everywhere when
     * accessing tunnels and bridges. For now, you should just not change
     * the values for road and rail.
     */
    ///< Transport by train
    Rail,
    ///< Transport by road vehicle
    Road,
    ///< Transport over water
    Water,
    ///< Transport through air
    Air,
    ///< Sentinel for invalid transport types.
    Invalid = 0xFF,
}

impl TransportType {
    pub const ALL: [TransportType; 4] = [
        TransportType::Rail,
        TransportType::Road,
        TransportType::Water,
        TransportType::Air,
    ];
}
