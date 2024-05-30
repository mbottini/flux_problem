use crate::material::Material;
use crate::piece_materials::PieceMaterials;

/// Geometry of the piece being simulated.
/// a: x-direction distance where core becomes reflector
/// b: y-direction distance where cores transition
/// l_x: x-direction total distance
/// l_y: y-direction total distance
/// materials: Material coefficients of each part of the piece
pub struct Geometry {
    a: f64,
    b: f64,
    l_x: f64,
    l_y: f64,
    materials: PieceMaterials,
}

impl Geometry {
    /// Given x and y coordinates, return the material properties of that location.
    fn get_material(&self, x: f64, y: f64) -> &Material {
        if x > self.a {
            &self.materials.reflector
        } else if y > self.b {
            &self.materials.core
        } else {
            &self.materials.regular
        }
    }
}
