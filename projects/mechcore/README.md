# 🤖 MechCore

Este es el núcleo del sistema **MechMind**. Proporciona las funcionalidades base y la gestión de estados para los robots de la serie Mech.

## 🚀 Inicio Rápido

Para compilar el núcleo:

```bash
cargo build -p mechcore
```

Para ejecutar el sistema en modo estándar:

```bash
cargo run -p mechcore
```

Para activar el **Modo Turbo**:

```bash
cargo run -p mechcore --features turbo
```

## 📂 Estructura

- `src/main.rs`: Punto de entrada del sistema.
- `Cargo.toml`: Definición de dependencias y features.
