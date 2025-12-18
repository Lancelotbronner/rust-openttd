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
    DIR_N = 0,
    /// Northeast
    DIR_NE = 1,
    /// East
    DIR_E = 2,
    /// Southeast
    DIR_SE = 3,
    /// South
    DIR_S = 4,
    /// Southwest
    DIR_SW = 5,
    /// West
    DIR_W = 6,
    /// Northwest
    DIR_NW = 7,
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
    DIRDIFF_SAME = 0,
    /// Angle of 45 degrees right
    DIRDIFF_45RIGHT = 1,
    /// Angle of 90 degrees right
    DIRDIFF_90RIGHT = 2,
    /// One direction is the opposite of the other one
    DIRDIFF_REVERSE = 4,
    /// Angle of 90 degrees left
    DIRDIFF_90LEFT = 6,
    /// Angle of 45 degrees left
    DIRDIFF_45LEFT = 7,
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
    DIAGDIR_NE = 0,
    /// Southeast
    DIAGDIR_SE = 1,
    /// Southwest
    DIAGDIR_SW = 2,
    /// Northwest
    DIAGDIR_NW = 3,
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
    DIAGDIRDIFF_SAME = 0,
    /// 90 degrees right
    DIAGDIRDIFF_90RIGHT = 1,
    /// Reverse directions
    DIAGDIRDIFF_REVERSE = 2,
    /// 90 degrees left
    DIAGDIRDIFF_90LEFT = 3,
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
    AXIS_X = 0,
    /// The y axis
    AXIS_Y = 1,
}
// DECLARE_ENUM_AS_ADDABLE(Axis)
