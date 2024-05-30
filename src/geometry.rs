use crate::material::Material;
use crate::piece_materials::PieceMaterials;

/// Parameters for representing the geometry of a simulation piece.
pub struct Geometry {
    /// X-direction distance where core becomes reflector.
    pub a: f64,
    /// Y-direction distance where cores transition.
    pub b: f64,
    /// X-direction total distance.
    pub l_x: f64,
    /// Y-direction total distance.
    pub l_y: f64,
    /// Material coefficients of each part of the piece.
    pub materials: PieceMaterials,
}

impl Geometry {
    /// Given x and y coordinates, return the material properties of that location.
    pub fn get_material(&self, x: f64, y: f64) -> &Material {
        if x > self.a {
            &self.materials.reflector
        } else if y > self.b {
            &self.materials.core
        } else {
            &self.materials.regular
        }
    }
}
