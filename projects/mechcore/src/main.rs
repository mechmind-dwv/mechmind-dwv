use std::time::SystemTime;

fn main() {
    let start_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    println!("⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢");
    println!("🤖 *SISTEMA MECHCORE ACTIVADO*");
    println!("🌌 Tiempo UNIX: {}", start_time);
    println!("🦀 Rust Version: {}", env!("CARGO_PKG_VERSION"));
    println!("🚀 Modo: {}", if cfg!(feature = "turbo") { "TURBO" } else { "ESTÁNDAR" });
    println!("⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢⬢");
}
