#!/bin/bash

echo "🔄 Verificando estado del despliegue..."
STATUS=$(gh api /repos/mechmind-dwv/mechmind-dwv/pages | jq -r .status)

if [ "$STATUS" = "built" ]; then
  echo "✅ Despliegue exitoso!"
  echo "🌐 URL: https://mechmind-dwv.github.io"
elif [ "$STATUS" = "null" ]; then
  echo "⚠️ No se encontró configuración de GitHub Pages"
  echo "Configura Pages en: https://github.com/mechmind-dwv/mechmind-dwv/settings/pages"
else
  echo "🚧 Estado del despliegue: $STATUS"
  echo "⏳ Monitoreando ejecución activa..."
  
  # Obtener ID de la última ejecución del workflow 'gh-pages.yml'
  RUN_ID=$(gh run list --workflow=gh-pages.yml --json databaseId -q '.[0].databaseId')
  
  if [ -n "$RUN_ID" ]; then
    gh run watch $RUN_ID
  else
    echo "❌ No se encontraron ejecuciones activas"
  fi
fi
