/**
 * Short-hand to apply GUI zoom level.
 * @param value Pixel amount at #ZoomLevel::Min (full zoom in).
 * @return Pixel amount at current interface size.
 */
pub fn UnScaleGUI(value: i32) -> i32 {
    UnScaleByZoom(value, _gui_zoom)
}

/**
 * Scale zoom level relative to GUI zoom.
 * @param value zoom level to scale
 * @return scaled zoom level
 */
inline ZoomLevel ScaleZoomGUI(ZoomLevel value)
{
return std::clamp(value + (_gui_zoom - ZoomLevel::Normal), ZoomLevel::Min, ZoomLevel::Max);
}

/**
 * UnScale zoom level relative to GUI zoom.
 * @param value zoom level to scale
 * @return un-scaled zoom level
 */
inline ZoomLevel UnScaleZoomGUI(ZoomLevel value)
{
return std::clamp(value - (_gui_zoom - ZoomLevel::Normal), ZoomLevel::Min, ZoomLevel::Max);
}

/**
 * Scale traditional pixel dimensions to GUI zoom level, for drawing sprites.
 * @param value Pixel amount at #ZOOM_BASE (traditional "normal" interface size).
 * @return Pixel amount at current interface size.
 */
inline int ScaleSpriteTrad(int value)
{
return UnScaleGUI(value * ZOOM_BASE);
}

/**
 * Scale traditional pixel dimensions to GUI zoom level.
 * @param value Pixel amount at #ZOOM_BASE (traditional "normal" interface size).
 * @return Pixel amount at current interface size.
 */
inline int ScaleGUITrad(int value)
{
return value * _gui_scale / 100;
}
