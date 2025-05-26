// Sistema central con documentación
#[derive(Debug)]
pub struct MechSystem {
    projects: Vec<&'static str>,
    energy_level: u32,
}

impl MechSystem {
    /// Crea nueva instancia con valores por defecto
    pub fn new() -> Self {
        MechSystem {
            projects: vec!["MechBot-3X", "AI-Core"],
            energy_level: 100,
        }
    }
}
