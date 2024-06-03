use crate::material::Material;
use crate::piece_materials::PieceMaterials;

/// Parameters for representing the geometry of a simulation piece.
#[derive(Debug, PartialEq)]
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
            &self.materials.core2
        }
    }
}

mod test {
    #[cfg(test)]
    use super::*;

    #[cfg(test)]
    const REFLECTOR_MAT: Material = Material {
        diffusion_coefficient: 0.65,
        sigma_a: 0.12,
        sigma_f: 0.185,
    };
    #[cfg(test)]
    const CORE2_MAT: Material = Material {
        diffusion_coefficient: 0.12,
        sigma_a: 0.10,
        sigma_f: 0.01,
    };
    #[cfg(test)]
    const CORE_MAT: Material = Material {
        diffusion_coefficient: 0.185,
        sigma_a: 0.15,
        sigma_f: 0.0,
    };

    #[cfg(test)]
    const TEST_GEOMETRY: Geometry = Geometry {
        a: 50.0,
        b: 60.0,
        l_x: 100.0,
        l_y: 100.0,
        materials: PieceMaterials {
            reflector: REFLECTOR_MAT,
            core: CORE_MAT,
            core2: CORE2_MAT,
        },
    };

    #[test]
    fn test_get_material() {
        assert_eq!(TEST_GEOMETRY.get_material(0.0, 0.0), &CORE2_MAT);
        assert_eq!(TEST_GEOMETRY.get_material(51.0, 0.0), &REFLECTOR_MAT);
        assert_eq!(TEST_GEOMETRY.get_material(0.0, 61.0), &CORE_MAT);
    }
}
