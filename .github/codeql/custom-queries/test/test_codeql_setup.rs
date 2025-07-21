// ¡Verificación en Rust!
#[test]
fn test_codeql_setup() {
    assert!(std::path::exists(".github/codeql"));
    println!("✅ Todo listo para escanear");
}
