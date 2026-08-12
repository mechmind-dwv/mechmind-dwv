//! # MechSystem Model
//! Lógica central para la gestión del estado del robot.

#[derive(Debug, Clone)]
pub struct MechSystem {
    pub projects: Vec<String>,
    pub energy_level: u32,
    pub status: SystemStatus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SystemStatus {
    Idle,
    Active,
    Turbo,
    LowBattery,
}

impl MechSystem {
    /// Crea una nueva instancia con valores por defecto.
    pub fn new() -> Self {
        MechSystem {
            projects: vec![
                "MechBot-2X".to_string(),
                "MechCore".to_string(),
                "MechROS2".to_string(),
            ],
            energy_level: 100,
            status: SystemStatus::Idle,
        }
    }

    /// Actualiza el estado del sistema basado en el nivel de energía.
    pub fn update_status(&mut self) {
        if self.energy_level < 20 {
            self.status = SystemStatus::LowBattery;
        }
    }
}

impl Default for MechSystem {
    fn default() -> Self {
        Self::new()
    }
}
