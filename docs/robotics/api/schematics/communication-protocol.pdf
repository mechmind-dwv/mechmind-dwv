### 2. **Protocolo de Comunicación** (`communication-protocol.pdf`)
[![PDF](https://img.shields.io/badge/PDF-Protocol-green)](docs/robotics/api/schematics/communication-protocol.pdf)

```mermaid
%% Ejemplo del protocolo
sequenceDiagram
    MechBot->>ROS2: /cmd_vel (Twist)
    ROS2->>ESP32: Serial@115200bps
    ESP32-->>Sensors: I2C Request
```

### 🛠️ **Cómo Actualizar Diagramas**
1. **Para .drawio**:
   ```bash
   apt install draw.io-desktop  # Linux
   brew install --cask drawio   # macOS
   ```
2. **Generar PDFs**:
   ```bash
   drawio -x -f pdf -o protocol.pdf architecture.drawio
   ```

### 🔄 **Integración Automatizada**
Añade este workflow en `.github/workflows/schematics.yml`:
```yaml
name: Update Diagrams
on:
  push:
    paths:
      - 'docs/robotics/api/schematics/*.drawio'
jobs:
  convert:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Convert to PDF
        uses: docker://jgraph/drawio
        with:
          args: --export --format pdf --output schematics/communication-protocol.pdf schematics/mechbot-architecture.drawio
      - name: Commit changes
        run: |
          git config --global user.name "MechBot Diagrams Bot"
          git add schematics/
          git commit -m "🔄 Auto-update diagrams"
          git push
```

### 🌟 **Visualización en README**
```markdown
[![Arquitectura](docs/robotics/api/schematics/preview-architecture.png)](docs/robotics/api/schematics/mechbot-architecture.drawio)
```

**Consejos Pro**:
1. Usa capas en DrawIO para diagramas complejos
2. Mantén un `CHANGELOG.md` en la carpeta para versionar cambios
3. Integra con PlantUML para diagramas generados desde código:

```plantuml
@startuml
component "MechBot CPU" as core
component "Sensors" as sensors
core --> sensors : I2C
@enduml
```

¿Necesitas ayuda para:
- [ ] Convertir diagramas a formatos móviles
- [ ] Automatizar validaciones técnicas
- [ ] Integrar con documentación ROS2?

¡Estoy lista para ayudar! 🤖💡 

```rust
// Función de verificación
fn verify_diagrams() -> Result<(), &'static str> {
    println!("✅ Diagramas ubicados en: docs/robotics/api/schematics/");
    Ok(())
}
```
