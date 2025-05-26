```markdown
# 🤖 **Estructura de docs/robotics/** (MechBot-2X)

```bash
docs/
└── robotics/
    ├── schematics/               # Diseños técnicos
    │   ├── mechbot-v2.1.sch      # KiCAD Schematic
    │   └── pcb-layout.pdf        # Placa PCB
    ├── firmware/                 # Código embebido
    │   ├── src/
    │   │   └── main.rs           # Firmware en Rust
    │   └── platformio.ini        # Configuración
    ├── simulations/              # Entornos virtuales
    │   ├── gazebo/               # ROS/Gazebo
    │   └── webots/               # Webots
    ├── api/                     # Documentación API
    │   └── rustdoc/              # `cargo doc --open`
    └── DATASHEET.md              # Especificaciones técnicas
```

---
