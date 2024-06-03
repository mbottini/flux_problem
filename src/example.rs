/// Example usage of the library.
use flux_problem::{
    geometry::Geometry, material::Material, phi_state::PhiState, piece_materials::PieceMaterials,
};

fn main() {
    const REFLECTOR_MAT: Material = Material {
        diffusion_coefficient: 0.65,
        sigma_a: 0.12,
        sigma_f: 0.185,
    };
    const CORE2_MAT: Material = Material {
        diffusion_coefficient: 0.12,
        sigma_a: 0.10,
        sigma_f: 0.01,
    };
    const CORE_MAT: Material = Material {
        diffusion_coefficient: 0.185,
        sigma_a: 0.15,
        sigma_f: 0.0,
    };
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

    println!("{}", PhiState::<20, 20>::new(TEST_GEOMETRY));
}
