#!/bin/bash
echo "🚀 Iniciando sistema MechMind..."
cd "$(dirname "$0")/.." || exit 1
[ -f "projects/mechcore/Cargo.toml" ] || { echo "❌ Falta Cargo.toml"; exit 1; }
[ -d "projects/mechcore/src" ] || { echo "❌ Falta directorio src"; exit 1; }
echo "🦾 Iniciando MechCore..."
cd projects/mechcore && cargo run --release
