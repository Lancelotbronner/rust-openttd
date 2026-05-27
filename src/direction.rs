use enum_bitset::EnumBitset;

/**
 * Defines the 8 directions on the map.
 *
 * This enum defines 8 possible directions which are used for
 * the vehicles in the game. The directions are aligned straight
 * to the viewport, not to the map. So north points to the top of
 * your viewport and not rotated by 45 degrees left or right to get
 * a "north" used in you games.
 */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = Directions)]
pub enum Direction {
    /// North
    N ,
    /// Northeast
    NE ,
    /// East
    E ,
    /// Southeast
    SE ,
    /// South
    S ,
    /// Southwest
    SW ,
    /// West
    W ,
    /// Northwest
    NW ,
}
// DECLARE_INCREMENT_DECREMENT_OPERATORS(Direction)

/**
 * Enumeration for the difference between two directions.
 *
 * This enumeration is used to mark differences between
 * two directions. If you get one direction you can align
 * a second direction in 8 different ways. This enumeration
 * only contains 6 of these 8 differences, but the remaining
 * two can be calculated by adding to differences together.
 * This also means you can add two differences together and
 * get the difference you really want to get. The difference
 * of 45 degrees left + the difference of 45 degrees right results in the
 * difference of 0 degrees.
 *
 * @note To get this mentioned addition of direction you must use
 *       modulo DIR_END or use the #ChangeDirDiff(DirDiff, DirDiff) function.
 * @see ChangeDirDiff(DirDiff, DirDiff)
 */
pub enum DirDiff {
    /// Both directions faces to the same direction
    Same = 0,
    /// Angle of 45 degrees right
    Right45 = 1,
    /// Angle of 90 degrees right
    Right90 = 2,
    /// One direction is the opposite of the other one
    Reverse = 4,
    /// Angle of 90 degrees left
    Left90 = 6,
    /// Angle of 45 degrees left
    Left45 = 7,
}

/**
 * Enumeration for diagonal directions.
 *
 * This enumeration is used for the 4 direction of the tile-edges.
 */
#[derive(EnumBitset, Copy, Clone)]
#[bitset(name = DiagDirections)]
pub enum DiagDirection {
    /// Northeast, upper right on your monitor
    NE,
    /// Southeast
    SE,
    /// Southwest
    SW,
    /// Northwest
    NW,
}
// DECLARE_INCREMENT_DECREMENT_OPERATORS(DiagDirection)
// DECLARE_ENUM_AS_ADDABLE(DiagDirection)

/**
 * Enumeration for the difference between to DiagDirection.
 *
 * As the DiagDirection only contains 4 possible directions the
 * difference between two of these directions can only be in 4 ways.
 * As the DirDiff enumeration the values can be added together and
 * you will get the resulting difference (use modulo DIAGDIR_END).
 *
 * @see DirDiff
 */
pub enum DiagDirDiff {
    /// Same directions
    Same = 0,
    /// 90 degrees right
    Right = 1,
    /// Reverse directions
    Reverse = 2,
    /// 90 degrees left
    Left = 3,
}
// DECLARE_INCREMENT_DECREMENT_OPERATORS(DiagDirDiff)

/**
 * Enumeration for the two axis X and Y
 *
 * This enumeration represents the two axis X and Y in the game.
 * The X axis is the one which goes align the north-west edge
 * (and south-east edge). The Y axis must be so the one which goes
 * align the north-east edge (and south-west) edge.
 */
pub enum Axis {
    /// The X axis
    X = 0,
    /// The y axis
    Y = 1,
	///< Flag for an invalid Axis
	Invalid = 0xFF,
}
// DECLARE_ENUM_AS_ADDABLE(Axis)
