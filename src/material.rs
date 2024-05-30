/// Material coefficients, representing some physics gobbledegook about
/// the heat transfer properties of some material, I guess. Don't look at
/// me, I just work here.
#[derive(Debug)]
pub struct Material {
    pub diffusion_coefficient: f64,
    pub sigma_a: f64,
    pub sigma_f: f64,
}
