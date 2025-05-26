## 🏗 Estructura del Proyecto Rust

## 🔍 Stack Principal
<div align="center">
  <img src="https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExcDl6d3V0b2N0Z3V4bHl5dWZ4Z2J6eGJmN2RycG5xZ2NqYzB1eWZ1ZyZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/3oKIPnAiaMCws8nOsE/giphy.gif" width="250">

  ![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
  ![ROS2](https://img.shields.io/badge/ROS2-22314E?style=for-the-badge&logo=ros&logoColor=white)
  ![Embedded](https://img.shields.io/badge/Embedded-00599C?style=for-the-badge&logo=embeddedc&logoColor=white)
  ![Crates.io](https://img.shields.io/crates/v/serde?style=for-the-badge&logo=rust)
</div>

## 🏗 Estructura del Proyecto Rust
```
proyecto_rust/
├── Cargo.toml
├── src/
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Crate principal
│   ├── robotics/        # Módulo de robótica
│   │   ├── mod.rs       # Interfaces públicas
│   │   ├── control.rs   # Lógica de control
│   │   └── sensors.rs   # Manejo de sensores
│   └── ai/              # Módulo de IA
│       ├── mod.rs
│       └── neural.rs    # Redes neuronales
├── tests/               # Pruebas integración
├── docs/                # Documentación
└── .github/workflows/   # CI/CD
    └── rust.yml
```
