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
