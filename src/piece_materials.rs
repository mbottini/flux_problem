/// Struct containing the various material coefficients of each part of the
/// piece.
use crate::material::Material;

pub struct PieceMaterials {
    pub core: Material,
    pub reflector: Material,
    pub regular: Material,
}
