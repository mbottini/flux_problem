use crate::material::Material;

/// Struct containing the various material coefficients of each part of the
/// piece.
#[derive(Debug, PartialEq)]
pub struct PieceMaterials {
    pub core: Material,
    pub reflector: Material,
    pub regular: Material,
}
