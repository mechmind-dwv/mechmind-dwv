#!/bin/bash
# DESPLIEGUE INTERESTELAR (v2.0)

echo "🚀 Iniciando sistema MechMind..."
cd ~/MechMind-dwv 

# Verificar estructura crítica
[ -f "projects/mechcore/Cargo.toml" ] || { echo "❌ Error: Falta Cargo.toml"; exit 1; }
[ -d "projects/mechcore/src" ] || { echo "❌ Error: Falta directorio src"; exit 1; }

# Ejecutar núcleo principal
echo "🦾 Iniciando MechCore..."
cd projects/mechcore
cargo run --release
