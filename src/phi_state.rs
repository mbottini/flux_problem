use crate::geometry::Geometry;

/// The instantaneous state of a piece at a particular point in time.
/// XResolution and YResolution store the number of X and Y coordinates
/// in the phi linspace.
struct PhiState<const XResolution: usize, const YResolution: usize> {
    geometry: Geometry,
    phi: [[f64; XResolution]; YResolution],
}
