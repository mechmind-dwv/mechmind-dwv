mod models;

use std::time::SystemTime;
use crate::models::system::{MechSystem, SystemStatus};

fn main() {
    let start_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut system = MechSystem::new();
    
    // Simular activación
    if cfg!(feature = "turbo") {
        system.status = SystemStatus::Turbo;
    } else {
        system.status = SystemStatus::Active;
    }

    println!("⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢");
    println!("🤖 *SISTEMA MECHCORE ACTIVADO*");
    println!("🌌 Tiempo UNIX: {}", start_time);
    println!("🦀 Proyectos: {:?}", system.projects);
    println!("🔋 Energía: {}%", system.energy_level);
    println!("🚀 Modo: {:?}", system.status);
    println!("⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢");
}
