use crate::geometry::Geometry;
use std::f64::consts::PI;

/// The instantaneous state of a piece at a particular point in time.
/// X_RES and Y_RES store the number of X and Y coordinates
/// in the phi linspace.
#[derive(Debug, PartialEq)]
pub struct PhiState<const X_RES: usize, const Y_RES: usize> {
    geometry: Geometry,
    phi: [[f64; X_RES]; Y_RES],
}

/// Creates initial phi value, given the geometry and x/y
/// coordinates.
fn initial_phi_value(g: &Geometry, x: usize, y: usize, x_res: usize, y_res: usize) -> f64 {
    let x_coord: f64 = g.l_x / x_res as f64 * x as f64;
    let y_coord: f64 = g.l_y / y_res as f64 * y as f64;
    (PI / 2.0 * x_coord).cos() * (PI / 2.0 * y_coord).cos()
}

impl<const X_RES: usize, const Y_RES: usize> PhiState<X_RES, Y_RES> {
    /// Creates new PhiState. This starts with an undulating initial state that varies according
    /// to cos(x) * cos(y).
    pub fn new(g: Geometry) -> Self {
        let mut res = PhiState {
            geometry: g,
            phi: [[0.0; X_RES]; Y_RES],
        };
        for y in 0..Y_RES {
            for x in 0..X_RES {
                res.phi[y][x] = initial_phi_value(&res.geometry, x, y, X_RES, Y_RES)
            }
        }
        res
    }
}

impl<const X_RES: usize, const Y_RES: usize> std::fmt::Display for PhiState<X_RES, Y_RES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Geometry: {:?}\n", self.geometry)?;
        for arr in self.phi {
            write!(f, "{arr:?}")?;
        }
        Ok(())
    }
}
