### 🛠️ **Script de Despliegue Unificado** (``)
#!/bin/bash
# Deploy MechMind System v2.0

set -eo pipefail

# 1. Validar entorno
check_environment() {
  [ -f "STRUCTURE.md" ] || { echo "❌ Error: No STRUCTURE.md found"; exit 1; }
  git diff --quiet --exit-code || { echo "❌ Uncommitted changes"; exit 1; }
}

# 2. Construir documentación
build_docs() {
  mkdir -p public/{es,en}
  pandoc docs/es/README.md -o public/es/index.html --template=docs/assets/template.html
  pandoc docs/en/README.md -o public/en/index.html --template=docs/assets/template.html
}

# 3. Sincronizar con GitHub Pages
deploy() {
  gh workflow run docs.yml -f force-rebuild=true
  gh run watch $(gh run list -w docs.yml -L 1 --json databaseId -q '.[0].databaseId')
}

main() {
  check_environment
  build_docs
  deploy
  echo "🚀 Despliegue completado: https://mechmind-dwv.github.io"
}

main "$@"
